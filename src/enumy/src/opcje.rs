use strum::{Display, EnumIter, IntoStaticStr};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum OptKompresjaPlikówFiltracjaPlików {
    Wszystkie,
    Graficzne,
    Audio,
    Tekstowe,
    Pdf,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum OptKompresjaPlikówPoziomKompresjiZstd {
    Brak,
    Standard,
    Duża,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum OptInterpolacja {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, Display)]
pub enum OptKompresjaDds{
    Fast,
    Normal,
    High,
    Unreasonable,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, Display)]
pub enum OptFormatDds {
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
#[derive(Clone, Debug, PartialEq)]
pub enum OptRozszerzeniaPlikówZdjęciowych {
    Jpg {
        jakosc: u8,
        progresywny: bool,
        bit_depth: Vec<OptFormatyKoloruObrazOgólny>,
    },
    Png {
        kompresja: u8,
        bit_depth: Vec<OptFormatyKoloruObrazOgólny>,
    },
    Webp {
        jakosc: u8,
        lossless: bool,
        bit_depth: Vec<OptFormatyKoloruObrazOgólny>,
    },
    Tga {
        bit_depth: Vec<OptFormatyKoloruObrazuTga>,
    },
    Ff {
        metoda_kompresji: OptMetodaKompresjiZdjecia,
    },
    Qoi {
        bit_depth: Vec<OptFormatyKoloruObrazuQoi>,
    },
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum OptRozszerzeniaPlikówZdjęciowychPojedyncze {
    Jpg {
        jakosc: u8,
        progresywny: bool,
        bit_depth: OptFormatyKoloruObrazOgólny,
    },
    Png {
        kompresja: u8,
        bit_depth: OptFormatyKoloruObrazOgólny,
    },
    Webp {
        jakosc: u8,
        lossless: bool,
        bit_depth: OptFormatyKoloruObrazOgólny,
    },
    Tga {
        bit_depth: OptFormatyKoloruObrazuTga,
    },
    Ff {
        metoda_kompresji: OptMetodaKompresjiZdjecia,
    },
    Qoi {
        bit_depth: OptFormatyKoloruObrazuQoi,
    },
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum OptRozszerzeniaPlikówZdjęciowychZnacznik{
    Jpg,
    Png,
    Webp,
    Tga,
    Ff,
    Qoi
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OptMetodaKompresjiZdjecia {
    Zstd(u8),  //1-22|3
    Bzip2(u8), //1-9|?
    Xz(u8),    //1-9|6
    Brak,
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OptFormatyKoloruObrazOgólny {
    L8,
    L8a,
    B8,
    B8a,
    L16,
    L16a,
    B16,
    B16a,
    B32,
    B32a,
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OptFormatyKoloruObrazuTga {
    Szary8,
    HighColor16, //alpga 1 bit (on/off)
    TrueColor24,
    TrueColorA32,
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OptFormatyKoloruObrazuQoi {
    Color24,
    ColorA32,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OptRozdzielczościObrazów {
    R16,
    R32,
    R64,
    R128,
    R256,
    R512,
    R1k,
    R2k,
    R4k,
    R6k,
    R8k,
    R16k,
    Oryginalna,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum OptEfektZaszumiania {
    Tak(u8),
    Nie,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptUIWariantPodstrony {
    Pakowanie,
    Rozpakowanie,
    KonwersjaFoto,
    DaneDoŁączeniaZdjęćo,
    ObslugaDds,
    // Ustawienia,
    // Logi,
    Dev,
}