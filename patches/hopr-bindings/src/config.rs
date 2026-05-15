use std::{collections::BTreeMap, str::FromStr};

use alloy::{
    contract::Result as ContractResult,
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::MULTICALL3_ADDRESS,
    rpc::types::TransactionRequest,
    sol_types::{SolCall, SolValue},
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use tracing::debug;

use crate::{
    constants::*,
    hopr_announcements::HoprAnnouncements::{self, HoprAnnouncementsInstance},
    hopr_announcements_proxy::HoprAnnouncementsProxy,
    hopr_channels::HoprChannels::{self, HoprChannelsInstance},
    hopr_node_management_module::HoprNodeManagementModule::{self, HoprNodeManagementModuleInstance},
    hopr_node_safe_migration::HoprNodeSafeMigration::{self, HoprNodeSafeMigrationInstance},
    hopr_node_safe_registry::HoprNodeSafeRegistry::{self, HoprNodeSafeRegistryInstance},
    hopr_node_stake_factory::HoprNodeStakeFactory::{self, HoprNodeStakeFactoryInstance},
    hopr_ticket_price_oracle::HoprTicketPriceOracle::{self, HoprTicketPriceOracleInstance},
    hopr_token::HoprToken::{self, HoprTokenInstance},
    hopr_winning_probability_oracle::HoprWinningProbabilityOracle::{self, HoprWinningProbabilityOracleInstance},
};
pub const CONTRACTS_ADDRESSES_FILE_CONTENT: &str = include_str!(concat!(env!("OUT_DIR"), "/contracts-addresses.json"));

/// Holds addresses of all smart contracts.
#[serde_as]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContractAddresses {
    /// Announcements contract
    #[serde_as(as = "DisplayFromStr")]
    pub announcements: Address,
    /// Channels contract
    #[serde_as(as = "DisplayFromStr")]
    pub channels: Address,
    /// Node management module contract (can be zero if safe is not used)
    #[serde_as(as = "DisplayFromStr")]
    pub module_implementation: Address,
    /// Migration helper for node safes and modules
    #[serde_as(as = "DisplayFromStr")]
    pub node_safe_migration: Address,
    /// Safe registry contract
    #[serde_as(as = "DisplayFromStr")]
    pub node_safe_registry: Address,
    /// Stake factory contract
    #[serde_as(as = "DisplayFromStr")]
    pub node_stake_factory: Address,
    /// Price oracle contract
    #[serde_as(as = "DisplayFromStr")]
    pub ticket_price_oracle: Address,
    /// Token contract
    #[serde_as(as = "DisplayFromStr")]
    pub token: Address,
    /// Minimum ticket winning probability contract
    #[serde_as(as = "DisplayFromStr")]
    pub winning_probability_oracle: Address,
    /// XHOPR token contract
    #[serde(default)]
    #[serde_as(as = "DisplayFromStr")]
    pub xhopr_token: Address,
}

