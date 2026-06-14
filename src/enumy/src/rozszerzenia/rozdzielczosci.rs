
use strum::{Display, EnumIter, EnumMessage};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum Rozdzielczości {
    #[strum(message = "roz_16", detailed_message = "info_roz_16")]
    R16 = 16,
    #[strum(message = "roz_32", detailed_message = "info_roz_32")]
    R32 = 32,
    #[strum(message = "roz_64", detailed_message = "info_roz_64")]
    R64 = 64,
    #[strum(message = "roz_128", detailed_message = "info_roz_128")]
    R128 = 128,
    #[strum(message = "roz_256", detailed_message = "info_roz_256")]
    R256 = 256,
    #[strum(message = "roz_512", detailed_message = "info_roz_512")]
    R512 = 512,
    #[strum(message = "roz_1024", detailed_message = "info_roz_1024")]
    R1k = 1024,
    #[strum(message = "roz_2048", detailed_message = "info_roz_2048")]
    R2k = 2048,
    #[strum(message = "roz_4096", detailed_message = "info_roz_4096")]
    R4k = 4096,
    #[strum(message = "roz_6144", detailed_message = "info_roz_6144")]
    R6k = 6144,
    #[strum(message = "roz_8192", detailed_message = "info_roz_8192")]
    R8k = 8192,
    #[strum(message = "roz_16384", detailed_message = "info_roz_16384")]
    R16k = 16384,
    #[strum(message = "roz_org", detailed_message = "info_roz_org")]
    Oryginalna = 0,
}

// impl Display for Rozdzielczości {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Rozdzielczości::R16 => write!(f, "16"),
//             Rozdzielczości::R32 => write!(f, "32"),
//             Rozdzielczości::R64 => write!(f, "64"),
//             Rozdzielczości::R128 => write!(f, "128"),
//             Rozdzielczości::R256 => write!(f, "256"),
//             Rozdzielczości::R512 => write!(f, "512"),
//             Rozdzielczości::R1k => write!(f, "1024"),
//             Rozdzielczości::R2k => write!(f, "2048"),
//             Rozdzielczości::R4k => write!(f, "4096"),
//             Rozdzielczości::R6k => write!(f, "6144"),
//             Rozdzielczości::R8k => write!(f, "8192"),
//             Rozdzielczości::R16k => write!(f, "16384"),
//             Rozdzielczości::Oryginalna => write!(f, "Oryginal"),
//         }
//     }
// }
impl Rozdzielczości{
    /// # Get small name
    /// Mainly for ui
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::R16 => "16",
            Self::R32 => "32",
            Self::R64 => "64",
            Self::R128 => "128",
            Self::R256 => "256",
            Self::R512 => "512",
            Self::R1k => "1k",
            Self::R2k => "2k",
            Self::R4k => "4",
            Self::R6k => "6k",
            Self::R8k => "8k",
            Self::R16k => "16k",
            Self::Oryginalna => "org",
        }
    }
    /// # Get name
    /// If u'll call such fn then u'll get extension according to choosen resolution.
    /// 
    /// Purpose is to use that for naming convencion.
    pub fn rozszerzenie(&self) -> &'static str {
        match self{
            Self::R16 => "_16",
            Self::R32 => "_32",
            Self::R64 => "_64",
            Self::R128 => "_128",
            Self::R256 => "_256",
            Self::R512 => "_512",
            Self::R1k => "_1024",
            Self::R2k => "_2k",
            Self::R4k => "_4k",
            Self::R6k => "_6k",
            Self::R8k => "_8k",
            Self::R16k => "_16k",
            Self::Oryginalna => "",
        }
    }
}
