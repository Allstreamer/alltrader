use crate::app::{ControlWindow, TradingGUI};


#[derive(Debug, Default)]
pub struct XData {
    visible: bool,
}

impl ControlWindow for XData {
    fn draw(&mut self, trading_gui: &mut TradingGUI, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            ui.heading("Create Agent");
        });
    }

    fn name(&self) -> String {
        String::from("X")
    }

    fn visibility(&mut self) -> &mut bool {
        &mut self.visible
    }
}
