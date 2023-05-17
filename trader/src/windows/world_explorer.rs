use crate::app::{ControlWindow, TradingGUI};
use crate::backend::push_command;
use crate::backend::Command;
use crate::backend::CommandRequest;
use crate::utils::ExpectLock;
use egui::*;
use plot::{Corner, Legend, Plot, PlotPoint, Text};
#[derive(Debug, Default)]
pub struct WorldExplorerData {
    only_show_systems_with_ships: bool,
    visible: bool,
}

impl ControlWindow for WorldExplorerData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            let plot = Plot::new("universe")
                .legend(Legend::default().position(Corner::RightBottom))
                .show_x(true)
                .show_y(true)
                .width(500.0)
                .height(500.0)
                .data_aspect(1.0);
            ui.checkbox(
                &mut self.only_show_systems_with_ships,
                "Only show systems with ships",
            );
            let mut systems_to_render = vec![];
            if let Some(ship_list) = &trading_gui.game_data.ship_data {
                for ship in ship_list {
                    systems_to_render.push(&ship.nav.system_symbol);
                }
            }
            if let Some(universe_list) = &trading_gui.game_data.universe_data {
                plot.show(ui, |plot_ui| {
                    for system in universe_list {
                        if systems_to_render.contains(&&system.symbol)
                            || !self.only_show_systems_with_ships
                        {
                            plot_ui.text(
                                Text::new(PlotPoint::new(system.y, system.x), &system.symbol)
                                    .name("System")
                                    .color(Color32::RED),
                            );
                            for waypoint in &system.waypoints {
                                plot_ui.text(
                                    Text::new(
                                        PlotPoint::new(
                                            // Waypoints usualy spread around 200 units around their base system
                                            // and systems are usually 2 units apart at the core of the galaxy
                                            system.y as f64 + (waypoint.y as f64 / 200.0) * 2.0,
                                            system.x as f64 + (waypoint.x as f64 / 200.0) * 2.0,
                                        ),
                                        &waypoint.symbol,
                                    )
                                    .name("Waypoint")
                                    .color(Color32::BLUE),
                                );
                            }
                        }
                    }
                    if let Some(ship_list) = &trading_gui.game_data.ship_data {
                        for ship in ship_list {
                            plot_ui.text(
                                Text::new(
                                    PlotPoint::new(
                                        ship.nav.route.destination.y,
                                        ship.nav.route.destination.y,
                                    ),
                                    &ship.symbol,
                                )
                                .name("Ship")
                                .color(Color32::GREEN),
                            );
                        }
                    }
                });
            }

            if ui.button("Refresh").clicked() {
                push_command(
                    &trading_gui.msg_queue,
                    CommandRequest(Command::GetUniverse, self.name()),
                );
            };
        });

        {
            let mut response_data = ExpectLock!(trading_gui.response_data.lock());
            if let Some(v) = &response_data.universe_data {
                if v.1 == self.name() {
                    trading_gui.game_data.universe_data = Some(v.clone().0);
                    response_data.universe_data = None;
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("WorldExplorer")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
