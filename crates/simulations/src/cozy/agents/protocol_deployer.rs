use std::{borrow::Cow, collections::HashMap, sync::Arc};

use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::build_call_contract_txenv,
};

use super::errors::CozyAgentError;
use crate::cozy::{
    bindings_wrapper::*,
    types::CozyProtocolDeployParams,
    utils::{build_deploy_contract_tx, build_unlinked_deploy_contract_tx, Counter},
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress,
};

pub struct ProtocolDeployer {
    pub name: Option<Cow<'static, str>>,
    pub address: Address,
    deploy_params: CozyProtocolDeployParams,
    weth: Arc<CozyProtocolContract>,
}

impl ProtocolDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        deploy_params: CozyProtocolDeployParams,
        weth: &Arc<CozyProtocolContract>,
    ) -> Self {
        Self {
            name,
            address,
            deploy_params,
            weth: weth.clone(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for ProtocolDeployer {
    fn id(&self) -> AgentId {
        AgentId {
            name: self.name.clone(),
            address: self.address,
        }
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        let mut nonce_counter = Counter::new(0);
        // Deploy external libraries.
        log::info!("{:?} deploying protocol libraries.", self.name);
        let libraries = self
            .deploy_libraries(state, &channel, &mut nonce_counter)
            .expect("Error deploying libraries.");
        // Deploy core protocol.
        log::info!("{:?} deploying core protocol.", self.name);
        self.deploy_protocol(state, &channel, &libraries.clone(), &mut nonce_counter)
            .expect("Error deploying protocol.");
    }
}

impl ProtocolDeployer {
    fn deploy_libraries(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: &AgentChannel<CozyUpdate>,
        nonce_counter: &mut Counter,
    ) -> Result<HashMap<Address, &BindingsWrapper>> {
        let mut libraries: HashMap<Address, &BindingsWrapper> = HashMap::new();

        let configurator_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let delay_lib_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let demandside_lib_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let redemption_lib_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let state_transitions_lib_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let supply_side_lib_addr = Address::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));

        libraries.insert(configurator_addr, &CONFIGURATORLIB);
        libraries.insert(delay_lib_addr, &DELAYLIB);
        libraries.insert(demandside_lib_addr, &DEMANDSIDELIB);
        libraries.insert(redemption_lib_addr, &REDEMPTIONLIB);
        libraries.insert(state_transitions_lib_addr, &STATETRANSITIONSLIB);
        libraries.insert(supply_side_lib_addr, &SUPPLYSIDELIB);

        let (configurator_lib_tx, _) =
            build_deploy_contract_tx(self.address, &CONFIGURATORLIB, ())?;
        let (delay_lib_tx, _) = build_deploy_contract_tx(self.address, &DELAYLIB, ())?;
        let (demand_side_lib_tx, _) = build_deploy_contract_tx(self.address, &DEMANDSIDELIB, ())?;
        let (redemption_lib_tx, _) = build_deploy_contract_tx(self.address, &REDEMPTIONLIB, ())?;
        let (state_transitions_lib_tx, _) =
            build_deploy_contract_tx(self.address, &STATETRANSITIONSLIB, ())?;
        let (supply_side_lib_tx, _) = build_deploy_contract_tx(self.address, &SUPPLYSIDELIB, ())?;

        for tx in vec![
            configurator_lib_tx,
            delay_lib_tx,
            demand_side_lib_tx,
            redemption_lib_tx,
            state_transitions_lib_tx,
            supply_side_lib_tx,
        ] {
            channel.send(SimUpdate::Evm(tx));
        }

        Ok(libraries)
    }

    fn deploy_protocol(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: &AgentChannel<CozyUpdate>,
        libraries: &HashMap<Address, &BindingsWrapper>,
        nonce_counter: &mut Counter,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let manager_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let set_logic_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        // next nonce is initialization of the Set logic.
        nonce_counter.increment();
        let set_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let ptoken_logic_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        // next nonce is initialization of the PToken logic.
        nonce_counter.increment();
        let ptoken_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let backstop_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let cozyrouter_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));

        let mut evm_updates: Vec<SimUpdate<CozyUpdate>> = vec![];
        let mut world_updates: Vec<SimUpdate<CozyUpdate>> = vec![];

        // Deploy manager.
        let owner: EthersAddress = self.address.into();
        let pauser: EthersAddress = self.address.into();
        let manager_args = (
            backstop_addr,
            set_factory_addr,
            owner,
            pauser,
            self.deploy_params.delays.clone(),
            self.deploy_params.fees.clone(),
            self.deploy_params.allowed_markets_per_set,
        );
        let (manager_tx, manager_contract) =
            build_deploy_contract_tx(self.address, &MANAGER, manager_args)?;
        evm_updates.push(SimUpdate::Evm(manager_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(MANAGER.name.into(), manager_addr.into(), manager_contract),
        )));

        // Deploy set logic.
        let set_logic_args = (manager_addr, ptoken_factory_addr, backstop_addr);
        let (set_logic_tx, set_logic_contract) =
            build_unlinked_deploy_contract_tx(self.address, &SET, &libraries, set_logic_args)?;

        // Initialize set logic.
        let weth_addr = self.weth.address.into();
        let empty_market_configs: Vec<MarketConfig> = vec![];
        let set_initialize_args: (
            EthersAddress,
            EthersAddress,
            EthersAddress,
            SetConfig,
            Vec<MarketConfig>,
        ) = (
            EthersAddress::zero(),
            EthersAddress::zero(),
            weth_addr,
            SetConfig {
                deposit_fee: 0,
                leverage_factor: 0,
            },
            empty_market_configs,
        );
        let set_initialize_call_data = set_logic_contract
            .encode_function("initialize", set_initialize_args)
            .expect("Issue encoding func.");
        let set_initialize_tx = build_call_contract_txenv(
            self.address,
            set_logic_addr.into(),
            set_initialize_call_data,
            None,
            None,
        );

        evm_updates.push(SimUpdate::Evm(set_logic_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(SET.name.into(), set_logic_addr.into(), set_logic_contract),
        )));
        evm_updates.push(SimUpdate::Evm(set_initialize_tx));

        // Deploy set factory.
        let set_factory_args = (manager_addr, set_logic_addr);
        let (set_factory_tx, set_factory_contract) =
            build_deploy_contract_tx(self.address, &SETFACTORY, set_factory_args)?;
        evm_updates.push(SimUpdate::Evm(set_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                SETFACTORY.name.into(),
                set_factory_addr.into(),
                set_factory_contract,
            ),
        )));

        // Deploy ptoken logic.
        let ptoken_logic_args = (manager_addr,);
        let (ptoken_logic_tx, ptoken_logic_contract) =
            build_deploy_contract_tx(self.address, &PTOKEN, ptoken_logic_args)?;

        // Initialize ptoken logic.
        let ptoken_initialize_args = (EthersAddress::zero(), EthersAddress::zero(), 0_u8);
        let ptoken_initialize_call_data = ptoken_logic_contract
            .encode_function("initialize", ptoken_initialize_args)
            .expect("Issue encoding func.");
        let ptoken_initialize_tx = build_call_contract_txenv(
            self.address,
            ptoken_logic_addr.into(),
            ptoken_initialize_call_data,
            None,
            None,
        );

        evm_updates.push(SimUpdate::Evm(ptoken_logic_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                PTOKEN.name.into(),
                ptoken_logic_addr.into(),
                ptoken_logic_contract,
            ),
        )));
        evm_updates.push(SimUpdate::Evm(ptoken_initialize_tx));

        // Deploy ptoken factory.
        let ptoken_factory_args = (ptoken_logic_addr,);
        let (ptoken_factory_tx, ptoken_factory_contract) =
            build_deploy_contract_tx(self.address, &PTOKENFACTORY, ptoken_factory_args)?;
        evm_updates.push(SimUpdate::Evm(ptoken_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                PTOKENFACTORY.name.into(),
                ptoken_factory_addr.into(),
                ptoken_factory_contract,
            ),
        )));

        // Deploy backstop.
        let backstop_args = (manager_addr, weth_addr);
        let (backstop_tx, backstop_contract) =
            build_deploy_contract_tx(self.address, &BACKSTOP, backstop_args)?;
        evm_updates.push(SimUpdate::Evm(backstop_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                BACKSTOP.name.into(),
                backstop_addr.into(),
                backstop_contract,
            ),
        )));

        // Deploy CozyRouter.
        let cozyrouter_args = (manager_addr, weth_addr, weth_addr, weth_addr);
        let (cozyrouter_tx, cozyrouter_contract) =
            build_deploy_contract_tx(self.address, &COZYROUTER, cozyrouter_args)?;
        evm_updates.push(SimUpdate::Evm(cozyrouter_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                COZYROUTER.name.into(),
                cozyrouter_addr.into(),
                cozyrouter_contract,
            ),
        )));

        let jump_rate_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let dynamic_level_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let drip_decay_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let uma_trigger_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));
        let chainlink_trigger_factory_addr = EthersAddress::from(create_address(
            self.address.into(),
            nonce_counter.get_and_increment_count(),
        ));

        // Deploy cost model jump rate factory.
        let (jump_rate_factory_tx, jump_rate_factory_contract) =
            build_deploy_contract_tx(self.address, &COSTMODELJUMPRATEFACTORY, ())?;
        evm_updates.push(SimUpdate::Evm(jump_rate_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                COSTMODELJUMPRATEFACTORY.name.into(),
                jump_rate_factory_addr.into(),
                jump_rate_factory_contract,
            ),
        )));

        // Deploy cost model dynamic level factory.
        let (dynamic_level_factory_tx, dynamic_level_factory_contract) =
            build_deploy_contract_tx(self.address, &COSTMODELDYNAMICLEVELFACTORY, ())?;
        evm_updates.push(SimUpdate::Evm(dynamic_level_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                COSTMODELDYNAMICLEVELFACTORY.name.into(),
                dynamic_level_factory_addr.into(),
                dynamic_level_factory_contract,
            ),
        )));

        // Deploy drip decay model factory.
        let (drip_decay_factory_tx, drip_decay_factory_contract) =
            build_deploy_contract_tx(self.address, &DRIPDECAYMODELCONSTANTFACTORY, ())?;
        evm_updates.push(SimUpdate::Evm(drip_decay_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                DRIPDECAYMODELCONSTANTFACTORY.name.into(),
                drip_decay_factory_addr.into(),
                drip_decay_factory_contract,
            ),
        )));

        // Deploy uma trigger factory.
        let uma_trigger_factory_args = (manager_addr, manager_addr);
        let (uma_trigger_factory_tx, uma_trigger_factory_contract) =
            build_deploy_contract_tx(self.address, &UMATRIGGERFACTORY, uma_trigger_factory_args)?;
        evm_updates.push(SimUpdate::Evm(uma_trigger_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                UMATRIGGERFACTORY.name.into(),
                uma_trigger_factory_addr.into(),
                uma_trigger_factory_contract,
            ),
        )));

        // Deploy Chainlink trigger factory.
        let chainlink_trigger_factory_args = (manager_addr,);
        let (chainlink_trigger_factory_tx, chainlink_trigger_factory_contract) =
            build_deploy_contract_tx(
                self.address,
                &CHAINLINKTRIGGERFACTORY,
                chainlink_trigger_factory_args,
            )?;
        evm_updates.push(SimUpdate::Evm(chainlink_trigger_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(
                CHAINLINKTRIGGERFACTORY.name.into(),
                chainlink_trigger_factory_addr.into(),
                chainlink_trigger_factory_contract,
            ),
        )));

        // Send updates.
        for evm_update in evm_updates {
            channel.send(evm_update);
        }
        for world_update in world_updates {
            channel.send(world_update);
        }

        Ok(())
    }
}
