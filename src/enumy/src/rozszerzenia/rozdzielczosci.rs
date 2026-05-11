
use strum::{Display, EnumIter, EnumMessage};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, EnumIter, EnumMessage, Display)]
pub enum Rozdzielczości {
    #[strum(message = "roz_16", detailed_message = "info_roz_16")]
    R16,
    #[strum(message = "roz_32", detailed_message = "info_roz_32")]
    R32,
    #[strum(message = "roz_64", detailed_message = "info_roz_64")]
    R64,
    #[strum(message = "roz_128", detailed_message = "info_roz_128")]
    R128,
    #[strum(message = "roz_256", detailed_message = "info_roz_256")]
    R256,
    #[strum(message = "roz_512", detailed_message = "info_roz_512")]
    R512,
    #[strum(message = "roz_1024", detailed_message = "info_roz_1024")]
    R1k,
    #[strum(message = "roz_2048", detailed_message = "info_roz_2048")]
    R2k,
    #[strum(message = "roz_4096", detailed_message = "info_roz_4096")]
    R4k,
    #[strum(message = "roz_6144", detailed_message = "info_roz_6144")]
    R6k,
    #[strum(message = "roz_8192", detailed_message = "info_roz_8192")]
    R8k,
    #[strum(message = "roz_16384", detailed_message = "info_roz_16384")]
    R16k,
    #[strum(message = "roz_org", detailed_message = "info_roz_org")]
    Oryginalna,
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
}
