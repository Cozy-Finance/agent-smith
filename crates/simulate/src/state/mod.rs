use std::collections::HashMap;

use eyre::{Result, *};
use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB, RefDBWrapper},
    primitives::{AccountInfo, Address, ExecutionResult, TxEnv},
    Database, EVM,
};
use thiserror::Error;

use crate::{
    agent::agent_channel::{AgentSimUpdate, AgentUpdateResults},
    state::{
        update::{SimUpdate, SimUpdateResult, UpdateData},
        world::World,
    },
    time_policy::TimeEnv,
    utils::*,
    EvmAddress,
};

pub mod update;
pub mod world;

#[derive(Error, Debug)]
pub enum SimStateError {
    #[error("Evm db error")]
    EvmDbError,
}

#[derive(Clone)]
pub struct SimState<U: UpdateData> {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub world: Option<Box<dyn World<WorldUpdateData = U>>>,
    pub update_results: HashMap<EvmAddress, HashMap<u64, SimUpdateResult<U>>>,
}

impl<U: UpdateData> Default for SimState<U> {
    fn default() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        SimState {
            evm,
            world: None,
            update_results: HashMap::new(),
        }
    }
}

impl<U: UpdateData> SimState<U> {
    pub fn new(world: Option<Box<dyn World<WorldUpdateData = U>>>) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000000000); // This is a large contract size limit, beware!
        evm.database(db);
        SimState {
            evm,
            world,
            update_results: HashMap::new(),
        }
    }

    /// Update the time env.
    /// # Arguments
    /// * `time_env` - The time env.
    pub fn update_time_env(&mut self, time_env: TimeEnv) {
        self.evm.env.block.number = time_env.number;
        self.evm.env.block.timestamp = time_env.timestamp;
    }

    // Add an account to evm.
    pub fn insert_account_info(
        &mut self,
        address: Address,
        account_info: AccountInfo,
    ) -> Result<()> {
        self.evm
            .db()
            .ok_or(SimStateError::EvmDbError)?
            .insert_account_info(address, account_info);

        Ok(())
    }

    pub fn get_account_info(&self, address: Address) -> Result<AccountInfo> {
        let raw_db = self.evm.db.as_ref().ok_or(SimStateError::EvmDbError)?;
        let db = RefDBWrapper::new(&raw_db).db;
        Ok(db
            .basic(address)
            .map_err(|_| SimStateError::EvmDbError)?
            .ok_or(SimStateError::EvmDbError)?)
    }

    pub fn get_results(&self, address: &EvmAddress) -> AgentUpdateResults<U> {
        AgentUpdateResults::new(self.update_results.get(address))
    }

    pub fn clear_all_results(&mut self) {
        self.update_results.clear()
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_raw_evm_tx(&mut self, tx: &TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx.clone();
        match self.evm.transact_commit() {
            Ok(result) => result,
            Err(e) => panic!("Raw evm tx execution failed: {:?}.", e),
        }
    }

    /// Execute a transaction in the execution environment without writing to DB.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn simulate_raw_evm_tx(&mut self, tx: &TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx.clone();
        match self.evm.transact() {
            Ok(result_and_state) => result_and_state.result,
            Err(e) => panic!("Raw evm tx simulation failed: {:?}.", e),
        }
    }

    pub fn execute_raw_world_update(&mut self, update: &U) -> Option<U> {
        match self.world {
            Some(ref mut world) => world.execute(update),
            _ => None,
        }
    }

    pub fn insert_into_update_results(
        &mut self,
        tag: u64,
        address: EvmAddress,
        result: SimUpdateResult<U>,
    ) {
        if let Some(agent_update_results) = self.update_results.get_mut(&address) {
            agent_update_results.insert(tag, result);
        } else {
            self.update_results
                .insert(address, HashMap::from([(tag, result)]));
        }
    }

    pub fn execute(&mut self, agent_update: &AgentSimUpdate<U>) {
        match &agent_update.update {
            SimUpdate::Evm(tx) => {
                let result = self.execute_raw_evm_tx(tx);
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::Evm(result),
                    );
                }
            }
            SimUpdate::World(update) => {
                let result = self.execute_raw_world_update(update);
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::World(result),
                    );
                }
            }
            SimUpdate::Bundle(tx, update) => {
                let sim_evm_result = self.simulate_raw_evm_tx(tx);
                let bundle_success = is_execution_success(&sim_evm_result);
                if bundle_success {
                    let evm_result = self.execute_raw_evm_tx(tx);
                    let world_result = self.execute_raw_world_update(update);
                    if let Some(tag) = agent_update.tag {
                        self.insert_into_update_results(
                            tag,
                            agent_update.address,
                            SimUpdateResult::Bundle(true, evm_result, world_result),
                        );
                    }
                } else if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::Bundle(false, sim_evm_result, None),
                    );
                }
            }
            SimUpdate::MultiBundle(txs, updates) => {
                let sim_evm_results = txs
                    .iter()
                    .map(|t| self.simulate_raw_evm_tx(t))
                    .collect::<Vec<_>>();
                let bundle_success = sim_evm_results
                    .iter()
                    .map(|result| is_execution_success(result))
                    .all(|x| x);
                if bundle_success {
                    let evm_results = txs
                        .iter()
                        .map(|tx| self.execute_raw_evm_tx(tx))
                        .collect::<Vec<_>>();
                    let world_results = updates
                        .iter()
                        .map(|update| self.execute_raw_world_update(update))
                        .collect::<Vec<_>>();
                    if let Some(tag) = agent_update.tag {
                        self.insert_into_update_results(
                            tag,
                            agent_update.address,
                            SimUpdateResult::MultiBundle(true, evm_results, world_results),
                        );
                    }
                } else if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::MultiBundle(true, sim_evm_results, vec![]),
                    );
                }
            }
        }
    }
}
