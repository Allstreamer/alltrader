use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    backend::{push_command, Command, CommandData, CommandRequest},
    gamedata::GameData,
    utils::ContinueLock,
    windows::{
        agent::AgentData, auth::AuthMenuData, contract_info::ContractInfoData,
        contracts::ContractsData, ship_info::ShipInfoData, ships::ShipMenuData, status::StatusData,
        world_explorer::WorldExplorerData,
    },
};
use egui::Ui;

fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../res/icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn gui_main(
    msg_queue: Arc<Mutex<VecDeque<CommandRequest>>>,
    response_data: Arc<Mutex<CommandData>>,
) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        icon_data: Some(load_icon()),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "All-Trader",
        options,
        Box::new(|_cc| Box::new(TradingGUI::new(msg_queue, response_data))),
    )?;
    Ok(())
}

pub trait ControlWindow {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context);
    fn name(&self) -> String;
    fn visibility(&mut self) -> &mut bool;
}

pub struct TradingGUI {
    pub menus: Arc<Mutex<Vec<Box<dyn ControlWindow>>>>,
    pub msg_queue: Arc<Mutex<VecDeque<CommandRequest>>>,
    pub response_data: Arc<Mutex<CommandData>>,
    pub game_data: GameData,
}

impl TradingGUI {
    fn new(
        msg_queue: Arc<Mutex<VecDeque<CommandRequest>>>,
        response_data: Arc<Mutex<CommandData>>,
    ) -> Self {
        Self {
            menus: Arc::new(Mutex::new(vec![
                Box::<AuthMenuData>::default(),
                Box::<AgentData>::default(),
                Box::<ShipMenuData>::default(),
                Box::<ShipInfoData>::default(),
                Box::<ContractsData>::default(),
                Box::<ContractInfoData>::default(),
                Box::<WorldExplorerData>::default(),
                Box::<StatusData>::default(),
            ])),
            msg_queue,
            response_data,
            game_data: Default::default(),
        }
    }
}

impl eframe::App for TradingGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                        // ID may be empty below since we aren't expecting/processing a response
                        push_command(&self.msg_queue, CommandRequest(Command::Quit, "".into()));
                    }
                });
                global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Menus");

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    let menus = Arc::clone(&self.menus);
                    let mut menus_lock = ContinueLock!(menus.try_lock());
                    for menu in menus_lock.iter_mut() {
                        let menu_name = menu.name().to_owned();
                        ui.toggle_value(menu.visibility(), menu_name);
                    }
                },
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        {
            let menus = Arc::clone(&self.menus);
            let mut menus_lock = ContinueLock!(menus.try_lock());
            for i in 0..menus_lock.len() {
                if *menus_lock[i].visibility() {
                    menus_lock[i].draw(self, ctx);
                }
            }
        }
    }
}

/// Show larger buttons for switching between light and dark mode (globally).
pub fn global_dark_light_mode_buttons(ui: &mut Ui) {
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.light_dark_radio_buttons(ui);
    ui.ctx().set_visuals(visuals);
}
