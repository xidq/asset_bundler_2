use strum::{Display, EnumIter, EnumMessage, IntoStaticStr};


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, Display, Default)]
pub enum ForDdsKompresja {
    Fast,
    #[default]
    Normal,
    High,
    Unreasonable,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumMessage, Display,Default)]
pub enum ForFfKompresja {
    #[strum(message = "comp_Zstd", detailed_message = "hint_comp_Zstd")]
    Zstd(u8),  //1-22|3
    #[strum(message = "comp_Bzip2", detailed_message = "hint_comp_Bzip2")]
    Bzip2(u8), //1-9|6?
    #[strum(message = "comp_Xz", detailed_message = "hint_comp_Xz")]
    Xz(u8),    //1-9|6
    #[default]
    #[strum(message = "comp_Zstd", detailed_message = "hint_comp_Zstd")]
    Brak,
}

impl ForFfKompresja {
    pub fn ustaw_domyslny_poziom(self) -> Self {
        match self {
            Self::Zstd(_) => Self::Zstd(3),  // Twoje domyślne 3
            Self::Bzip2(_) => Self::Bzip2(6), // Twoje domyślne 6
            Self::Xz(_) => Self::Xz(6),       // Twoje domyślne 6
            Self::Brak => Self::Brak,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, Display)]
pub enum ForDds {
    DxgiFormatBc1Unorm,
    DxgiFormatBc1UnormSrgb,
    DxgiFormatBc1Typeless,
    DxgiFormatBc2Unorm,
    DxgiFormatBc2UnormSrgb,
    DxgiFormatBc2Typeless,
    DxgiFormatBc3Unorm,
    DxgiFormatBc3UnormSrgb,
    DxgiFormatBc3Typeless,
    DxgiFormatBc4Unorm,
    DxgiFormatBc4Snorm,
    DxgiFormatBc4Typeless,
    DxgiFormatBc5Unorm,
    DxgiFormatBc5Snorm,
    DxgiFormatBc5Typeless,
    DxgiFormatBc6HUF16,
    DxgiFormatBc6HSF16,
    DxgiFormatBc6HTypeless,
    DxgiFormatBc7Unorm,
    DxgiFormatBc7UnormSrgb,
    DxgiFormatBc7Typeless,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumIter, EnumMessage, Display, Default)]
pub enum ForAvifKompresja {
    Undefined,
    Hevc,
    Avc,
    Jpeg,
    #[default]
    Av1, //default
    Vvc,
    Evc,
    Jpeg2000,
    Uncompressed,
    Mask,
    HtJ2k,
}

impl ForAvifKompresja {
    pub fn krotkie(&self) -> &'static str{
        match self{
            Self::Undefined => "UND",
            Self::Hevc => "HEV",
            Self::Avc => "AVC",
            Self::Jpeg => "JPG",
            Self::Av1 => "AV1",
            Self::Vvc => "VVC",
            Self::Evc => "EVC",
            Self::Jpeg2000 => "JP2",
            Self::Uncompressed => "UNC",
            Self::Mask => "MAS",
            Self::HtJ2k => "HJ2",
        }
    }
}
