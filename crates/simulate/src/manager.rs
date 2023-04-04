#![warn(missing_docs)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

// use core::slice::SlicePattern;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::RwLock as AsyncRwLock;

use bytes::Bytes;
use revm::primitives::{AccountInfo, ExecutionResult, Output, B160};

use crate::{
    agent::{admin::Admin, Agent, user::User},
    environment::SimulationEnvironment,
};

// TODO: Maybe need a `SimulationAccount` that abstracts some of the revm primitives further.

/// Manages simulations.
pub struct SimulationManager<'a> {
    /// `SimulationEnvironment` that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the simulation environment.
    pub agents: HashMap<&'a str, Box<dyn Agent>>,
}

impl<'a> Default for SimulationManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SimulationManager<'a> {
    /// Constructor function to instantiate a
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
        };
        let admin = Box::new(Admin::new());
        simulation_manager.add_agent("admin", admin);
        simulation_manager
    }
    // /// Returns a reference to the admin agent.
    // pub fn admin(&self) -> &Box<dyn Agent> {
    //     self.agents.get("admin").unwrap()
    // }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    /// Add an [`Agent`] to the current simulation.
    pub fn add_agent(&mut self, name: &'a str, agent: Box<dyn Agent>) {
        self.agents.insert(name, agent);
    }

    // TODO: maybe should make the name optional here, but I struggled with this.
    /// Allow the manager to create a dummy user account.
    pub async fn create_user(&mut self, address: B160, name: &'a str) {
        self.environment
            .evm
            .write()
            .await
            .db()
            .unwrap()
            .insert_account_info(address, AccountInfo::default());
        let user = Box::new(User::new(address));
        self.add_agent(name, user);
    }

    /// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
    pub fn unpack_execution(&self, execution_result: ExecutionResult) -> Bytes {
        match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Call(value) => value,
                Output::Create(_, Some(_)) => {
                    panic!("Failed. This was a 'Create' call, use 'Deploy' instead.")
                }
                _ => panic!("This call has failed."),
            },
            _ => panic!("This call generated no execution result. This should not happen."),
        }
    }
}
