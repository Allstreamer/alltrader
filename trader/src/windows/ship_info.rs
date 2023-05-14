use crate::app::{ControlWindow, TradingGUI};


#[derive(Debug, Default)]
pub struct ShipInfoData {
    visible: bool,
}

impl ControlWindow for ShipInfoData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            
        });
    }

    fn name(&self) -> String {
        String::from("Ship Info")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
