//! Configurable artificial-latency model for the inter-node UDP relays.
//!
//! A [`DelayDist`] draws a per-datagram delay; a [`LatencyConfig`] maps a directed
//! link `src → dst` to the distribution that should shape it. Resolution order is
//! `per_link` → `per_node` (keyed by destination) → `default`, so a global delay can
//! be set once and selectively overridden for individual nodes or links.

use std::{collections::HashMap, time::Duration};

use rand::Rng;

/// A delay distribution sampled once per forwarded datagram.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DelayDist {
    /// Constant delay.
    Fixed(Duration),
    /// Uniformly distributed delay in `[min, max]`.
    Uniform { min: Duration, max: Duration },
    /// Normally distributed delay, clamped to be non-negative.
    Normal { mean: Duration, stddev: Duration },
}

impl DelayDist {
    /// Draw a delay. Never returns a negative duration.
    pub fn sample(&self) -> Duration {
        match *self {
            DelayDist::Fixed(d) => d,
            DelayDist::Uniform { min, max } => {
                let lo = min.as_secs_f64();
                let hi = max.as_secs_f64();
                if hi <= lo {
                    return min;
                }
                Duration::from_secs_f64(rand::rng().random_range(lo..hi))
            }
            DelayDist::Normal { mean, stddev } => {
                let sample = mean.as_secs_f64() + stddev.as_secs_f64() * standard_normal();
                Duration::from_secs_f64(sample.max(0.0))
            }
        }
    }
}

