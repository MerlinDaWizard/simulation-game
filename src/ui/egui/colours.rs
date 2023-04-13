use egui::Color32;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColourScheme {
    // Meta
    pub name: String,
    // Main
    pub crust: Srgba_Unmultiplied,
    pub mantle: Srgba_Unmultiplied,
    pub base: Srgba_Unmultiplied,
    pub surface0: Srgba_Unmultiplied,
    pub surface1: Srgba_Unmultiplied,
    pub surface2: Srgba_Unmultiplied,
    pub overlay0: Srgba_Unmultiplied,
    pub overlay1: Srgba_Unmultiplied,
    pub overlay2: Srgba_Unmultiplied,
    pub subtext0: Srgba_Unmultiplied,
    pub subtext1: Srgba_Unmultiplied,
    pub text: Srgba_Unmultiplied,
    // Accents
    pub maroon: Srgba_Unmultiplied,
    pub lavender: Srgba_Unmultiplied,
    pub blue: Srgba_Unmultiplied,
    pub sapphire: Srgba_Unmultiplied,
    pub sky: Srgba_Unmultiplied,
    pub teal: Srgba_Unmultiplied,
    pub green: Srgba_Unmultiplied,
    pub yellow: Srgba_Unmultiplied,
    pub peach: Srgba_Unmultiplied,
    pub red: Srgba_Unmultiplied,
    pub mauve: Srgba_Unmultiplied,
    pub pink: Srgba_Unmultiplied,
    pub flamingo: Srgba_Unmultiplied,
    pub rosewater: Srgba_Unmultiplied,
}

/// An p colour in the SSrgba_Unmultiplied colour space. E.g. SSrgba_Unmultiplieda(30, 32, 48, 255)
/// We serialise and deserialise using this instead of colour32 due to rust's no implementing traits on foreign types & the fact that Srgba_Unmultiplied contains private fields
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Srgba_Unmultiplied(pub [u8; 4]);

impl From<Srgba_Unmultiplied> for Color32 {
    fn from(item: Srgba_Unmultiplied) -> Self {
        Color32::from_rgba_unmultiplied(item.0[0], item.0[1], item.0[2], item.0[3])
    }
}

impl From<Color32> for Srgba_Unmultiplied {
    fn from(item: Color32) -> Self {
        Srgba_Unmultiplied(item.to_srgba_unmultiplied())
    }
}
