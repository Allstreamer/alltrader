use spacedust::models::Agent;

#[derive(Debug, Clone, Default)]
pub struct GameData {
    pub agent_data: Option<Agent>,
}
