use crate::{
    app::{ControlWindow, TradingGUI},
    backend::{push_command, Command, CommandRequest},
    utils::ExpectLock,
};

#[derive(Debug, Default)]
pub struct AgentData {
    visible: bool,
}

impl ControlWindow for AgentData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            if let Some(v) = &trading_gui.game_data.agent_data {
                egui::Grid::new("user_data_grid")
                    .num_columns(2)
                    .spacing([40.0, 20.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("User ID");
                        ui.label(&v.account_id);
                        ui.end_row();

                        ui.label("Headquaters");
                        ui.label(&v.headquarters);
                        ui.end_row();

                        ui.label("Symbol:");
                        ui.label(&v.symbol);
                        ui.end_row();

                        ui.label("Credits:");
                        ui.label(v.credits.to_string());
                        ui.end_row();
                    });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    if ui.button("Refresh").clicked() {
                        push_command(
                            &trading_gui.msg_queue,
                            CommandRequest(Command::GetMyAgent, self.name()),
                        );
                    }
                });
                {
                    let response_data = ExpectLock!(trading_gui.response_data.lock());
                    if let Some(v) = &response_data.agent_data {
                        if v.1 == self.name() {
                            trading_gui.game_data.agent_data = Some(*v.0.data.clone());
                        }
                    }
                }
            }
        });
    }

    fn name(&self) -> String {
        String::from("Agent View")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
