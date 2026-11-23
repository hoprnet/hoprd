//! Periodic component state reporting, run by the node alongside its other tasks.
//!
//! The chain connector, channel graph and P2P transport each own their snapshot schema, timer,
//! record budget and assessment. Every interval each emits one INFO record with
//! `event = "component_state"`, tagged with the component name and this node's `instance`.
//!
//! `HOPR_STATE_DUMP_INTERVAL` sets the interval in seconds: 60 by default, `0` disables reporting,
//! and other values must lie within 5–86400. An invalid value stops reporting with an error but
//! leaves the node running.
//!
//! With JSON logging, extract the records with:
//!
//! ```sh
//! jq 'select(.fields.event == "component_state") | .fields.state | fromjson' node.jsonl
//! ```
use std::sync::Arc;

use hopr_lib::{
    api::{graph::NetworkGraphView as _, types::primitive::prelude::Address},
    state_dump::{StateDump, StateDumpFactory, StateDumpSections},
};
use hopr_network_graph::SharedChannelGraph;
use hopr_transport_p2p::HoprNetwork;

use crate::HoprBlokliConnector;

/// Runs the chain, graph and transport state reporters for one node.
pub(crate) struct ComponentStateReporter {
    instance: String,
}

impl ComponentStateReporter {
    /// `instance` identifies this node in every record.
    pub(crate) fn new(instance: String) -> Self {
        Self { instance }
    }
}

impl StateDumpFactory<Arc<HoprBlokliConnector>, SharedChannelGraph, HoprNetwork>
    for ComponentStateReporter
{
    fn create(
        self,
        chain: Arc<HoprBlokliConnector>,
        graph: SharedChannelGraph,
        network: HoprNetwork,
        safe: Address,
        // hoprd registers no extra sections.
        _sections: StateDumpSections,
    ) -> Option<StateDump> {
        let instance = self.instance;
        Some(StateDump::new(None, async move {
            let (chain_result, graph_result, transport_result) = futures::join!(
                chain.run_state_reporting_with_funding_policy(&instance, safe, || graph
                    .ticket_face_value()),
                // No path sampling: hoprd keeps no bounded path-sample source.
                graph.run_state_reporting(&instance, Vec::new),
                network.run_state_reporting(&instance),
            );
            for (component, result) in [
                ("chain", chain_result),
                ("graph", graph_result),
                ("transport", transport_result),
            ] {
                if let Err(error) = result {
                    tracing::error!(component, %error, "component state reporting stopped");
                }
            }
        }))
    }
}
