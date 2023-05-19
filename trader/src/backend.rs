use crate::{parse_system::System, utils::ContinueLock};
use color_eyre::Result;
use spacedust::{
    apis::{
        agents_api::get_my_agent,
        configuration::Configuration,
        contracts_api::get_contracts,
        default_api::register,
        fleet_api::{get_my_ships, refuel_ship},
    },
    models::{
        register_request::Faction, register_request::RegisterRequest, GetContracts200Response,
        GetMyAgent200Response, GetMyShips200Response, RefuelShip200Response, Register201Response,
        Ship,
    },
};
use std::{collections::VecDeque, sync::Arc};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::utils::UnwrapReq;

use crate::parse_system::parse_json;

// Just for clarity when reading
type ResponseID = String;
#[derive(Debug)]
pub struct CommandRequest(pub Command, pub ResponseID);

#[derive(Debug)]
pub enum Command {
    Register {
        symbol: String,
        faction: Faction,
        email: String,
    },
    SetToken {
        token: String,
    },
    GetMyShips,
    GetMyAgent,
    GetConfig,
    GetMyContracts,
    GetUniverse,
    Refuel {
        ship: Ship,
    },
    Quit,
}
use crate::config::get_config_key;
use crate::config::set_config_key;

pub fn push_command(msg_queue: &Arc<Mutex<VecDeque<CommandRequest>>>, cmd: CommandRequest) {
    let mut msg_queue_lock = ContinueLock!(msg_queue.try_lock());
    msg_queue_lock.push_front(cmd);
}

#[derive(Debug, Default)]
pub struct CommandData {
    pub agent_data: Option<(GetMyAgent200Response, ResponseID)>,
    pub register_data: Option<(Register201Response, ResponseID)>,
    pub ships_data: Option<(GetMyShips200Response, ResponseID)>,
    pub contract_data: Option<(GetContracts200Response, ResponseID)>,
    pub universe_data: Option<(Vec<System>, ResponseID)>,
    pub refuel_data: Option<(RefuelShip200Response, ResponseID)>,
}

pub fn run_backend(
    msg_queue: Arc<Mutex<VecDeque<CommandRequest>>>,
    response_data: Arc<Mutex<CommandData>>,
) -> Result<()> {
    let _ = std::thread::spawn(move || {
        let mut config = Configuration::new();
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for gui to lock
                let latest_cmd = {
                    let mut msg_queue_lock = msg_queue.lock().await;
                    if msg_queue_lock.is_empty() {
                        drop(msg_queue_lock);
                        continue;
                    }
                    // Check above garanties element
                    msg_queue_lock.pop_back().unwrap()
                };
                dbg!(&latest_cmd.0);
                let mut response_data_lock = response_data.lock().await;
                match latest_cmd.0 {
                    Command::Quit => break,
                    Command::SetToken { token } => {
                        set_config_key("spacetraders", "token", &token);
                        config.bearer_access_token = Some(token);
                    }
                    Command::GetMyAgent => {
                        response_data_lock.agent_data =
                            UnwrapReq!(get_my_agent(&config).await, latest_cmd.1);
                    }
                    Command::GetMyShips => {
                        // TODO: Create Function to get all ships even if list is longer than 20 ships
                        response_data_lock.ships_data = UnwrapReq!(
                            get_my_ships(&config, Some(1), Some(20)).await,
                            latest_cmd.1
                        );
                    }
                    Command::Register {
                        symbol,
                        faction,
                        email,
                    } => {
                        let mut request = RegisterRequest::new(faction, symbol);
                        if !email.is_empty() {
                            request.email = Some(email);
                        }
                        response_data_lock.register_data =
                            UnwrapReq!(register(&config, Some(request)).await, latest_cmd.1);
                    }
                    Command::GetConfig => {
                        let token = get_config_key("spacetraders", "token");
                        config.bearer_access_token = token;
                    }
                    Command::GetMyContracts => {
                        response_data_lock.contract_data = UnwrapReq!(
                            get_contracts(&config, Some(1), Some(20)).await,
                            latest_cmd.1
                        );
                    }
                    Command::GetUniverse => {
                        response_data_lock.universe_data =
                            UnwrapReq!(parse_json().await, latest_cmd.1)
                    }
                    Command::Refuel { ship } => {
                        response_data_lock.refuel_data =
                            UnwrapReq!(refuel_ship(&config, &ship.symbol, 0).await, latest_cmd.1);
                    }
                }
                drop(response_data_lock);
            }
        });
    });

    Ok(())
}
