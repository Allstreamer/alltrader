use std::future::Future;
use std::str::FromStr;

use spacedust::apis::default_api::register;
use spacedust::models::RegisterRequest;
use spacedust::models::register_request::Faction;
use spacedust::apis::default_api::RegisterError;

use crate::app::TradingGUI;
use crate::app::ControlWindow;

pub struct AuthMenuData {
    temp_agent_name: String,
    temp_token: String,
    temp_faction: Faction,
    visable: bool,
    register_promise: Option<Box<dyn Future<Output = Result<spacedust::models::Register201Response, spacedust::apis::Error<RegisterError>>>>>,
}

impl Default for AuthMenuData {
    fn default() -> Self {
        Self { 
            temp_agent_name: Default::default(),
            temp_token: Default::default(),
            temp_faction: Default::default(),
            visable: false,
            register_promise: None
        }
    }
}

impl ControlWindow for AuthMenuData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new("Auth").show(ctx, |ui| {
            ui.heading("Create Agent");
            egui::TextEdit::singleline(&mut self.temp_agent_name).hint_text("Agency Name").show(ui);

            egui::ComboBox::from_label("Select Faction")
                .selected_text(format!("{:?}", self.temp_faction))
                .show_ui(ui, |ui: &mut egui::Ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.temp_faction, Faction::Cosmic, "Cosmic");
                    ui.selectable_value(&mut self.temp_faction, Faction::Dominion, "Dominion");
                    ui.selectable_value(&mut self.temp_faction, Faction::Galactic, "Galactic");
                    ui.selectable_value(&mut self.temp_faction, Faction::Quantum, "Quantum");
                    ui.selectable_value(&mut self.temp_faction, Faction::Void, "Void");
                });

            if ui.button("Create Agent").clicked() {
                //self.register_promise = Some(Box::new(register(&trading_gui.api_config, Some(RegisterRequest::new(self.temp_faction, self.temp_agent_name)))));
            }
    
            ui.separator();
            ui.heading("Set Token");
            ui.text_edit_singleline(&mut self.temp_token);
            if ui.button("Set").clicked() {
                //trading_gui.api_config.bearer_access_token = Some(self.temp_token.to_owned());
            }
        });
    }

    fn name(&self) -> String {
        String::from_str("Auth").unwrap()
    }

    fn visability(&mut self) -> &mut bool {
        &mut self.visable
    }
}