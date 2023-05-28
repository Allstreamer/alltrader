use crate::app::{ControlWindow, TradingGUI};
use crate::backend::push_command;
use crate::backend::Command;
use crate::backend::CommandRequest;
use crate::utils::ContinueLock;
use regex::Regex;
#[derive(Debug, Default)]
pub struct StatusData {
    visible: bool,
}

impl ControlWindow for StatusData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            if let Some(status) = &trading_gui.game_data.status_data {
                egui::ScrollArea::vertical()
                    .max_height(600.0)
                    .show(ui, |ui| {
                        egui::Grid::new("status")
                            .num_columns(1)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Status");
                                ui.label(&status.status);
                            });
                        egui::Grid::new("version")
                            .num_columns(1)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Version");
                                ui.label(&status.version);
                            });
                        egui::Grid::new("reset")
                            .num_columns(1)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("last reset");
                                ui.label(&status.reset_date);
                            });
                        egui::Grid::new("Description")
                            .num_columns(2)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Description");
                                ui.add(egui::Label::new(&status.description).wrap(true));
                            });
                        egui::Grid::new("Stats")
                            .num_columns(2)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.collapsing("Stats", |ui| {
                                    egui::Grid::new("Stats_info_grid").num_columns(2).show(
                                        ui,
                                        |ui| {
                                            ui.label("Agents:");
                                            ui.label(&status.stats.agents.to_string());
                                            ui.end_row();

                                            ui.label("Ships:");
                                            ui.label(&status.stats.ships.to_string());
                                            ui.end_row();
                                            ui.label("Systems:");
                                            ui.label(&status.stats.systems.to_string());
                                            ui.end_row();
                                            ui.label("waypoints:");
                                            ui.label(&status.stats.waypoints.to_string());
                                            ui.end_row();
                                        },
                                    );
                                });
                            });
                        egui::Grid::new("Leaderboard")
                            .num_columns(2)
                            .spacing([40.0, 20.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.collapsing("Leaderboard", |ui| {
                                    egui::Grid::new("credits_info_grid")
                                        .num_columns(2)
                                        .min_col_width(500.0)
                                        .show(ui, |ui| {
                                            ui.collapsing("Credits", |ui| {
                                                let mut credit_leaderboard_data =
                                                    status.leaderboards.most_credits.clone();
                                                for (i, credit) in
                                                    credit_leaderboard_data.iter_mut().enumerate()
                                                {
                                                    ui.label(format!(
                                                        "{}. {} {}",
                                                        i + 1,
                                                        &credit.agent_symbol,
                                                        &credit.credits
                                                    ));
                                                    ui.end_row();
                                                }
                                            });
                                            ui.end_row();
                                            egui::Grid::new("charts_info_grid")
                                                .num_columns(2)
                                                .min_col_width(500.0)
                                                .show(ui, |ui| {
                                                    ui.collapsing("Charts", |ui| {
                                                        let mut chart_leaderboard_data = status
                                                            .leaderboards
                                                            .most_submitted_charts
                                                            .clone();
                                                        for (i, chart) in chart_leaderboard_data
                                                            .iter_mut()
                                                            .enumerate()
                                                        {
                                                            ui.label(format!(
                                                                "{}. {} {}",
                                                                i + 1,
                                                                &chart.agent_symbol,
                                                                &chart.chart_count
                                                            ));
                                                            ui.end_row();
                                                        }
                                                    })
                                                })
                                        });
                                });
                            });
                        egui::Grid::new("resets")
                            .num_columns(1)
                            .spacing([40.0, 20.0])
                            .striped(false)
                            .show(ui, |ui| {
                                ui.label("next reset");
                                ui.label(&status.server_resets.next);
                                ui.end_row();
                                ui.label("frequency");
                                ui.label(&status.server_resets.frequency);
                            });
                        egui::Grid::new("announcements_info_grid")
                            .num_columns(1)
                            .show(ui, |ui| {
                                ui.collapsing("announcements", |ui| {
                                    let mut announcements = status.announcements.clone();
                                    for (_i, announcement) in announcements.iter_mut().enumerate() {
                                        ui.collapsing(&announcement.title, |ui| {
                                            let segments = extract_segments(&announcement.body);
                                            for segment in &segments {
                                                match segment {
                                                    Segment::String(string) => ui.label(string),
                                                    Segment::Url(url) => ui.hyperlink_to(url, url),
                                                };
                                            }
                                            ui.end_row();
                                        });
                                    }
                                })
                            });
                        egui::Grid::new("links_info_grid")
                            .num_columns(1)
                            .show(ui, |ui| {
                                ui.collapsing("links", |ui| {
                                    let mut links = status.links.clone();
                                    for (_i, link) in links.iter_mut().enumerate() {
                                        ui.hyperlink_to(&link.name, &link.url);
                                        ui.end_row();
                                    }
                                })
                            });
                    });
            };

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Refresh").clicked() {
                    push_command(
                        &trading_gui.msg_queue,
                        CommandRequest(Command::GetStatus, self.name()),
                    );
                }
            });
        });
        {
            let mut response_data = ContinueLock!(trading_gui.response_data.try_lock());
            if let Some(v) = &response_data.status_data {
                if v.1 == self.name() {
                    trading_gui.game_data.status_data = Some(v.0.to_owned());
                    response_data.status_data = None;
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("Status")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}

#[derive(Debug)]
enum Segment {
    String(String),
    Url(String),
}

fn extract_segments(text: &str) -> Vec<Segment> {
    let re = Regex::new(r"\b(https?://\S+)\b").unwrap();

    let mut results: Vec<Segment> = Vec::new();

    let mut last_index = 0;
    for capture in re.captures_iter(text) {
        let url_start = capture.get(0).unwrap().start();
        let url_end = capture.get(0).unwrap().end();

        if last_index != url_start {
            let string = &text[last_index..url_start];
            results.push(Segment::String(string.to_string()));
        }

        let url = &text[url_start..url_end];
        results.push(Segment::Url(url.to_string()));

        last_index = url_end;
    }

    if last_index < text.len() {
        let string = &text[last_index..];
        results.push(Segment::String(string.to_string()));
    }

    results
}
