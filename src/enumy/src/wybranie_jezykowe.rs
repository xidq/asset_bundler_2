use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn z_systemu(kod: String) -> Self {
        let kod = kod.to_lowercase();

        // Mapowanie kodów ISO na Twoje warianty
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
pub enum DevToolsMenu {
    // Wariant z polem 'jezyk'
    UstawieniaJęzyka { jezyk: WybórJęzyka },
    // Tutaj możesz dodawać kolejne warianty w ten sam sposób:
    TrybDebugowania { aktywny: bool },
}