/// Standard-normal sample via the Box–Muller transform (avoids a `rand_distr` dep).
fn standard_normal() -> f64 {
    let mut rng = rand::rng();
    let u1: f64 = rng.random::<f64>().max(f64::MIN_POSITIVE);
    let u2: f64 = rng.random::<f64>();
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Per-link latency configuration with global / per-node / per-link granularity.
#[derive(Clone, Debug, Default)]
pub struct LatencyConfig {
    /// Applied to any link without a more specific match.
    pub default: Option<DelayDist>,
    /// Keyed by destination node; overrides `default`.
    pub per_node: HashMap<usize, DelayDist>,
    /// Keyed by `(src, dst)` node pair; overrides `per_node`.
    pub per_link: HashMap<(usize, usize), DelayDist>,
}

impl LatencyConfig {
    /// A config carrying only a single global delay.
    pub fn global(dist: DelayDist) -> Self {
        Self {
            default: Some(dist),
            ..Default::default()
        }
    }

    /// Resolve the distribution shaping the directed link `src → dst`.
    pub fn resolve(&self, src: usize, dst: usize) -> Option<DelayDist> {
        self.per_link
            .get(&(src, dst))
            .or_else(|| self.per_node.get(&dst))
            .copied()
            .or(self.default)
    }

    /// True when no delay would ever be applied.
    pub fn is_empty(&self) -> bool {
        self.default.is_none() && self.per_node.is_empty() && self.per_link.is_empty()
    }

    /// Load a per-node / per-link config from YAML (see `docs/localcluster/README.md`).
    pub fn from_yaml(s: &str) -> Result<Self, String> {
        let raw: RawLatencyConfig =
            serde_saphyr::from_str(s).map_err(|e| format!("invalid latency config: {e}"))?;

        let default = raw.default.as_deref().map(parse_delay).transpose()?;
        let per_node = raw
            .per_node
            .iter()
            .map(|(node, spec)| Ok((*node, parse_delay(spec)?)))
            .collect::<Result<_, String>>()?;
        let per_link = raw
            .per_link
            .iter()
            .map(|link| Ok(((link.from, link.to), parse_delay(&link.delay)?)))
            .collect::<Result<_, String>>()?;

        Ok(Self {
            default,
            per_node,
            per_link,
        })
    }
}

#[derive(serde::Deserialize)]
struct RawLatencyConfig {
    #[serde(default)]
    default: Option<String>,
    #[serde(default)]
    per_node: HashMap<usize, String>,
    #[serde(default)]
    per_link: Vec<RawLink>,
}

#[derive(serde::Deserialize)]
struct RawLink {
    from: usize,
    to: usize,
    delay: String,
}

/// Parse a delay spec into a [`DelayDist`].
///
/// Accepted forms:
/// - `100ms`                  → fixed
/// - `100ms±30ms` / `100ms+-30ms` → uniform `[mean-jitter, mean+jitter]` (clamped ≥0)
/// - `uniform:50ms,150ms`     → uniform `[min, max]`
/// - `normal:100ms,30ms`      → normal `{mean, stddev}`
///
/// Durations accept `us`/`µs`, `ms`, `s` suffixes (default `ms` if unit-less).
pub fn parse_delay(s: &str) -> Result<DelayDist, String> {
    let s = s.trim();

    if let Some(rest) = s.strip_prefix("uniform:") {
        let (min, max) = split_pair(rest)?;
        return Ok(DelayDist::Uniform {
            min: parse_duration(min)?,
            max: parse_duration(max)?,
        });
    }
    if let Some(rest) = s.strip_prefix("normal:") {
        let (mean, stddev) = split_pair(rest)?;
        return Ok(DelayDist::Normal {
            mean: parse_duration(mean)?,
            stddev: parse_duration(stddev)?,
        });
    }

    let separator = if s.contains('±') {
        Some("±")
    } else if s.contains("+-") {
        Some("+-")
    } else {
        None
    };

    if let Some(sep) = separator {
        let (mean_s, jitter_s) = s.split_once(sep).expect("separator present");
        let mean = parse_duration(mean_s)?;
        let jitter = parse_duration(jitter_s)?;
        let min = mean.saturating_sub(jitter);
        let max = mean.saturating_add(jitter);
        return Ok(DelayDist::Uniform { min, max });
    }

    Ok(DelayDist::Fixed(parse_duration(s)?))
}

fn split_pair(s: &str) -> Result<(&str, &str), String> {
    s.split_once(',')
        .map(|(a, b)| (a.trim(), b.trim()))
        .ok_or_else(|| format!("expected two comma-separated durations, got '{s}'"))
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    let (value, unit) = if let Some(v) = s.strip_suffix("ms") {
        (v, "ms")
    } else if let Some(v) = s.strip_suffix("us").or_else(|| s.strip_suffix("µs")) {
        (v, "us")
    } else if let Some(v) = s.strip_suffix('s') {
        (v, "s")
    } else {
        (s, "ms")
    };

    let value: f64 = value
        .trim()
        .parse()
        .map_err(|_| format!("'{s}' is not a valid duration"))?;
    if value < 0.0 || !value.is_finite() {
        return Err(format!("duration must be a non-negative number, got '{s}'"));
    }

    let secs = match unit {
        "s" => value,
        "ms" => value / 1_000.0,
        "us" => value / 1_000_000.0,
        _ => unreachable!(),
    };
    Ok(Duration::from_secs_f64(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_fixed_with_units() {
        assert_eq!(
            parse_delay("100ms").unwrap(),
            DelayDist::Fixed(Duration::from_millis(100))
        );
        assert_eq!(
            parse_delay("2s").unwrap(),
            DelayDist::Fixed(Duration::from_secs(2))
        );
        assert_eq!(
            parse_delay("500us").unwrap(),
            DelayDist::Fixed(Duration::from_micros(500))
        );
        // Unit-less defaults to milliseconds.
        assert_eq!(
            parse_delay("250").unwrap(),
            DelayDist::Fixed(Duration::from_millis(250))
        );
    }

    #[test]
    fn parses_jitter_as_uniform() {
        assert_eq!(
            parse_delay("100ms±30ms").unwrap(),
            DelayDist::Uniform {
                min: Duration::from_millis(70),
                max: Duration::from_millis(130)
            }
        );
        // Jitter larger than mean clamps the lower bound at zero.
        assert_eq!(
            parse_delay("10ms+-30ms").unwrap(),
            DelayDist::Uniform {
                min: Duration::ZERO,
                max: Duration::from_millis(40)
            }
        );
    }

    #[test]
    fn parses_explicit_distributions() {
        assert_eq!(
            parse_delay("uniform:50ms,150ms").unwrap(),
            DelayDist::Uniform {
                min: Duration::from_millis(50),
                max: Duration::from_millis(150)
            }
        );
        assert_eq!(
            parse_delay("normal:100ms,30ms").unwrap(),
            DelayDist::Normal {
                mean: Duration::from_millis(100),
                stddev: Duration::from_millis(30)
            }
        );
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_delay("abc").is_err());
        assert!(parse_delay("uniform:10ms").is_err());
    }

    #[test]
    fn fixed_sample_is_exact() {
        let d = DelayDist::Fixed(Duration::from_millis(42));
        assert_eq!(d.sample(), Duration::from_millis(42));
    }

    #[test]
    fn uniform_sample_within_bounds() {
        let d = DelayDist::Uniform {
            min: Duration::from_millis(50),
            max: Duration::from_millis(150),
        };
        for _ in 0..1000 {
            let s = d.sample();
            assert!(s >= Duration::from_millis(50) && s <= Duration::from_millis(150));
        }
    }

    #[test]
    fn normal_sample_never_negative() {
        let d = DelayDist::Normal {
            mean: Duration::from_millis(5),
            stddev: Duration::from_millis(50),
        };
        for _ in 0..1000 {
            assert!(d.sample() >= Duration::ZERO);
        }
    }

    #[test]
    fn resolve_precedence_link_over_node_over_default() {
        let mut cfg = LatencyConfig::global(DelayDist::Fixed(Duration::from_millis(10)));
        cfg.per_node
            .insert(2, DelayDist::Fixed(Duration::from_millis(20)));
        cfg.per_link
            .insert((0, 2), DelayDist::Fixed(Duration::from_millis(30)));

        // Link-specific wins.
        assert_eq!(
            cfg.resolve(0, 2),
            Some(DelayDist::Fixed(Duration::from_millis(30)))
        );
        // Falls back to per-node (destination 2) for other sources.
        assert_eq!(
            cfg.resolve(1, 2),
            Some(DelayDist::Fixed(Duration::from_millis(20)))
        );
        // Falls back to default for unrelated links.
        assert_eq!(
            cfg.resolve(0, 1),
            Some(DelayDist::Fixed(Duration::from_millis(10)))
        );
    }

    #[test]
    fn loads_yaml_config() {
        let yaml = r#"
default: "100ms±30ms"
per_node:
  2: "200ms"
per_link:
  - { from: 0, to: 1, delay: "500ms" }
"#;
        let cfg = LatencyConfig::from_yaml(yaml).unwrap();
        assert_eq!(
            cfg.default,
            Some(DelayDist::Uniform {
                min: Duration::from_millis(70),
                max: Duration::from_millis(130)
            })
        );
        assert_eq!(
            cfg.resolve(9, 2),
            Some(DelayDist::Fixed(Duration::from_millis(200)))
        );
        assert_eq!(
            cfg.resolve(0, 1),
            Some(DelayDist::Fixed(Duration::from_millis(500)))
        );
        assert!(!cfg.is_empty());
    }
}
