use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use color_eyre::Result;
use spacedust::{
    apis::{configuration::Configuration, default_api::register},
    models::{register_request::Faction, Register201Response, RegisterRequest},
};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum Command {
    Register { symbol: String, faction: Faction },
    Quit,
}

#[derive(Debug, Default)]
pub struct CommandData {
    pub register_data: Option<Register201Response>,
}

pub fn run_backend(
    msg_queue: Arc<Mutex<VecDeque<Command>>>,
    response_data: Arc<Mutex<CommandData>>,
) -> Result<()> {
    let _ = std::thread::spawn(move || {
        let config = Configuration::new();
        let rt = Runtime::new().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for gui to lock
            let mut msg_queue_lock = msg_queue.lock().expect("FUGGG noooooo");
            if msg_queue_lock.is_empty() {
                drop(msg_queue_lock);
                continue;
            }
            // Check above garanties element
            let latest_cmd = msg_queue_lock.pop_back().unwrap();
            match latest_cmd {
                Command::Quit => break,
                Command::Register { symbol, faction } => {
                    let mut response_data_lock =
                        response_data.lock().expect("OH SHIT, it's going down");
                    rt.block_on(async {
                        response_data_lock.register_data =
                            register(&config, Some(RegisterRequest::new(faction, symbol)))
                                .await
                                .ok();
                    })
                }
            }
        }
    });

    Ok(())
}
