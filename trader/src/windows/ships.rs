use crate::app::{ControlWindow, TradingGUI};

#[derive(Debug, Default)]
pub struct ShipMenuData {
    visible: bool,
}

impl ControlWindow for ShipMenuData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            ui.heading("Ship List");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Refresh").clicked() {}
            });
        });
    }

    fn name(&self) -> String {
        String::from("Ships")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
