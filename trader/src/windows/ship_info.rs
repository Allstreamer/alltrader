use std::fmt::Debug;

use crate::app::{ControlWindow, TradingGUI};

#[derive(Debug, Default)]
pub struct ShipInfoData {
    visible: bool,
}

impl ControlWindow for ShipInfoData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            match &trading_gui.game_data.selected_ship {
                Some(selected_ship) => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.collapsing(&selected_ship.symbol, |ui| {
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
                            ui.collapsing("nav", |_ui| {
                                // TODO
                            });
                            ui.collapsing("crew", |ui| {
                                egui::Grid::new("registration_ship_info_grid")
                                    .num_columns(2)
                                    .show(ui, |ui| {
                                        ui.label("Crew Count:");
                                        ui.label(format!(
                                            "{}/{}",
                                            selected_ship.crew.current, selected_ship.crew.capacity
                                        ));
                                        ui.end_row();

                                        ui.label("Crew Requirement:");
                                        ui.label(format!(
                                            "{}/{}",
                                            selected_ship.crew.current, selected_ship.crew.required
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
                            ui.collapsing("frame", |_ui| {
                                // TODO
                            });
                            ui.collapsing("reactor", |_ui| {
                                // TODO
                            });
                            ui.collapsing("engine", |_ui| {
                                // TODO
                            });
                            ui.collapsing("modules", |ui| {
                                for (i, module) in selected_ship.modules.iter().enumerate() {
                                    ui.collapsing(format!("{} {}", i + 1, module.name), |ui| {
                                        egui::Grid::new("module_grid".to_string() + &i.to_string())
                                            .num_columns(2)
                                            .striped(true)
                                            .show(ui, |ui| {
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
                                                if let Some(v) = module.requirements.power {
                                                    ui.label("Required Power");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
                                                if let Some(v) = module.requirements.crew {
                                                    ui.label("Required Crew");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
                                                if let Some(v) = module.requirements.slots {
                                                    ui.label("Required Mount Slots");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
                                            });
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
                                                // May wanna make a macro for this
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
                                                if let Some(v) = mount.requirements.power {
                                                    ui.label("Required Power");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
                                                if let Some(v) = mount.requirements.crew {
                                                    ui.label("Required Crew");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
                                                if let Some(v) = mount.requirements.slots {
                                                    ui.label("Required Mount Slots");
                                                    ui.label(v.to_string());
                                                    ui.end_row();
                                                }
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
                        });
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
