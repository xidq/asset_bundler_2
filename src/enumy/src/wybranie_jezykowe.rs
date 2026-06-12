use std::fmt;
use strum::EnumIter;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Eq)]
pub enum WybórJęzyka {
    PL,
    EN,
    DE,
    HU,
    ES,
    KR,
    JP,
    TH,
}
/// # Lang related stuff
/// So here we can find imlpementations
impl WybórJęzyka {
    pub const ALL: [WybórJęzyka; 8] = [
        Self::PL,
        Self::EN,
        Self::DE,
        Self::HU,
        Self::ES,
        Self::KR,
        Self::JP,
        Self::TH,
    ];

    /// # Mapping
    /// Mapping languages for codes and stuff
    /// 
    /// for now:
    /// - English
    /// - Deutsch
    /// - Magyar
    /// - Espanol
    /// - Korean
    /// - Japaneese
    /// - Thai
    ///
    /// There's no guarantee that I translated all of that for those languages for now tho 
    pub fn z_systemu(kod: String) -> Self {
        let kod = kod.to_lowercase();

        // Mapowanie kodów ISO na warianty
        if kod.starts_with("pl") {
            Self::PL
        } else if kod.starts_with("de") {
            Self::DE
        } else if kod.starts_with("hu") {
            Self::HU
        } else if kod.starts_with("es") {
            Self::ES
        } else if kod.starts_with("kr") {
            Self::KR
        } else if kod.starts_with("jp") {
            Self::JP
        } else if kod.starts_with("th") {
            Self::TH
        } else {
            // Jeśli system to cokolwiek innego (np. fr, it, jp) -> English jako fallback
            Self::EN
        }
    }
    /// # Gettin' font
    /// Different alphabets can have different needs of font related stuff
    /// 
    /// So Asian languages have their own embedded in binary,
    /// ofc I changed one for latin alphabet too as u can see in gui...
    pub fn get_font(&self) -> iced::Font {
        match self {
            WybórJęzyka::KR => iced::Font {
                family: iced::font::Family::Name("Noto Serif KR"), // Nazwa z pliku TTF
                ..Default::default()
            },
            WybórJęzyka::JP => iced::Font {
                family: iced::font::Family::Name("Noto Serif JP"),
                ..Default::default()
            },
            WybórJęzyka::TH => iced::Font {
                family: iced::font::Family::Name("Noto Sans Thai Condensed"),
                ..Default::default()
            },
            _ => iced::Font {
                family: iced::font::Family::Name("Lato"),
                ..Default::default()
            },
        }
    }
}

impl fmt::Display for WybórJęzyka {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Używamy składni strukturalnej wewnątrz Enuma
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UstawieniaMenu {
    // Wariant z polem 'jezyk'
    UstawieniaJęzyka { jezyk: WybórJęzyka },
    // Tutaj możesz dodawać kolejne warianty w ten sam sposób:
    TrybDebugowania { aktywny: bool },
}
