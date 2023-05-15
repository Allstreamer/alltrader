use crate::parse_system::System;
use spacedust::models::{Agent, Contract, Ship};
#[derive(Debug, Clone, Default)]
pub struct GameData {
    pub agent_data: Option<Agent>,
    pub ship_data: Option<Vec<Ship>>,
    pub selected_ship: Option<Ship>,
    pub contract_data: Option<Vec<Contract>>,
    pub selected_contract: Option<Contract>,
    pub universe_data: Option<Vec<System>>,
}
