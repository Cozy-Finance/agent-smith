use std::{
    borrow::Cow,
    cmp::min,
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
    sync::Arc,
};

use bindings::cozy_protocol::cozy_router;
use revm::primitives::TxEnv;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
    u256::{f64_to_u256, U256},
    utils::{is_execution_success, unpack_execution},
};

use crate::cozy::{
    agents::errors::{CozyAgentError, CozyAgentResult},
    constants::*,
    types::CozyAgentTriggerProbModel,
    utils::{float_to_wad, wad},
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyPtokenLogic, CozyRouter, CozySetLogic},
};

pub struct ActiveBuyer {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    ptoken_logic: Arc<CozyPtokenLogic>,
    set_logic: Arc<CozySetLogic>,
    target_trigger: Address,
    protection_owned: U256,
    ptokens_owned: HashMap<(Address, u16), U256>,
    capital: U256,
    waiting_time: U256,
    last_action_time: U256,
    trigger_prob_dist: CozyAgentTriggerProbModel,
    rng: rand::rngs::StdRng,
}

#[derive(Debug, Clone)]
pub struct ActiveBuyerTxData {
    tx_type: Cow<'static, str>,
    amt: U256,
    set_address: Address,
    market_id: u16,
}

impl Display for ActiveBuyerTxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:X} {} {}",
            self.tx_type, self.set_address, self.market_id, self.amt
        )?;
        Ok(())
    }
}

impl FromStr for ActiveBuyerTxData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!(
                "ActiveBuyerTxData string must split into four tokens: {}.",
                s
            ));
        }

        let amt = U256::from_dec_str(parts.pop().expect("Checked parts.len() == 4."))?;
        let market_id: u16 = parts.pop().expect("Checked parts.len() == 4.").parse()?;
        let set_address: Address = parts.pop().expect("Checked parts.len() == 4.").parse()?;
        let tx_type: String = parts.pop().expect("Checked parts.len() == 4.").into();

        Ok(ActiveBuyerTxData {
            tx_type: tx_type.into(),
            amt,
            set_address,
            market_id,
        })
    }
}