impl IntoIterator for &ContractAddresses {
    type IntoIter = std::vec::IntoIter<Address>;
    type Item = Address;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.token,
            self.channels,
            self.announcements,
            self.node_safe_registry,
            self.node_safe_migration,
            self.ticket_price_oracle,
            self.winning_probability_oracle,
            self.node_stake_factory,
            self.module_implementation,
            self.xhopr_token,
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SingleNetworkContractAddresses {
    pub chain_id: u64,
    pub indexer_start_block_number: u32,
    pub addresses: ContractAddresses,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworksWithContractAddresses {
    pub networks: BTreeMap<String, SingleNetworkContractAddresses>,
}

impl Default for NetworksWithContractAddresses {
    fn default() -> Self {
        Self::from_str(CONTRACTS_ADDRESSES_FILE_CONTENT)
            .expect("bundled public contracts addresses should be always convertible")
    }
}

impl FromStr for NetworksWithContractAddresses {
    type Err = serde_json::Error;

    fn from_str(data: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str::<NetworksWithContractAddresses>(data)
    }
}

/// Holds instances to contracts.
/// The contract instances do not include xHOPR token,
/// as it is not used by the node and is only included in the addresses for completeness.
#[derive(Debug, Clone)]
pub struct ContractInstances<P> {
    pub token: HoprTokenInstance<P>,
    pub channels: HoprChannelsInstance<P>,
    pub announcements: HoprAnnouncementsInstance<P>,
    pub safe_registry: HoprNodeSafeRegistryInstance<P>,
    pub price_oracle: HoprTicketPriceOracleInstance<P>,
    pub win_prob_oracle: HoprWinningProbabilityOracleInstance<P>,
    pub stake_factory: HoprNodeStakeFactoryInstance<P>,
    pub module_implementation: HoprNodeManagementModuleInstance<P>,
    pub node_safe_migration: HoprNodeSafeMigrationInstance<P>,
}

impl<P> ContractInstances<P>
where
    P: alloy::providers::Provider + Clone,
{
    pub fn new(contract_addresses: &ContractAddresses, provider: P) -> Self {
        Self {
            token: HoprTokenInstance::new(contract_addresses.token, provider.clone()),
            channels: HoprChannelsInstance::new(contract_addresses.channels, provider.clone()),
            announcements: HoprAnnouncementsInstance::new(contract_addresses.announcements, provider.clone()),
            safe_registry: HoprNodeSafeRegistryInstance::new(contract_addresses.node_safe_registry, provider.clone()),
            price_oracle: HoprTicketPriceOracleInstance::new(contract_addresses.ticket_price_oracle, provider.clone()),
            win_prob_oracle: HoprWinningProbabilityOracleInstance::new(
                contract_addresses.winning_probability_oracle,
                provider.clone(),
            ),
            stake_factory: HoprNodeStakeFactoryInstance::new(contract_addresses.node_stake_factory, provider.clone()),
            module_implementation: HoprNodeManagementModuleInstance::new(
                contract_addresses.module_implementation,
                provider.clone(),
            ),
            node_safe_migration: HoprNodeSafeMigrationInstance::new(
                contract_addresses.node_safe_migration,
                provider.clone(),
            ),
        }
    }

    pub async fn deploy_erc1820_registry(provider: P) -> ContractResult<()> {
        debug!("deploying ERC1820 registry...");
        // Fund 1820 deployer and deploy ERC1820Registry
        let tx = TransactionRequest::default()
            .with_to(ERC_1820_DEPLOYER)
            .with_value(ETH_VALUE_FOR_ERC1820_DEPLOYER);

        // Sequentially executing the following transactions:
        // 1. Fund the deployer wallet
        provider.send_transaction(tx.clone()).await?.watch().await?;
        // 2. Use the funded deployer wallet to deploy ERC1820Registry with a signed txn
        provider
            .send_raw_transaction(&ERC_1820_REGISTRY_DEPLOY_CODE)
            .await?
            .watch()
            .await?;

        Ok(())
    }

    pub async fn deploy_multicall3(provider: P) -> ContractResult<()> {
        debug!("deploying Multicall3...");
        // Fund Multicall3 deployer and deploy Multicall3
        let multicall3_code = provider.get_code_at(MULTICALL3_ADDRESS).await?;
        if multicall3_code.is_empty() {
            // Fund Multicall3 deployer and deploy Multicall3
            let tx = TransactionRequest::default()
                .with_to(crate::constants::MULTICALL3_DEPLOYER)
                .with_value(crate::constants::ETH_VALUE_FOR_MULTICALL3_DEPLOYER);
            // Sequentially executing the following transactions:
            // 1. Fund the deployer wallet
            provider.send_transaction(tx.clone()).await?.watch().await?;
            // 2. Use the funded deployer wallet to deploy Multicall3 with a signed txn
            provider
                .send_raw_transaction(MULTICALL3_DEPLOY_CODE)
                .await?
                .watch()
                .await?;
        }
        Ok(())
    }

    pub async fn deploy_safe_suites(provider: P) -> ContractResult<()> {
        debug!("deploying Safe contracts...");

        // Check if safe suite has been deployed. If so, skip this step
        let code = provider.get_code_at(SAFE_SINGLETON_ADDRESS).await?;

        // only deploy contracts when needed
        if code.is_empty() {
            debug!("deploying safe code");
            // Deploy Safe diamond deployment proxy singleton
            let safe_diamond_proxy_address = {
                // Fund the Safe deployer with 0.01 anvil-eth and deploy the Safe diamond deployment proxy singleton
                let tx = TransactionRequest::default()
                    .with_to(SAFE_DEPLOYER_ADDRESS)
                    .with_value(SAFE_DEPLOYER_BALANCE);

                provider.send_transaction(tx).await?.watch().await?;

                let tx = provider
                    .send_raw_transaction(&SAFE_DIAMOND_PROXY_SINGLETON_DEPLOY_CODE)
                    .await?
                    .get_receipt()
                    .await?;
                tx.contract_address.unwrap()
            };
            debug!("Safe diamond proxy singleton {:?}", safe_diamond_proxy_address);

            // Deploy minimum Safe suite
            // 1. Safe proxy factory deploySafeProxyFactory();
            let _tx_safe_proxy_factory = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_PROXY_FACTORY_DEPLOY_CODE);
            // 2. Handler: only CompatibilityFallbackHandler and omit TokenCallbackHandler as it's not used now
            // 2. Handler: deploy Safe ExtensibleFallbackHandler, v1.5.0
            let _tx_safe_compatibility_fallback_handler = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_COMPATIBILITY_FALLBACK_HANDLER_DEPLOY_CODE_V150);
            // 3. Library: only MultiSendCallOnly and omit MultiSendCall
            let _tx_safe_multisend_call_only = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_MULTISEND_CALL_ONLY_DEPLOY_CODE);
            // 4. Safe singleton v1.4.1 deploySafe();
            let _tx_safe_singleton_v141 = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_SINGLETON_DEPLOY_CODE_V141);
            // 5. Safe L2 singleton v1.4.1 deploySafe();
            let _tx_safe_l2_singleton_v141 = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_SINGLETON_L2_DEPLOY_CODE_V141);
            // 6. Safe multisend:
            let _tx_safe_multisend = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_MULTISEND_DEPLOY_CODE);
            // 7. Safe L2 singleton v1.5.0 deploySafe();
            let _tx_safe_l2_singleton_v150 = TransactionRequest::default()
                .with_to(safe_diamond_proxy_address)
                .with_input(SAFE_SINGLETON_L2_DEPLOY_CODE_V150);
            // other omitted libs: SimulateTxAccessor, CreateCall, and SignMessageLib
            // broadcast those transactions
            provider.send_transaction(_tx_safe_proxy_factory).await?.watch().await?;
            provider
                .send_transaction(_tx_safe_compatibility_fallback_handler)
                .await?
                .watch()
                .await?;
            provider
                .send_transaction(_tx_safe_multisend_call_only)
                .await?
                .watch()
                .await?;
            provider
                .send_transaction(_tx_safe_singleton_v141)
                .await?
                .watch()
                .await?;
            provider
                .send_transaction(_tx_safe_l2_singleton_v141)
                .await?
                .watch()
                .await?;
            provider.send_transaction(_tx_safe_multisend).await?.watch().await?;
            provider
                .send_transaction(_tx_safe_l2_singleton_v150)
                .await?
                .watch()
                .await?;
        }

        let code_safe_singleton_v141 = provider.get_code_at(SAFE_SINGLETON_L2_ADDRESS_V141).await?;
        let code_safe_singleton_v150 = provider.get_code_at(SAFE_SINGLETON_L2_ADDRESS_V150).await?;
        let code_compatibility_handler_v150 = provider
            .get_code_at(SAFE_COMPATIBILITY_FALLBACK_HANDLER_ADRESS_V150)
            .await?;
        assert!(
            !code_safe_singleton_v141.is_empty(),
            "Safe singleton v1.4.1 not deployed"
        );
        assert!(
            !code_safe_singleton_v150.is_empty(),
            "Safe singleton v1.5.0 not deployed"
        );
        assert!(
            !code_compatibility_handler_v150.is_empty(),
            "Safe compatibility handler v1.5.0 not deployed"
        );
        Ok(())
    }

    /// Deploys testing environment (with dummy network registry proxy) via the given provider.
    async fn inner_deploy_common_contracts_for_testing(provider: P, deployer_address: Address) -> ContractResult<Self> {
        // Pre-deploy common contracts
        ContractInstances::deploy_erc1820_registry(provider.clone()).await?;
        ContractInstances::deploy_multicall3(provider.clone()).await?;
        ContractInstances::deploy_safe_suites(provider.clone()).await?;

        debug!("deploying contracts...");

        let module_implementation = HoprNodeManagementModule::deploy(provider.clone()).await?;
        let safe_registry = HoprNodeSafeRegistry::deploy(provider.clone()).await?;
        let price_oracle = HoprTicketPriceOracle::deploy(
            provider.clone(),
            deployer_address,
            U256::from(100000000000000000_u128), // U256::from(100000000000000000_u128),
        )
        .await?;
        let win_prob_oracle = HoprWinningProbabilityOracle::deploy(
            provider.clone(),
            deployer_address,
            alloy::primitives::aliases::U56::from(0xFFFFFFFFFFFFFF_u64), /* 0xFFFFFFFFFFFFFF in hex or
                                                                          * 72057594037927935 in
                                                                          * decimal values */
        )
        .await?;
        let token = HoprToken::deploy(provider.clone()).await?;
        let channels = HoprChannels::deploy(
            provider.clone(),
            Address::from(token.address().as_ref()),
            1_u32,
            Address::from(safe_registry.address().as_ref()),
        )
        .await?;
        let announcements_implementation = HoprAnnouncements::deploy(provider.clone()).await?;
        let announcement_initialize_parameters = (
            *token.address(),
            *safe_registry.address(),
            INIT_KEY_BINDING_FEE,
            deployer_address,
        )
            .abi_encode();
        let encode_initialization = HoprAnnouncements::initializeCall {
            initParams: announcement_initialize_parameters.into(),
        }
        .abi_encode();

        let announcements_proxy = HoprAnnouncementsProxy::deploy(
            provider.clone(),
            Address::from(announcements_implementation.address().as_ref()),
            encode_initialization.into(),
        )
        .await?;

        let stake_factory = HoprNodeStakeFactory::deploy(
            provider.clone(),
            Address::from(module_implementation.address().as_ref()),
            Address::from(announcements_proxy.address().as_ref()),
            deployer_address,
        )
        .await?;

        let node_safe_migration = HoprNodeSafeMigration::deploy(
            provider.clone(),
            Address::from(module_implementation.address().as_ref()),
            Address::from(stake_factory.address().as_ref()),
        )
        .await?;

        // get the defaultHoprNetwork from the stake factory
        let default_hopr_network = stake_factory.defaultHoprNetwork().call().await?;
        let new_default_hopr_network = HoprNodeStakeFactory::HoprNetwork {
            tokenAddress: *token.address(),
            defaultTokenAllowance: default_hopr_network.defaultTokenAllowance,
            defaultAnnouncementTarget: default_hopr_network.defaultAnnouncementTarget,
        };
        // Update the `defaultHoprNetwork` in the factory contract, to update the token address
        stake_factory
            .updateHoprNetwork(new_default_hopr_network)
            .send()
            .await?
            .watch()
            .await?;

        Ok(Self {
            token,
            channels,
            announcements: HoprAnnouncementsInstance::new(*announcements_proxy.address(), provider.clone()),
            safe_registry,
            price_oracle,
            win_prob_oracle,
            stake_factory,
            module_implementation,
            node_safe_migration,
        })
    }

    /// Deploys testing environment (with dummy network registry proxy) via the given provider.
    pub async fn deploy_for_testing(provider: P, deployer_address: Address) -> ContractResult<Self> {
        let instances = Self::inner_deploy_common_contracts_for_testing(provider.clone(), deployer_address).await?;

        Ok(Self { ..instances })
    }

    pub fn get_contract_addresses(&self) -> ContractAddresses {
        ContractAddresses {
            token: *self.token.address(),
            channels: *self.channels.address(),
            announcements: *self.announcements.address(),
            node_safe_registry: *self.safe_registry.address(),
            ticket_price_oracle: *self.price_oracle.address(),
            winning_probability_oracle: *self.win_prob_oracle.address(),
            node_stake_factory: *self.stake_factory.address(),
            module_implementation: *self.module_implementation.address(),
            node_safe_migration: *self.node_safe_migration.address(),
            xhopr_token: Address::ZERO, /* xHOPR token is not used by the node and is only included in the addresses
                                         * for completeness, so we can set it to zero here */
        }
    }
}

impl<P> From<&ContractInstances<P>> for ContractAddresses
where
    P: alloy::providers::Provider + Clone,
{
    fn from(instances: &ContractInstances<P>) -> Self {
        Self {
            token: *instances.token.address(),
            channels: *instances.channels.address(),
            announcements: *instances.announcements.address(),
            node_safe_registry: *instances.safe_registry.address(),
            ticket_price_oracle: *instances.price_oracle.address(),
            winning_probability_oracle: *instances.win_prob_oracle.address(),
            node_safe_migration: *instances.node_safe_migration.address(),
            node_stake_factory: *instances.stake_factory.address(),
            module_implementation: *instances.module_implementation.address(),
            xhopr_token: Address::ZERO, /* xHOPR token is not used by the node and is only included in the addresses
                                         * for completeness, so we can set it to zero here */
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NetworksWithContractAddresses;

    #[test]
    fn networks_with_contract_addresses_are_default_constructible() {
        let contract_addresses: NetworksWithContractAddresses = Default::default();

        assert!(!contract_addresses.networks.is_empty());
    }
}
