# CT remote-connectivity regression test

## Purpose

CT traffic must continue to select a valid path through a remote relayer even when
the graph has many much lower-latency alternatives. This models a CT node in
Central Europe and a relayer in New Zealand. It is deliberately not a VPN QoS
test: the requirement is graph coverage, not a low-latency route.

## Fixture

Build a graph with one CT source, one destination, 60 healthy low-latency relayer
paths, and one healthy remote relayer path. Give each local path a 20 ms RTT and
the remote path a 2 s RTT. All paths must have identical acknowledgement rate,
capacity, and channel state so latency is the only ranking difference.

The 2 s value is intentionally much worse than a real Europe--New Zealand RTT.
It makes a failure unambiguous and prevents a timing-sensitive test.

## Assertions

Run the fixture twice.

1. With `PathPlannerConfig::default()`, the default candidate cap (50) and the
   8-candidate latency prune deterministically exclude the remote candidate.
2. With the following hoprd configuration, the remote candidate must be included
   in the generated candidate collection:

   ```yaml
   hopr:
     network:
       path_planner:
         latency_halflife: "10s"
         max_cached_paths: 128
         min_paths_anonymity_floor: 0
         return_path_exploration: 0.25
   ```

The executable hoprd unit test constructs this graph and checks both halves: the
default selector excludes the remote candidate, while the configured selector
retains it. It also proves the YAML values reach the transport configuration.
A CT traffic test may add seeded return-path draws later; it must assert an exact
route rather than a probabilistic traffic fraction.

## Boundaries

This test does not claim that a remote node has the same QoS as a nearby node.
It only proves the CT policy continues to observe and reach it. A separate
network test should cover packet loss, jitter, and real transport timeouts.
