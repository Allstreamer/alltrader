use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    backend::{Command, CommandData},
    windows::auth::AuthMenuData,
};

pub fn gui_main(
    msg_queue: Arc<Mutex<VecDeque<Command>>>,
    response_data: Arc<Mutex<CommandData>>,
) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
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
    fn visability(&mut self) -> &mut bool;
}

pub struct TradingGUI {
    pub menus: Arc<Mutex<Vec<Box<dyn ControlWindow>>>>,
    pub msg_queue: Arc<Mutex<VecDeque<Command>>>,
    pub response_data: Arc<Mutex<CommandData>>,
}

impl TradingGUI {
    fn new(
        msg_queue: Arc<Mutex<VecDeque<Command>>>,
        response_data: Arc<Mutex<CommandData>>,
    ) -> Self {
        Self {
            menus: Arc::new(Mutex::new(vec![Box::<AuthMenuData>::default()])),
            msg_queue,
            response_data,
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
                        let mut msg_queue_lock = self.msg_queue.lock().expect("Shit");
                        msg_queue_lock.push_front(Command::Quit);
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Menus");

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    let menus = Arc::clone(&self.menus);
                    let mut menus_lock = menus.lock().unwrap();
                    for menu in menus_lock.iter_mut() {
                        let menu_name = menu.name().to_owned();
                        ui.toggle_value(menu.visability(), menu_name);
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
            let mut menus_lock = menus.lock().unwrap();
            for i in 0..menus_lock.len() {
                if *menus_lock[i].visability() {
                    menus_lock[i].draw(self, ctx);
                }
            }
        }
    }
}
