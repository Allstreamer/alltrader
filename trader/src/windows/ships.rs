use crate::{
    app::{ControlWindow, TradingGUI},
    backend::{push_command, Command, CommandRequest},
    utils::ContinueLock,
};

#[derive(Debug, Default)]
pub struct ShipMenuData {
    visible: bool,
}

impl ControlWindow for ShipMenuData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            if let Some(ship_list) = &trading_gui.game_data.ship_data {
                egui::Grid::new("ship_list_grid")
                    .num_columns(1)
                    .spacing([40.0, 20.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for ship in ship_list {
                            ui.selectable_value(
                                &mut trading_gui.game_data.selected_ship,
                                Some(ship.clone()),
                                &ship.symbol,
                            );
                            ui.end_row();
                        }
                    });
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Refresh").clicked() {
                    push_command(
                        &trading_gui.msg_queue,
                        CommandRequest(Command::GetMyShips, self.name()),
                    );
                }
            });
        });

        {
            let mut response_data = ContinueLock!(trading_gui.response_data.try_lock());
            if let Some(v) = &response_data.ships_data {
                if v.1 == self.name() {
                    trading_gui.game_data.ship_data = Some(v.0.data.to_owned());
                    response_data.ships_data = None;
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("Ships")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
