use std::fmt::Debug;

use crate::app::{ControlWindow, TradingGUI};

#[derive(Debug, Default)]
pub struct ShipInfoData {
    visible: bool,
}

macro_rules! DrawRequirements {
    ($ui:expr, $base:expr) => {
        if let Some(v) = $base.requirements.power {
            $ui.label("Required Power");
            $ui.label(v.to_string());
            $ui.end_row();
        }
        if let Some(v) = $base.requirements.crew {
            $ui.label("Required Crew");
            $ui.label(v.to_string());
            $ui.end_row();
        }
        if let Some(v) = $base.requirements.slots {
            $ui.label("Required Mount Slots");
            $ui.label(v.to_string());
            $ui.end_row();
        }
    };
}

impl ControlWindow for ShipInfoData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            match &trading_gui.game_data.selected_ship {
                Some(selected_ship) => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.collapsing(
                            format!(
                                "{} @ {}",
                                &selected_ship.symbol, selected_ship.nav.waypoint_symbol
                            ),
                            |ui| {
                                ui.collapsing("registration", |ui| {
                                    egui::Grid::new("registration_ship_info_grid")
                                        .num_columns(2)
                                        .show(ui, |ui| {
                                            if let Some(faction_symbol) =
                                                &selected_ship.registration.faction_symbol
                                            {
                                                ui.label("Faction:");
                                                ui.label(faction_symbol);
                                                ui.end_row();
                                            }

                                            ui.label("Role:");
                                            ui.label(&selected_ship.registration.role.to_string());
                                            ui.end_row();
                                        });
                                });
                                ui.collapsing("nav", |ui| {
                                    ui.label(format!(
                                        "Location: {}",
                                        selected_ship.nav.waypoint_symbol
                                    ));
                                    ui.label(format!(
                                        "Status: {}",
                                        selected_ship.nav.status.to_string()
                                    ));
                                    ui.label(format!(
                                        "Flight mode: {}",
                                        selected_ship.nav.flight_mode.to_string()
                                    ));
                                });
                                ui.collapsing("crew", |ui| {
                                    egui::Grid::new("registration_ship_info_grid")
                                        .num_columns(2)
                                        .show(ui, |ui| {
                                            ui.label("Crew Count:");
                                            ui.label(format!(
                                                "{}/{}",
                                                selected_ship.crew.current,
                                                selected_ship.crew.capacity
                                            ));
                                            ui.end_row();

                                            ui.label("Crew Requirement:");
                                            ui.label(format!(
                                                "{}/{}",
                                                selected_ship.crew.current,
                                                selected_ship.crew.required
                                            ));
                                            ui.end_row();

                                            ui.label("Crew Rotation:");
                                            ui.label(format!("{:?}", selected_ship.crew.rotation));
                                            ui.end_row();

                                            ui.label("Crew Morale:");
                                            ui.label(format!("{}/100", selected_ship.crew.morale));
                                            ui.end_row();

                                            ui.label("Crew Wage-Rate:");
                                            ui.label(format!("{}/h", selected_ship.crew.wages));
                                            ui.end_row();
                                        });
                                });
                                ui.collapsing("frame", |ui| {
                                    egui::Grid::new("registration_ship_info_grid")
                                        .num_columns(2)
                                        .show(ui, |ui| {
                                            ui.label("Name");
                                            ui.label(&selected_ship.frame.name);
                                            ui.end_row();

                                            ui.label("Description");
                                            ui.label(&selected_ship.frame.description);
                                            ui.end_row();

                                            if let Some(v) = selected_ship.frame.condition {
                                                ui.label("Condition");
                                                ui.label(v.to_string());
                                                ui.end_row();
                                            }

                                            ui.label("Module Slots");
                                            ui.label(selected_ship.frame.module_slots.to_string());
                                            ui.end_row();

                                            ui.label("Mounting Points");
                                            ui.label(
                                                selected_ship.frame.mounting_points.to_string(),
                                            );
                                            ui.end_row();
                                            DrawRequirements!(ui, selected_ship.frame);
                                        });
                                });
                                ui.collapsing("reactor", |ui| {
                                    egui::Grid::new("engine_info_grid".to_string())
                                        .num_columns(2)
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("Name");
                                            ui.label(&selected_ship.reactor.name);
                                            ui.end_row();

                                            ui.label("Description");
                                            ui.label(&selected_ship.reactor.description);
                                            ui.end_row();

                                            if let Some(v) = selected_ship.reactor.condition {
                                                ui.label("Condition");
                                                ui.label(v.to_string());
                                                ui.end_row();
                                            }
                                            ui.label("Power output");
                                            ui.label(
                                                selected_ship.reactor.power_output.to_string(),
                                            );
                                            ui.end_row();

                                            DrawRequirements!(ui, selected_ship.engine);
                                        });
                                });
                                ui.collapsing("engine", |ui| {
                                    egui::Grid::new("engine_info_grid".to_string())
                                        .num_columns(2)
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("Name");
                                            ui.label(&selected_ship.engine.name);
                                            ui.end_row();

                                            ui.label("Description");
                                            ui.label(&selected_ship.engine.description);
                                            ui.end_row();

                                            if let Some(v) = selected_ship.engine.condition {
                                                ui.label("Condition");
                                                ui.label(v.to_string());
                                                ui.end_row();
                                            }
                                            ui.label("Speed");
                                            ui.label(selected_ship.engine.speed.to_string());
                                            ui.end_row();

                                            DrawRequirements!(ui, selected_ship.engine);
                                        });
                                });
                                ui.collapsing("modules", |ui| {
                                    for (i, module) in selected_ship.modules.iter().enumerate() {
                                        ui.collapsing(format!("{} {}", i + 1, module.name), |ui| {
                                            egui::Grid::new(
                                                "module_grid".to_string() + &i.to_string(),
                                            )
                                            .num_columns(2)
                                            .striped(true)
                                            .show(
                                                ui,
                                                |ui| {
                                                    if let Some(v) = &module.description {
                                                        ui.label("Description");
                                                        ui.label(v);
                                                        ui.end_row();
                                                    }
                                                    if let Some(v) = module.range {
                                                        ui.label("Module Range");
                                                        ui.label(v.to_string());
                                                        ui.end_row();
                                                    }
                                                    if let Some(v) = module.capacity {
                                                        ui.label("Module Capacity");
                                                        ui.label(v.to_string());
                                                        ui.end_row();
                                                    }
                                                    DrawRequirements!(ui, module);
                                                },
                                            );
                                        });
                                    }
                                });
                                ui.collapsing("mounts", |ui| {
                                    for (i, mount) in selected_ship.mounts.iter().enumerate() {
                                        // String formatting is not only for style but as a method to
                                        // create uniqe ids
                                        ui.collapsing(format!("{} {}", i + 1, mount.name), |ui| {
                                            egui::Grid::new("mount_grid".to_string())
                                                .num_columns(2)
                                                .striped(true)
                                                .show(ui, |ui| {
                                                    // TODO: Coding sin: Repetition
                                                    // May wanna another make a macro for this
                                                    if let Some(v) = &mount.description {
                                                        ui.label("Description");
                                                        ui.label(v);
                                                        ui.end_row();
                                                    }
                                                    if let Some(v) = mount.strength {
                                                        ui.label("Attachment Strength");
                                                        ui.label(v.to_string());
                                                        ui.end_row();
                                                    }
                                                    DrawRequirements!(ui, mount);
                                                });
                                        });
                                    }
                                });
                                ui.collapsing("cargo", |ui| {
                                    ui.label(format!(
                                        "Cargo: {}/{}",
                                        selected_ship.cargo.units, selected_ship.cargo.capacity
                                    ));

                                    egui::Grid::new("cargo_inventory_grid").num_columns(2).show(
                                        ui,
                                        |ui| {
                                            for item in &selected_ship.cargo.inventory {
                                                ui.label(&item.name);
                                                ui.label(format!("x{}", item.units));
                                                ui.end_row();
                                            }
                                        },
                                    )
                                });
                                ui.label(format!(
                                    "Fuel: {}/{}",
                                    selected_ship.fuel.current, selected_ship.fuel.capacity
                                ));
                            },
                        );
                    });
                }
                None => {}
            }
        });
    }

    fn name(&self) -> String {
        String::from("Ship Info")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