impl ActiveBuyer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        set_logic: &Arc<CozySetLogic>,
        ptoken_logic: &Arc<CozyPtokenLogic>,
        target_trigger: Address,
        waiting_time: f64,
        trigger_prob_dist: CozyAgentTriggerProbModel,
        rng: rand::rngs::StdRng,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            ptoken_logic: ptoken_logic.clone(),
            target_trigger,
            protection_owned: U256::zero(),
            ptokens_owned: HashMap::new(),
            capital: U256::zero(),
            waiting_time: f64_to_u256(waiting_time),
            last_action_time: U256::zero(),
            trigger_prob_dist,
            rng,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for ActiveBuyer {
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
        channel.send(SimUpdate::Evm(
            self.token
                .build_max_approve_router_tx(self.address, self.cozyrouter.address)
                .expect("ActiveBuyer failed to build approve tx."),
        ));
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("ActiveBuyer failed to read token balance.");
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.is_time_to_act(state.read_timestamp()) || self.capital <= U256::zero() {
            return;
        }

        let targets = match self.get_target_sets_and_markets_ids(
            state,
            state.world.sets.values(),
            &self.target_trigger,
        ) {
            targets if targets.is_empty() => return,
            targets => targets,
        };

        let oracle_trigger_prob = state
            .world
            .triggers
            .get_by_addr(&self.target_trigger)
            .ok_or(CozyAgentError::UnregisteredAddress(self.target_trigger))
            .expect("ActiveBuyer queried unregistered trigger.")
            .current_prob;
        let my_trigger_prob = self
            .trigger_prob_dist
            .sample(&mut self.rng, oracle_trigger_prob);

        // Check if you want to make a purchase.
        let chosen_purchase = targets
            .iter()
            .map(|(set_address, market_id)| {
                self.get_arb_purchase_tx(state, *set_address, *market_id, my_trigger_prob)
            })
            .filter_map(Result::ok)
            .flatten()
            .max_by(|(_, a_data), (_, b_data)| a_data.amt.cmp(&b_data.amt));

        if let Some((chosen_purchase_tx, chosen_purchase_data)) = chosen_purchase {
            channel.send_with_tag(
                SimUpdate::Evm(chosen_purchase_tx),
                format!("{}", chosen_purchase_data).into(),
            );

            // Approve CozyRouter to spend corresponding pTokens, in case you want to sell.
            let ptoken_addr = self
                .set_logic
                .read_ptoken_addr(
                    self.address,
                    state,
                    chosen_purchase_data.set_address,
                    chosen_purchase_data.market_id,
                )
                .expect("ActiveBuyer failed to read pToken address.");
            let ptoken_approve_tx = self
                .ptoken_logic
                .build_max_approve_router_tx(self.address, ptoken_addr, self.cozyrouter.address)
                .expect("ActiveBuyer failed to build pToken approve tx.");

            channel.send(SimUpdate::Evm(ptoken_approve_tx));
        } else {
            // Check if you want to make a sale.
            let chosen_sale = targets
                .iter()
                .map(|(set_address, market_id)| {
                    self.get_arb_sell_tx(state, *set_address, *market_id, my_trigger_prob)
                })
                .filter_map(Result::ok)
                .flatten()
                .max_by(|(_, a_data), (_, b_data)| a_data.amt.cmp(&b_data.amt));

            if let Some((chosen_sale_tx, chosen_sale_data)) = chosen_sale {
                channel.send_with_tag(
                    SimUpdate::Evm(chosen_sale_tx),
                    format!("{}", chosen_sale_data).into(),
                );
            }
        }
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        if !self.is_time_to_act(state.read_timestamp()) {
            return;
        }
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("ActiveBuyer failed to read token balance.");
        self.last_action_time = state.read_timestamp();

        if let Some(update_results) = state.update_results.get(&self.address) {
            for (tag, result) in update_results {
                match result {
                    SimUpdateResult::Evm(result) if tag.parse::<ActiveBuyerTxData>().is_ok() => {
                        let tx_data: ActiveBuyerTxData =
                            tag.parse().expect("ActiveBuyer failed to parse tag.");
                        if tx_data.tx_type == ACTIVE_BUYER_PURCHASE && is_execution_success(result)
                        {
                            let ptokens_received = self
                                .cozyrouter
                                .decode_ptokens_received(result)
                                .expect("ActiveBuyer failed to decode pTokens received.");
                            match self
                                .ptokens_owned
                                .get_mut(&(tx_data.set_address, tx_data.market_id))
                            {
                                None => {
                                    self.ptokens_owned.insert(
                                        (tx_data.set_address, tx_data.market_id),
                                        ptokens_received,
                                    );
                                }
                                Some(curr_ptokens) => {
                                    *curr_ptokens += Into::<U256>::into(ptokens_received);
                                }
                            };
                        } else if tx_data.tx_type == ACTIVE_BUYER_SALE
                            && is_execution_success(result)
                        {
                            match self
                                .ptokens_owned
                                .get_mut(&(tx_data.set_address, tx_data.market_id))
                            {
                                None => {}
                                Some(curr_ptokens) => {
                                    *curr_ptokens -= tx_data.amt;
                                }
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        self.protection_owned = U256::zero();
        for ((set_addr, set_market_id), ptokens) in self.ptokens_owned.iter() {
            self.protection_owned += self
                .set_logic
                .read_protection_balance(self.address, state, *set_addr, *set_market_id, *ptokens)
                .expect("ActiveBuyer failed to read protection balance.");
        }
    }
}

impl ActiveBuyer {
    fn is_time_to_act(&self, curr_timestamp: U256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
    }

    fn get_target_sets_and_markets_ids(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        sets: &[CozySet],
        trigger: &Address,
    ) -> Vec<(Address, u16)> {
        sets.iter()
            .filter(|set| set.trigger_lookup.contains_key(trigger))
            .map(|set| {
                (
                    set.address,
                    *set.trigger_lookup.get(trigger).expect("Checked in filter."),
                )
            })
            .collect::<Vec<_>>()
    }

    fn get_arb_purchase_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
        my_prob: f64,
    ) -> CozyAgentResult<Option<(TxEnv, ActiveBuyerTxData)>> {
        let mut purchase_amt = self.set_logic.read_remaining_protection(
            self.address,
            state,
            set_address,
            market_id,
        )?;
        let mut max_cost = U256::MAX;
        loop {
            if purchase_amt == U256::zero() || max_cost == U256::zero() {
                return Ok(None);
            }
            max_cost = min((purchase_amt * float_to_wad(my_prob)) / wad(), self.capital);
            let purchase_args = cozy_router::PurchaseCall {
                set: set_address.into(),
                market_id,
                protection: purchase_amt,
                receiver: self.address.into(),
                max_cost,
            };
            let purchase_tx = self
                .cozyrouter
                .build_purchase_tx(self.address, purchase_args)?;
            match unpack_execution(state.simulate_evm_tx_ref(&purchase_tx, None)?) {
                Ok(_) => {
                    return Ok(Some((
                        purchase_tx,
                        ActiveBuyerTxData {
                            tx_type: ACTIVE_BUYER_PURCHASE.into(),
                            amt: purchase_amt,
                            set_address,
                            market_id,
                        },
                    )));
                }
                _ => {
                    purchase_amt /= 2;
                    continue;
                }
            };
        }
    }

    fn get_arb_sell_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
        my_prob: f64,
    ) -> CozyAgentResult<Option<(TxEnv, ActiveBuyerTxData)>> {
        let mut sell_amt = match self.ptokens_owned.get(&(set_address, market_id)) {
            None => return Ok(None),
            Some(ptokens_owned) => *ptokens_owned,
        };
        loop {
            if sell_amt == U256::zero() {
                return Ok(None);
            }
            let sell_amt_value = self.set_logic.read_protection_balance(
                self.address,
                state,
                set_address,
                market_id,
                sell_amt,
            )?;
            let min_refund = (sell_amt_value * float_to_wad(my_prob)) / wad();
            let sell_args = cozy_router::SellCall {
                set: set_address.into(),
                market_id,
                ptokens: sell_amt,
                receiver: self.address.into(),
                min_refund,
            };
            let sell_tx = self
                .cozyrouter
                .build_sell_tx(self.address, sell_args.clone())?;
            match unpack_execution(state.simulate_evm_tx_ref(&sell_tx, None)?) {
                Ok(_) => {
                    return Ok(Some((
                        sell_tx,
                        ActiveBuyerTxData {
                            tx_type: ACTIVE_BUYER_SALE.into(),
                            amt: sell_amt,
                            set_address,
                            market_id,
                        },
                    )));
                }
                _ => {
                    sell_amt /= 2;
                    continue;
                }
            };
        }
    }
}
