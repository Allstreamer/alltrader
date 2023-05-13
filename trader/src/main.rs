use color_eyre::Result;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

mod app;
mod backend;
mod windows;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let msg_queue = Arc::new(Mutex::new(VecDeque::new()));
    let response_data = Arc::new(Mutex::new(backend::CommandData::default()));

    let msg_queue_clone = Arc::clone(&msg_queue);
    let response_data_clone = Arc::clone(&response_data);

    backend::run_backend(msg_queue_clone, response_data_clone)?;
    app::gui_main(msg_queue, response_data).unwrap();
    Ok(())
}
