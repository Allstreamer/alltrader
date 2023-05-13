use std::sync::Arc;
use std::sync::Mutex;

use spacedust::apis::configuration::Configuration;
use spacedust::models::Register201ResponseData;
use spacedust::models::register_request::Faction;

pub async fn backend_main() -> color_eyre::Result<()> {


	Ok(())
}

enum Message {
	Register{symbol: String, faction: Faction},
}

struct BackendState {
    config: Arc<Mutex<Configuration>>,
	register_response: Option<Register201ResponseData>
}
