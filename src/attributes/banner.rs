pub use crate::errors::error::CartError;
pub use crate::colors::CartColor;
use std::fmt::Display;

pub trait Banner {
    fn display(&self, message: &CartColor);
}

pub struct CartBanner;

impl CartBanner {
    pub fn new() -> Self {
        Self
    }

    pub fn show_banner(&self, content: &str, color: CartColor) {
        self.display(&match color {
            CartColor::Black(_) => CartColor::Black(content.to_string()),
            CartColor::Red(_) => CartColor::Red(content.to_string()),
            CartColor::Green(_) => CartColor::Green(content.to_string()),
            CartColor::Yellow(_) => CartColor::Yellow(content.to_string()),
            CartColor::Blue(_) => CartColor::Blue(content.to_string()),
            CartColor::Magenta(_) => CartColor::Magenta(content.to_string()),
            CartColor::Cyan(_) => CartColor::Cyan(content.to_string()),
            CartColor::White(_) => CartColor::White(content.to_string()),
            CartColor::BrightBlack(_) => CartColor::BrightBlack(content.to_string()),
            CartColor::BrightRed(_) => CartColor::BrightRed(content.to_string()),
            CartColor::BrightGreen(_) => CartColor::BrightGreen(content.to_string()),
            CartColor::BrightYellow(_) => CartColor::BrightYellow(content.to_string()),
            CartColor::BrightBlue(_) => CartColor::BrightBlue(content.to_string()),
            CartColor::BrightMagenta(_) => CartColor::BrightMagenta(content.to_string()),
            CartColor::BrightCyan(_) => CartColor::BrightCyan(content.to_string()),
            CartColor::BrightWhite(_) => CartColor::BrightWhite(content.to_string()),
            CartColor::Reset(_) => CartColor::Reset(content.to_string()),
        });
    }

    pub fn main_banner(&self, content: &str) {
        self.show_banner(content.to_string(), CartColor::White("".to_string()));
    }

    pub fn loading_banner(&self, content: &str) {
        self.show_banner(content.to_string(), CartColor::BrightCyan("".to_string()));
    }

    pub fn building_banner(&self, content: &str) {
        self.show_banner(content.to_string(), CartColor::BrightYellow("".to_string()));
    }

    pub fn downloading_banner(&self, content: &str) {
        self.show_banner(content.to_string(), CartColor::BrightGreen("".to_string()));
    }
}

impl Banner for CartBanner {
    fn display(&self, message: &CartColor) {
        println!("{}", message);
    }
}
