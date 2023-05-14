use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use color_eyre::Result;
use spacedust::{
    apis::{agents_api::get_my_agent, configuration::Configuration, default_api::register, fleet_api::get_my_ships},
    models::{
        register_request::Faction, GetMyAgent200Response, Register201Response, RegisterRequest, GetMyShips200Response,
    },
};
use tokio::runtime::Runtime;

// Just for clarity when reading
type ResponseID = String;
#[derive(Debug)]
pub struct CommandRequest(pub Command, pub ResponseID);

#[derive(Debug)]
pub enum Command {
    Register { symbol: String, faction: Faction },
    SetToken { token: String },
    GetMyShips,
    GetMyAgent,
    Quit,
}

pub fn push_command(msg_queue: &Arc<Mutex<VecDeque<CommandRequest>>>, cmd: CommandRequest) {
    let mut msg_queue_lock = msg_queue.lock().expect("Tried to aquire lock on Mutex that was owned by panicked thread!");
    msg_queue_lock.push_front(cmd);
}

#[derive(Debug, Default)]
pub struct CommandData {
    pub agent_data: Option<(GetMyAgent200Response, ResponseID)>,
    pub register_data: Option<(Register201Response, ResponseID)>,
    pub ships_data: Option<(GetMyShips200Response, ResponseID)>,
}

macro_rules! UnwrapReq {
    ($req:expr, $id:expr) => {
        match $req {
            Ok(v) => Some((v, $id)),
            Err(e) => {
                dbg!(e);
                None
            }
        }
    };
}

pub fn run_backend(
    msg_queue: Arc<Mutex<VecDeque<CommandRequest>>>,
    response_data: Arc<Mutex<CommandData>>,
) -> Result<()> {
    let _ = std::thread::spawn(move || {
        let mut config = Configuration::new();
        let rt = Runtime::new().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for gui to lock
            let mut msg_queue_lock = msg_queue.lock().expect("Tried to aquire lock on Mutex that was owned by panicked thread!");
            if msg_queue_lock.is_empty() {
                drop(msg_queue_lock);
                continue;
            }
            // Check above garanties element
            let latest_cmd = msg_queue_lock.pop_back().unwrap();
            dbg!(&latest_cmd.0, &msg_queue_lock);
            let mut response_data_lock = response_data.lock().expect("Tried to aquire lock on Mutex that was owned by panicked thread!");
            match latest_cmd.0 {
                Command::Quit => break,
                Command::SetToken { token } => {
                    println!("Why am i getting called");
                    config.bearer_access_token = Some(token);
                },
                Command::GetMyAgent => {
                    rt.block_on(async {
                        response_data_lock.agent_data =
                            UnwrapReq!(get_my_agent(&config).await, latest_cmd.1);
                    });
                },
                Command::GetMyShips => {
                    rt.block_on(async {
                        // TODO: Create Function to get all ships even if list is longer than 20 ships
                        response_data_lock.ships_data = UnwrapReq!(get_my_ships(&config, Some(1), Some(20)).await, latest_cmd.1);
                    })
                }
                Command::Register { symbol, faction } => rt.block_on(async {
                    response_data_lock.register_data = UnwrapReq!(
                        register(&config, Some(RegisterRequest::new(faction, symbol))).await,
                        latest_cmd.1
                    );
                }),
            }
            drop(response_data_lock);
        }
    });

    Ok(())
}
