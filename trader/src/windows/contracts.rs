use crate::{
    app::{ControlWindow, TradingGUI},
    backend::{push_command, Command, CommandRequest},
};
#[derive(Debug, Default)]
pub struct ContractsData {
    selected_contract: String,
    visible: bool,
}

impl ControlWindow for ContractsData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            ui.heading("Contracts");
            if let Some(contract_list) = &trading_gui.game_data.contract_data {
                egui::Grid::new("ship_list_grid")
                    .num_columns(1)
                    .spacing([40.0, 20.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for contract in contract_list {
                            ui.selectable_value(
                                &mut trading_gui.game_data.selected_contract,
                                Some(contract.clone()),
                                &contract.id.clone(),
                            );
                            ui.end_row();
                        }
                    });
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Refresh").clicked() {
                    push_command(
                        &trading_gui.msg_queue,
                        CommandRequest(Command::GetMyContracts, self.name()),
                    );
                }
            });
        });
        {
            let mut response_data = trading_gui.response_data.lock().unwrap();
            if let Some(v) = &response_data.contract_data {
                if v.1 == self.name() {
                    trading_gui.game_data.contract_data = Some(v.0.data.to_owned());
                    response_data.contract_data = None;
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("Contracts")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
