use std::fmt;
use std::fmt::Formatter;
use crate::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use crate::rozszerzenia::bdepth::BdepthAvif;
// use crate::rozszerzenia::bdepth::RozszerzeniaBdepth;
use crate::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForFfKompresja};
use crate::rozszerzenia::rozszerzenia::{ImgExt, ImgExtTag};

// impl fmt::Display for RozszerzeniaBdepth {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.krotka().fmt(f)
//     }
// }
// impl RozszerzeniaBdepth {
//     fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
//         match self {
//             Self::Luma8  => ("L8", "Luma 8", "8-bit Luminance"),
//             Self::Luma8Alpha => ("L8a", "Luma 8a", "8-bit Luminance + Alpha"),
//             Self::Rgb8 => ("R8", "Rgb 8", "8-bit Rgb"),
//             Self::Rgb8Alpha => ("R8a", "Rgb 8a", "8-bit Rgb + Alpha"),
//             Self::Rgb10 => ("R10", "Rgb 10", "10-bit Rgb"),
//             Self::Rgb10Alpha => ("R10a", "Rgb 10a", "10-bit Rgb + Alpha"),
//             Self::Luma16 => ("L16", "Luma 16", "16-bit Luminance"),
//             Self::Luma16Alpha => ("L16a", "Luma 16a", "16-bit Luminance + Alpha"),
//             Self::Rgb16 => ("R16", "Rgb 16", "16-bit Rgb"),
//             Self::Rgb16Alpha => ("R16a", "Rgb 16a", "16-bit Rgb + Alpha"),
//             Self::HighColor16 => ("HC16", "High Color 16", "16-bit Rgb + Alpha"),
//             Self::TrueColor24  => ("TC24", "TC 24", "24-bit Rgb"),
//             Self::TrueColorA32 => ("TC32", "True Color 32", "32-bit Rgb + Alpha"),
//             Self::Color24 => ("R24", "Color 24", "24-bit Rgb"),
//             Self::Color32 => ("R32", "Color 32", "32-bit Rgb + Alpha"),
//             Self::F32 => ("F32","Float 32", "32-bit Float"),
//             Self::F32Alpha => ("F32a", "Float 32", "32-bit Float + Alpha"),
//         }
//     }
//
//     // Te metody zwracają "obiekty-wyświetlacze"
//     pub fn krotka(&self) -> RozszerzeniaBdepthKrotkaDisplay<'_> { RozszerzeniaBdepthKrotkaDisplay(self) }
//     pub fn mid(&self) -> RozszerzeniaBdepthMidDisplay<'_> { RozszerzeniaBdepthMidDisplay(self) }
//     pub fn dluga(&self) -> RozszerzeniaBdepthDlugaDisplay<'_> { RozszerzeniaBdepthDlugaDisplay(self) }
// }
//
// // 2. Implementacja Display dla wersji KRÓTKIEJ
// pub struct RozszerzeniaBdepthKrotkaDisplay<'a>(&'a RozszerzeniaBdepth);
// impl fmt::Display for RozszerzeniaBdepthKrotkaDisplay<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Pobieramy krotkę i bierzemy pierwszy element (index 0)
//         write!(f, "{}", self.0.nazwy().0)
//     }
// }
// pub struct RozszerzeniaBdepthMidDisplay<'a>(&'a RozszerzeniaBdepth);
// impl fmt::Display for RozszerzeniaBdepthMidDisplay<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Pobieramy krotkę i bierzemy pierwszy element (index 0)
//         write!(f, "{}", self.0.nazwy().0)
//     }
// }
//
// // 3. Implementacja Display dla wersji DŁUGIEJ
// pub struct RozszerzeniaBdepthDlugaDisplay<'a>(&'a RozszerzeniaBdepth);
// impl fmt::Display for RozszerzeniaBdepthDlugaDisplay<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Pobieramy krotkę i bierzemy drugi element (index 1)
//         write!(f, "{}", self.0.nazwy().1)
//     }
// }



