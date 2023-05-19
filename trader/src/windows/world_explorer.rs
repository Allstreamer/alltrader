use crate::app::{ControlWindow, TradingGUI};
use crate::backend::push_command;
use crate::backend::Command;
use crate::backend::CommandRequest;
use crate::utils::ContinueLock;
use egui::epaint::ahash::{HashMap, HashMapExt};
use egui::plot::{Line, MarkerShape, PlotPoints, Points};
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
                    // Here is some code for debugging the plot bounds:
                    //
                    // After zooming in to about a scale of less that 500.0
                    // start rendering text
                    const TEXT_RENDER_LEVEL: f64 = 100.0;

                    // Still render Systems that are 1 unit outside of the view of the grapth
                    // This is to allow systems to render even if the center of the system
                    // isnt in view, since plants that belong to that system may still be in view.
                    const SYSTEM_CULL_LENIENCY: f64 = 1.0;

                    let render_text = plot_ui.plot_bounds().width() < TEXT_RENDER_LEVEL;

                    for system in universe_list {
                        if (plot_ui.plot_bounds().min()[0] - SYSTEM_CULL_LENIENCY > system.x as f64)
                            || ((system.x as f64)
                                > plot_ui.plot_bounds().max()[0] + SYSTEM_CULL_LENIENCY)
                            || (plot_ui.plot_bounds().min()[1] - SYSTEM_CULL_LENIENCY
                                > system.y as f64)
                            || ((system.y as f64)
                                > plot_ui.plot_bounds().max()[1] + SYSTEM_CULL_LENIENCY)
                        {
                            continue;
                        }

                        if systems_to_render.contains(&&system.symbol)
                            || !self.only_show_systems_with_ships
                        {
                            if render_text {
                                plot_ui.text(
                                    Text::new(PlotPoint::new(system.x, system.y), &system.symbol)
                                        .name("System")
                                        .color(Color32::RED),
                                );
                            } else {
                                let points = Points::new(vec![[system.x as f64, system.y as f64]])
                                    .radius(
                                        (map_range(
                                            (TEXT_RENDER_LEVEL, 20000.0),
                                            (6.0, 2.0),
                                            plot_ui.plot_bounds().width(),
                                        ) as f32)
                                            .max(2.0),
                                    )
                                    .shape(MarkerShape::Asterisk);
                                plot_ui.points(points);
                            }

                            if render_text {
                                let mut waypoint_record: HashMap<(i32, i32), u32> = HashMap::new();
                                for waypoint in &system.waypoints {
                                    let waypoint_entry = waypoint_record
                                        .entry((waypoint.x, waypoint.y))
                                        .or_insert(0);
                                    // Waypoints usualy spread around 200 units around their base system
                                    // and systems are usually 2 units apart at the core of the galaxy
                                    let waypoint_x =
                                        system.x as f64 + (waypoint.y as f64 / 200.0) * 2.0;
                                    let waypoint_y =
                                        system.y as f64 + (waypoint.x as f64 / 200.0) * 2.0;

                                    // Shift waypoint label by 0.1 for each duplicate position
                                    let waypoint_text_y =
                                        waypoint_y + (*waypoint_entry as f64 * 0.005);
                                    *waypoint_entry += 1;

                                    // Basic Point to point distance function
                                    let waypoint_distance_from_system =
                                        ((system.x as f64 - waypoint_x).powf(2.0)
                                            + (system.y as f64 - waypoint_y).powf(2.0))
                                        .sqrt();

                                    // Draw Orbit
                                    plot_ui.line(circle(
                                        system.x as f64,
                                        system.y as f64,
                                        waypoint_distance_from_system,
                                    ));

                                    // Draw waypoint label
                                    plot_ui.text(
                                        Text::new(
                                            // Text is offset on the y-axis to give space for the waypoint
                                            // icon
                                            PlotPoint::new(waypoint_x, waypoint_text_y + 0.008),
                                            &waypoint.symbol,
                                        )
                                        .name("Waypoint")
                                        .color(Color32::WHITE),
                                    );

                                    // Draw waypoint icon
                                    let points = Points::new(vec![[waypoint_x, waypoint_y]])
                                        .radius(6.0)
                                        .shape(MarkerShape::Diamond);
                                    plot_ui.points(points);
                                }
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
            let mut response_data = ContinueLock!(trading_gui.response_data.try_lock());
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

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

// https://github.com/emilk/egui/blob/7b76161a6a7e33a72e7331c1725758608c16ff30/crates/egui_demo_lib/src/demo/plot_demo.rs#LL225C15-L225C15
fn circle(x: f64, y: f64, radius: f64) -> Line {
    let n = 512;
    let circle_points: PlotPoints = (0..=n)
        .map(|i| {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=std::f64::consts::TAU);
            let r = radius;
            [r * t.cos() + x, r * t.sin() + y]
        })
        .collect();
    Line::new(circle_points)
        .color(Color32::from_rgb(100, 200, 100))
        .style(plot::LineStyle::Dashed { length: 10.0 })
        .name("orbits")
}
