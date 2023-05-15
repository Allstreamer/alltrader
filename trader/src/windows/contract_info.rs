use std::fmt::Debug;

use crate::app::{ControlWindow, TradingGUI};

#[derive(Debug, Default)]
pub struct ContractInfoData {
    visible: bool,
}

impl ControlWindow for ContractInfoData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            match &trading_gui.game_data.selected_contract {
                Some(selected_contract) => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.collapsing(&selected_contract.id, |ui| {
                            ui.label(format!("faction: {}", &selected_contract.faction_symbol));
                            // wierd that there is no to_string method for the enum but this works fine
                            ui.label(format!("type: {:?}", &selected_contract.r#type));
                            ui.collapsing("terms", |ui| {
                                // TODO: better date formatting
                                ui.label(format!(
                                    "deadline: {}",
                                    &selected_contract.terms.deadline
                                ));
                                ui.collapsing("payment", |ui| {
                                    // TODO: better date formatting
                                    ui.label(format!(
                                        "on accept: {}",
                                        &selected_contract.terms.payment.on_accepted
                                    ));
                                    ui.label(format!(
                                        "on fulfill: {}",
                                        &selected_contract.terms.payment.on_fulfilled
                                    ));
                                    ui.collapsing("deliver", |ui| {
                                        if let Some(goods) = &selected_contract.terms.deliver {
                                            for good in goods {
                                                ui.collapsing(&good.trade_symbol, |ui| {
                                                    ui.label(format!(
                                                        "destination: {}",
                                                        good.destination_symbol
                                                    ));
                                                    ui.label(format!(
                                                        "units required: {}",
                                                        good.units_required
                                                    ));
                                                    ui.label(format!(
                                                        "units fulfilled: {}",
                                                        good.units_fulfilled
                                                    ));
                                                });
                                            }
                                        }
                                    });
                                });
                            });
                            ui.label(format!("accepted: {}", &selected_contract.accepted));
                            ui.label(format!("fulfilled: {}", &selected_contract.fulfilled));
                            ui.label(format!("expiration: {}", &selected_contract.expiration));
                        });
                    });
                }
                None => {}
            }
        });
    }

    fn name(&self) -> String {
        String::from("Contract Info")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
