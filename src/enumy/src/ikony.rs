use iced::{Color, Element};
use iced::advanced::svg;

pub const FOLDER_PELNY_ZAMKNIETY: &str =
    include_str!("../../Data/icons/folder_pelny_zamkniety.svg");
pub const FOLDER_PUSTY_ZAMKNIETY: &str =
    include_str!("../../Data/icons/folder_pusty_zamkniety.svg");
pub const FOLDER_PUSTY_OTWARTY: &str = include_str!("../../Data/icons/folder_pusty_otwarty.svg");

pub fn folder_icon<'a, T>(
    is_active: bool,
    rodzaj: u8,
    akolor: &Color,
) -> Element<'a, T> {
    let kolor = (akolor.r,akolor.g,akolor.b);
    // 2. Definiujemy kolory (np. żółty gdy aktywny, szary gdy nie)
    fn f32_to_hex(r: f32, g: f32, b: f32, a: f32) -> String {
        // 1. Skalowanie i rzutowanie na u8
        let r_u8 = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g_u8 = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b_u8 = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
        let a_u8 = (a.clamp(0.0, 1.0) * 255.0).round() as u8;

        // Format #RRGGBBAA
        format!("#{:02x}{:02x}{:02x}{:02x}", r_u8, g_u8, b_u8, a_u8)
    }

    // Przykład użycia:
    let hex_main = f32_to_hex(kolor.0, kolor.1, kolor.2, 0.6);
    let hex_second = f32_to_hex(
        (kolor.0 - 0.3).clamp(0.0, 1.0),
        (kolor.1 - 0.3).clamp(0.0, 1.0),
        (kolor.2 - 0.3).clamp(0.0, 1.0),
        0.6,
    );

    let (main, back) = if is_active {
        (hex_main, hex_second) // Kolorowy folder
    } else {
        ("#E4E4E480".to_string(), "#CDCDCD80".to_string()) // Szary folder
    };

    // 3. Podmieniamy tagi na kolory HEX
    let processed = match rodzaj {
        1 => FOLDER_PUSTY_ZAMKNIETY
            .replace("MAIN_COLOR", &main)
            .replace("BACK_COLOR", &back),
        2 => FOLDER_PELNY_ZAMKNIETY
            .replace("MAIN_COLOR", &main)
            .replace("BACK_COLOR", &back),
        _ => FOLDER_PUSTY_OTWARTY
            .replace("MAIN_COLOR", &main)
            .replace("BACK_COLOR", &back),
    };

    // 4. Konwersja: String -> Handle
    let handle = svg::Handle::from_memory(processed.into_bytes());

    // 5. Zwracamy gotowy widget
    iced::widget::svg(handle).width(20).height(20).into()
}