impl OptKompresjaPlikówPoziomKompresjiZstd {
    pub const WSIOKOMPRESJI: [Self; 3] = [Self::Brak, Self::Standard, Self::Duża];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Brak => "comp_none",
            Self::Standard => "comp_std",
            Self::Duża => "comp_max",
        }
    }
}

impl OptKompresjaPlikówFiltracjaPlików {
    pub const WSIOPLIKOW: [Self; 5] = [
        Self::Wszystkie,
        Self::Graficzne,
        Self::Audio,
        Self::Tekstowe,
        Self::Pdf,
    ];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Wszystkie => "filter_all",
            Self::Graficzne => "filter_graphic",
            Self::Audio => "filter_audio",
            Self::Tekstowe => "filter_text",
            Self::Pdf => "filter_pdf",
        }
    }
}

// impl fmt::Display for OptKompresjaPlikówPoziomKompresjiZstd {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             OptKompresjaPlikówPoziomKompresjiZstd::Brak => write!(f, "None"),
//             OptKompresjaPlikówPoziomKompresjiZstd::Standard => write!(f, "Standard"),
//             OptKompresjaPlikówPoziomKompresjiZstd::Duża => write!(f, "Big"),
//         }
//     }
// }
impl fmt::Display for ForAvifChroma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForAvifChroma::C444 => write!(f, "4:4:4"),
            ForAvifChroma::C422 => write!(f, "4:2:2"),
            ForAvifChroma::C420 => write!(f, "4:2:0"),
        }
    }
}



// impl fmt::Display for JpgSamplingFac {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             JpgSamplingFac::R444 => write!(f, "4:4:4 full"),
//             JpgSamplingFac::R440 => write!(f, "4:4:0"),
//             JpgSamplingFac::R441 => write!(f, "4:4:1"),
//             JpgSamplingFac::R422 => write!(f, "4:2:2"),
//             JpgSamplingFac::R420 => write!(f, "4:2:0 internet"),
//             JpgSamplingFac::R421 => write!(f, "4:2:1"),
//             JpgSamplingFac::R411 => write!(f, "4:1:1"),
//             JpgSamplingFac::R410 => write!(f, "4:1:0"),
//         }
//     }
// }

// impl fmt::Display for JpgQuant {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             JpgQuant::Default => write!(f, "Default"),
//             JpgQuant::Flat => write!(f, "Flat"),
//             JpgQuant::CustomMsSsim => write!(f, "MS-SSIM"),
//             JpgQuant::CustomPsnrHvs => write!(f, "PSNR-HVS"),
//             JpgQuant::ImageMagick => write!(f, "ImageMagick table"),
//             JpgQuant::KleinSilversteinCarney => write!(f, "JPEG-DCT"),
//             JpgQuant::DentalXRays => write!(f, "X-Rays"),
//             JpgQuant::VisualDetectionModel => write!(f, "DCT coefficient"),
//             JpgQuant::ImprovedDetectionModel => write!(f, "Improved DCT coefficient"),
//         }
//     }
// }

// impl fmt::Display for OptRozszerzeniaPlikówZdjęciowychZnacznik{
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Jpg => ("Jpg", "Joint Photographic Experts Group"),
//             Self::Avif => ("Avif", "AV1 Image File Format"),
//             Self::Webp => ("Webp", "Web Photograph"),
//             Self::Png => ("PNG", "Portable Network Graphics"),
//             Self::Tga => ("TGA", "Truevision TGA"),
//             Self::Ff => ("FF", "Farbfeld"),
//             Self::Qoi => ("QOI", "Quite OK Image Format"),
//         }
//     }
//     pub fn krotka(&self) -> OptRozszerzeniaPlikówZdjęciowychZnacznikKrotkaDisplay<'_> { OptRozszerzeniaPlikówZdjęciowychZnacznikKrotkaDisplay(self) }
//     pub fn mid(&self) -> OptRozszerzeniaPlikówZdjęciowychZnacznikMidDisplay<'_> { OptRozszerzeniaPlikówZdjęciowychZnacznikMidDisplay(self) }
// }

