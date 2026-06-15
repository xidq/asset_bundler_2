use crate::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use crate::rozszerzenia::ext::ImgExt::Jpg;
use crate::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};
use strum::{Display, EnumIter, EnumMessage};

// pub const OPTFORMATDDS: &[OptFormatDds] = &[
//     OptFormatDds::DxgiFormatBc1Unorm,
//     OptFormatDds::DxgiFormatBc1UnormSrgb,
//     OptFormatDds::DxgiFormatBc1Typeless,
//     OptFormatDds::DxgiFormatBc2Unorm,
//     OptFormatDds::DxgiFormatBc2UnormSrgb,
//     OptFormatDds::DxgiFormatBc2Typeless,
//     OptFormatDds::DxgiFormatBc3Unorm,
//     OptFormatDds::DxgiFormatBc3UnormSrgb,
//     OptFormatDds::DxgiFormatBc3Typeless,
//     OptFormatDds::DxgiFormatBc4Unorm,
//     OptFormatDds::DxgiFormatBc4Snorm,
//     OptFormatDds::DxgiFormatBc4Typeless,
//     OptFormatDds::DxgiFormatBc5Unorm,
//     OptFormatDds::DxgiFormatBc5Snorm,
//     OptFormatDds::DxgiFormatBc5Typeless,
//     OptFormatDds::DxgiFormatBc6HUF16,
//     OptFormatDds::DxgiFormatBc6HSF16,
//     OptFormatDds::DxgiFormatBc6HTypeless,
//     OptFormatDds::DxgiFormatBc7Unorm,
//     OptFormatDds::DxgiFormatBc7UnormSrgb,
//     OptFormatDds::DxgiFormatBc7Typeless,
// ];
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumMessage, EnumIter, Display)]
pub enum ImgExt {
    Jpg {
        jakosc: u8,
        progresywny: bool,
        bit_depth: Vec<BdepthJpg>,
        sampling: ForJpgSamplingFac,
        quant: ForJpgQuant,
        scans:u8,
    },
    Png {
        kompresja: u8,
        bit_depth: Vec<BdepthPng>,
    },
    Webp {
        jakosc: u8,
        lossless: bool,
        bit_depth: Vec<BdepthWebp>,
    },
    Tga {
        bit_depth: Vec<BdepthTga>,
    },
    Ff {
        metoda_kompresji: ForFfKompresja,
    },
    Qoi {
        bit_depth: Vec<BdepthQoi>,
    },
    Avif{
        chroma: ForAvifChroma,
        speed:u8, //i32
        metoda_kompresji: ForAvifKompresja,
        lossy:Option<u8>,
        bit_depth: Vec<BdepthAvif>,
    },
    Exr{
        bit_depth: Vec<BdepthExr>,
        kompresja: ForExrKompresja,
    }
}
impl ImgExt {
    pub fn ma_wybrany_bit_depth(&self) -> bool {
        match self {
            // Warianty, które mają wektor bit_depth
            Self::Jpg { bit_depth, .. } => !bit_depth.is_empty(),
            Self::Png { bit_depth, .. } => !bit_depth.is_empty(),
            Self::Webp { bit_depth, .. } => !bit_depth.is_empty(),
            Self::Tga { bit_depth, .. } => !bit_depth.is_empty(),
            Self::Qoi { bit_depth, .. } => !bit_depth.is_empty(),
            Self::Avif { bit_depth, .. } => !bit_depth.is_empty(),
            // Warianty bez bit_depth (FF) uznajemy za "zawsze poprawne" w tym kontekście
            Self::Ff { .. } => true,
            Self::Exr { bit_depth, .. } => !bit_depth.is_empty(),
        }
    }
}
impl Default for ImgExt {
    fn default() -> Self {
        Jpg {
            jakosc: 90,
            progresywny: false,
            bit_depth: Vec::from([BdepthJpg::Rgb8]),
            sampling: Default::default(),
            quant: Default::default(),
            scans: 4,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumMessage, Display)]
pub enum ImgExtSingle {

    Jpg {
        jakosc: u8,
        progresywny: bool,
        bit_depth: BdepthJpg,
        sampling: ForJpgSamplingFac,
        quant: ForJpgQuant,
        scans:u8,
    },
    Png {
        kompresja: u8,
        bit_depth: BdepthPng,
    },
    Webp {
        jakosc: u8,
        lossless: bool,
        bit_depth: BdepthWebp,
    },
    Tga {
        bit_depth: BdepthTga,
    },
    Ff {
        metoda_kompresji: ForFfKompresja,
    },
    Qoi {
        bit_depth: BdepthQoi,
    },
    Avif{
        chroma: ForAvifChroma,
        speed:u8,
        metoda_kompresji: ForAvifKompresja,
        lossy:Option<u8>,
        bit_depth: BdepthAvif,
    },
    Exr{
        bit_depth: BdepthExr,
        kompresja: ForExrKompresja,
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumIter, EnumMessage, Display)]
pub enum ImgExtTag {
    #[strum(message = "Jpg", detailed_message = "Joint Photographic Experts Group")]
    Jpg,
    #[strum(message = "Png", detailed_message = "Portable Network Graphics")]
    Png,
    #[strum(message = "Webp", detailed_message = "Web Photograph")]
    Webp,
    #[strum(message = "Tga", detailed_message = "Truevision TGA")]
    Tga,
    #[strum(message = "FF", detailed_message = "Farbfeld")]
    Ff,
    #[strum(message = "Qoi", detailed_message = "Quite OK Image Format")]
    Qoi,
    #[strum(message = "Avif", detailed_message = "AV1 Image File Format")]
    Avif,
    #[strum(message = "Exr", detailed_message = "OpenEXR")]
    Exr,
    #[strum(message = "Unknown", detailed_message = "Unknown")]
    Unknown,
}
impl ImgExtTag {
    pub fn małe(&self) -> &'static str {
        match self{
            Self::Jpg => "Jpg",
            Self::Png => "Png",
            Self::Webp => "Webp",
            Self::Tga => "Tga",
            Self::Ff => "FF",
            Self::Qoi => "Qoi",
            Self::Avif => "Avif",
            Self::Exr => "Exr",
            Self::Unknown => "unknown",
        }
    }
    pub fn duże(&self) -> &'static str {
        match self{
            Self::Jpg => "Joint Photographic Experts Group",
            Self::Png => "Portable Network Graphics",
            Self::Webp => "Web Photograph",
            Self::Tga => "Truevision TGA",
            Self::Ff => "Farbfeld",
            Self::Qoi => "Quite OK Image Format",
            Self::Avif => "AV1 Image File Format",
            Self::Exr => "OpenExr",
            Self::Unknown => "unknown",

        }
    }
}
