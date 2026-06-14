use crate::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use crate::rozszerzenia::ext::{ImgExt, ImgExtSingle};
use crate::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};
use crate::rozszerzenia::rozszenienia_zdjec::{ImgExtAvif, ImgExtExr, ImgExtFf, ImgExtJpg, ImgExtPng, ImgExtQoi, ImgExtTga, ImgExtWebp};

impl ImgExt {
    pub fn def_jpg() -> ImgExt {
        ImgExt::Jpg {
            jakosc: 90,
            progresywny: false,
            bit_depth: vec![BdepthJpg::Rgb8],
            sampling: ForJpgSamplingFac::R420,
            quant: ForJpgQuant::Default,
            scans: 4,
        }
    }
    pub fn def_png() -> ImgExt {
        ImgExt::Png {
            kompresja: 3,
            bit_depth: vec![BdepthPng::Rgb8],
        }
    }
    pub fn def_avif() -> ImgExt {
        ImgExt::Avif {
            chroma: ForAvifChroma::C420,
            speed: 3,
            metoda_kompresji: ForAvifKompresja::Av1,
            lossy: Some(90),
            bit_depth: vec![BdepthAvif::Rgb10],
        }
    }
    pub fn def_tga() -> ImgExt {
        ImgExt::Tga {
            bit_depth: vec![BdepthTga::TrueColor24],
        }
    }
    pub fn def_qoi() -> ImgExt {
        ImgExt::Qoi {
            bit_depth: vec![BdepthQoi::Color24],
        }
    }
    pub fn def_ff() -> ImgExt {
        ImgExt::Ff {
            metoda_kompresji: ForFfKompresja::Brak,
        }
    }
    pub fn def_exr() -> ImgExt {
        ImgExt::Exr { 
            bit_depth: vec![BdepthExr::F16], 
            kompresja: ForExrKompresja::Brak, 
        }
    }
    pub fn def_webp() -> ImgExt {
        ImgExt::Webp {
            jakosc: 90,
            lossless: false,
            bit_depth: vec![BdepthWebp::Rgb8],
        }
    }
}
impl ImgExtSingle {
    pub fn def_jpg() -> ImgExtSingle {
        ImgExtSingle::Jpg {
            jakosc: 90,
            progresywny: false,
            bit_depth: BdepthJpg::Rgb8,
            sampling: ForJpgSamplingFac::R420,
            quant: ForJpgQuant::Default,
            scans: 4,
        }
    }
    pub fn def_png() -> ImgExtSingle {
        ImgExtSingle::Png {
            kompresja: 3,
            bit_depth: BdepthPng::Rgb8,
        }
    }
    pub fn def_avif() -> ImgExtSingle {
        ImgExtSingle::Avif {
            chroma: ForAvifChroma::C420,
            speed: 3,
            metoda_kompresji: ForAvifKompresja::Av1,
            lossy: Some(90),
            bit_depth: BdepthAvif::Rgb10,
        }
    }
    pub fn def_tga() -> ImgExtSingle {
        ImgExtSingle::Tga {
            bit_depth: BdepthTga::TrueColor24,
        }
    }
    pub fn def_qoi() -> ImgExtSingle {
        ImgExtSingle::Qoi {
            bit_depth: BdepthQoi::Color24,
        }
    }
    pub fn def_ff() -> ImgExtSingle {
        ImgExtSingle::Ff {
            metoda_kompresji: ForFfKompresja::Brak,
        }
    }
    pub fn def_exr() -> ImgExtSingle {
        ImgExtSingle::Exr {
            bit_depth: BdepthExr::F16,
            kompresja: ForExrKompresja::Brak,
        }
    }
    pub fn def_webp() -> ImgExtSingle {
        ImgExtSingle::Webp {
            jakosc: 90,
            lossless: false,
            bit_depth: BdepthWebp::Rgb8,
        }
    }
}


impl Default for ImgExtJpg {
    fn default() -> Self {
        ImgExtJpg{
            jakosc: 90,
            progresywny: false,
            bit_depth: vec![BdepthJpg::Rgb8],
            sampling: ForJpgSamplingFac::R420,
            quant: ForJpgQuant::Default,
            scans: 4,
        }
    }
}
impl Default for ImgExtPng {
    fn default() -> Self {
        ImgExtPng{
            kompresja: 3,
            bit_depth: vec![BdepthPng::Rgb8],
        }
    }
}
impl Default for ImgExtAvif {
    fn default() -> Self {
        ImgExtAvif{
            chroma: ForAvifChroma::C420,
            speed: 3,
            metoda_kompresji: ForAvifKompresja::Av1,
            lossy: Some(90),
            bit_depth: vec![BdepthAvif::Rgb10],
        }
    }
}
impl Default for ImgExtTga {
    fn default() -> Self {
        ImgExtTga{
            bit_depth: vec![BdepthTga::TrueColor24],
        }
    }
}
impl Default for ImgExtWebp {
    fn default() -> Self {
        ImgExtWebp{
            jakosc: 90,
            lossless: false,
            bit_depth: vec![BdepthWebp::Rgb8],
        }
    }
}
impl Default for ImgExtQoi{
    fn default() -> Self {
        ImgExtQoi{
            bit_depth: vec![BdepthQoi::Color24],
        }
    }
}
impl Default for ImgExtFf{
    fn default() -> Self {
        ImgExtFf{
            metoda_kompresji: ForFfKompresja::Brak,
        }
    }
}
impl Default for ImgExtExr{
    fn default() -> Self {
        ImgExtExr{
            bit_depth: vec![BdepthExr::F16],
            kompresja: ForExrKompresja::Brak,
        }
    }
}