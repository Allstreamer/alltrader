use spacedust::models::register_request::Faction;

use crate::app::ControlWindow;
use crate::app::TradingGUI;
use crate::backend::push_command;
use crate::backend::Command;
use crate::backend::CommandRequest;
use crate::utils::ExpectLock;

#[derive(Debug, Default)]
pub struct AuthMenuData {
    temp_agent_name: String,
    temp_token: String,
    temp_faction: Faction,
    email: String,
    visible: bool,
}

impl ControlWindow for AuthMenuData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            ui.heading("Create Agent");
            egui::TextEdit::singleline(&mut self.temp_agent_name)
                .hint_text("Agency Name")
                .show(ui);
            egui::TextEdit::singleline(&mut self.email)
                .hint_text("Email: (for donators)")
                .show(ui);

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
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(
                        Command::Register {
                            symbol: self.temp_agent_name.clone(),
                            faction: self.temp_faction,
                            email: self.email.clone(),
                        },
                        self.name(),
                    ),
                );
            }

            ui.separator();
            ui.heading("Set Token");
            ui.text_edit_singleline(&mut self.temp_token);
            if ui.button("Set").clicked() {
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(
                        Command::SetToken {
                            token: self.temp_token.clone(),
                        },
                        self.name(),
                    ),
                );
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(Command::GetMyAgent, self.name()),
                );
            }
            ui.separator();
            ui.heading("Get Agent");
            if ui.button("Get").clicked() {
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(Command::GetConfig, self.name()),
                );
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(Command::GetMyAgent, self.name()),
                );
            }
        });

        {
            let mut response_data = ExpectLock!(trading_gui.response_data.lock());
            if let Some(v) = &response_data.agent_data {
                if v.1 == self.name() {
                    trading_gui.game_data.agent_data = Some(*v.0.data.clone());
                    response_data.agent_data = None;
                }
            }

            if let Some(v) = &response_data.register_data {
                if v.1 == self.name() {
                    self.temp_token = v.0.data.token.clone();
                    trading_gui.game_data.ship_data = Some(vec![*v.0.data.ship.clone()]);

                    push_command(
                        &trading_gui.msg_queue,
                        CommandRequest(
                            Command::SetToken {
                                token: v.0.data.token.clone(),
                            },
                            self.name(),
                        ),
                    );
                    push_command(
                        &trading_gui.msg_queue,
                        CommandRequest(Command::GetMyAgent, self.name()),
                    );
                    response_data.register_data = None;
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("Auth")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