impl ImgExtTag {
    pub fn krótka(&self) -> &'static str {
        match self {
            Self::Jpg => "Jpg",
            Self::Avif => "Avif",
            Self::Webp => "Webp",
            Self::Png => "PNG",
            Self::Tga => "TGA",
            Self::Ff => "FF",
            Self::Qoi => "QOI",
        }
    }
    pub fn długa(&self) -> &'static str {
        match self {
            Self::Jpg => "Joint Photographic Experts Group",
            Self::Avif => "AV1 Image File Format",
            Self::Webp => "Web Photograph",
            Self::Png => "Portable Network Graphics",
            Self::Tga => "Truevision TGA",
            Self::Ff => "Farbfeld",
            Self::Qoi => "Quite OK Image Format",
        }
    }

}
impl ImgExt {
    fn nazwy(&self) -> (&'static str,  &'static str) {
        match self {
            Self::Jpg { .. } => ("Jpg", "Joint Photographic Experts Group"),
            Self::Avif { .. } => ("Avif", "AV1 Image File Format"),
            Self::Webp { .. } => ( "Webp", "Web Photograph"),
            Self::Png { .. } => ("PNG", "Portable Network Graphics"),
            Self::Tga { .. } => ("TGA", "Truevision TGA"),
            Self::Ff { .. } => ("FF", "Farbfeld"),
            Self::Qoi { .. } => ("QOI", "Quite OK Image Format"),
        }
    }

    // Te metody zwracają "obiekty-wyświetlacze"
    pub fn krotka(&self) -> RozszerzeniaKrotkaDisplay<'_> { RozszerzeniaKrotkaDisplay(self) }
    pub fn mid(&self) -> RozszerzeniaMidDisplay<'_> { RozszerzeniaMidDisplay(self) }
}


pub struct RozszerzeniaKrotkaDisplay<'a>(&'a ImgExt);

impl fmt::Display for RozszerzeniaKrotkaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}

pub struct RozszerzeniaMidDisplay<'a>(&'a ImgExt);

impl fmt::Display for RozszerzeniaMidDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}

impl ForFfKompresja {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Zstd(_) => "btn_id_batch_ff_zstd",
            Self::Xz(_) => "btn_id_batch_ff_xz",
            Self::Bzip2(_) => "btn_id_batch_ff_bzip2",
            Self::Brak => "btn_id_batch_ff_brak",
        }
    }

}

impl ImgExtTag {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Jpg => "btn_id_batch_jpg_ext",
            Self::Png => "btn_id_batch_png_ext",
            Self::Webp => "btn_id_batch_webp_ext",
            Self::Tga => "btn_id_batch_tga_ext",
            Self::Ff => "btn_id_batch_ff_ext",
            Self::Qoi => "btn_id_batch_qoi_ext",
            Self::Avif => "btn_id_batch_avif_ext",
        }
    }
    pub fn dds_id(&self) -> &'static str {
        match self {
            Self::Jpg => "btn_id_dds_jpg_ext",
            Self::Png => "btn_id_dds_png_ext",
            Self::Webp => "btn_id_dds_webp_ext",
            Self::Tga => "btn_id_dds_tga_ext",
            Self::Ff => "btn_id_dds_ff_ext",
            Self::Qoi => "btn_id_dds_qoi_ext",
            Self::Avif => "btn_id_dds_avif_ext",
        }
    }
    pub fn laczenie_id(&self) -> &'static str {
        match self {
            Self::Jpg => "btn_id_laczenie_jpg_ext",
            Self::Png => "btn_id_laczenie_png_ext",
            Self::Webp => "btn_id_laczenie_webp_ext",
            Self::Tga => "btn_id_laczenie_tga_ext",
            Self::Ff => "btn_id_laczenie_ff_ext",
            Self::Qoi => "btn_id_laczenie_qoi_ext",
            Self::Avif => "btn_id_laczenie_avif_ext",
        }
    }
}