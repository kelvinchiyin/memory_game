use eframe::egui;

#[derive(Debug, Clone)]
pub enum InputDirection {
    Forward,
    Reverse,
}

impl InputDirection {
    pub fn color(&self) -> egui::Color32 {
        match self {
            InputDirection::Forward => egui::Color32::from_rgb(0, 150, 255),
            InputDirection::Reverse => egui::Color32::from_rgb(255, 100, 100),
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            InputDirection::Forward => "FORWARD",
            InputDirection::Reverse => "REVERSE",
        }
    }
}
