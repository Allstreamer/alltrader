use std::sync::{Mutex, Arc};

use crate::windows::auth::AuthMenuData;


pub trait ControlWindow {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context);
    fn name(&self) -> String;
    fn visability(&mut self) -> &mut bool;
}

#[derive(Clone)]
pub struct TradingGUI {
    pub api_config: spacedust::apis::configuration::Configuration,
    pub menus: Arc<Mutex<Vec<Box<dyn ControlWindow>>>>
}

impl Default for TradingGUI {
    fn default() -> Self {
        // config.bearer_access_token = Some(dotenv!("TOKEN").into());
        Self { 
            api_config: spacedust::apis::configuration::Configuration::new(),
            menus: Arc::new(Mutex::new(vec![
                Box::new(AuthMenuData::default())
            ]))
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
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Menus");

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                    let menus = Arc::clone(&self.menus);
                    let mut menus_lock = menus.lock().unwrap();
                    for menu in menus_lock.iter_mut() {
                        let menu_name = menu.name().to_owned();
                        ui.toggle_value(menu.visability(), menu_name);
                    }
            });

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

impl TradingGUI {

}