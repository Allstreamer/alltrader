use spacedust::models::{Agent, Ship};

#[derive(Debug, Clone, Default)]
pub struct GameData {
    pub agent_data: Option<Agent>,
    pub ship_data: Option<Vec<Ship>>
}
