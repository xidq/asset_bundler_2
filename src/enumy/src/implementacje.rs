use crate::opcje::{AvifChroma, AvifMetodaKompresji, JpgQuant, JpgSamplingFac, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuTga, OptInterpolacja, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd, OptMetodaKompresjiZdjecia};
use std::fmt;

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

impl fmt::Display for OptMetodaKompresjiZdjecia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptMetodaKompresjiZdjecia::Zstd(_) => write!(f, "Zstd"),
            OptMetodaKompresjiZdjecia::Bzip2(_) => write!(f, "Bzip2"),
            OptMetodaKompresjiZdjecia::Xz(_) => write!(f, "Xz"),
            OptMetodaKompresjiZdjecia::Brak => write!(f, "Brak"),
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
impl fmt::Display for AvifChroma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvifChroma::C444 => write!(f, "4:4:4"),
            AvifChroma::C422 => write!(f, "4:2:2"),
            AvifChroma::C420 => write!(f, "4:2:0"),
        }
    }
}
impl fmt::Display for AvifMetodaKompresji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvifMetodaKompresji::Undefined => write!(f, "Undefined"),
            AvifMetodaKompresji::Hevc => write!(f, "Hevc"),
            AvifMetodaKompresji::Avc => write!(f, "Avc"),
            AvifMetodaKompresji::Jpeg => write!(f, "Jpeg"),
            AvifMetodaKompresji::Av1 => write!(f, "Av1"),
            AvifMetodaKompresji::Vvc => write!(f, "Vvc"),
            AvifMetodaKompresji::Evc => write!(f, "Evc"),
            AvifMetodaKompresji::Jpeg2000 => write!(f, "Jpeg2000"),
            AvifMetodaKompresji::Uncompressed => write!(f, "Uncompressed"),
            AvifMetodaKompresji::Mask => write!(f, "Mask"),
            AvifMetodaKompresji::HtJ2k => write!(f, "HtJ2k"),
        }
    }
}

impl OptFormatyKoloruObrazOgólny {
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::L8  => ("L8", "L 8bit", "8-bit Luminance"),
            Self::L8a => ("L8a", "LA 8bit", "8-bit Luminance + Alpha"),
            Self::B8  => ("B8", "RGB 8bit", "8-bit Rgb"),
            Self::B8a => ("B8a", "RGBA 8bit","8-bit Rgb + Alpha"),
            Self::L16  => ("L16", "L 16bit", "16-bit Luminance"),
            Self::L16a => ("L16a", "LA 16bit","16-bit Luminance + Alpha"),
            Self::B16  => ("B16",  "RGB 16bit","16-bit Rgb"),
            Self::B16a => ("B16a", "RGBA 16bit","16-bit Rgb + Alpha"),
            Self::B32 => ("F32", "RGB 32bit","32-bit Float (High Dynamic)"),
            Self::B32a => ("F32a", "RGBA 32bit","32-bit Float (High Dynamic) + Alpha"),
        }
    }

    // Te metody zwracają "obiekty-wyświetlacze"
    pub fn krotka(&self) -> OptFormatyKoloruObrazOgólnyKrotkaDisplay<'_> { OptFormatyKoloruObrazOgólnyKrotkaDisplay(self) }
    pub fn mid(&self) -> OptFormatyKoloruObrazOgólnyMidDisplay<'_> { OptFormatyKoloruObrazOgólnyMidDisplay(self) }
    pub fn dluga(&self) -> OptFormatyKoloruObrazOgólnyDlugaDisplay<'_> { OptFormatyKoloruObrazOgólnyDlugaDisplay(self) }
}

// 2. Implementacja Display dla wersji KRÓTKIEJ
pub struct OptFormatyKoloruObrazOgólnyKrotkaDisplay<'a>(&'a OptFormatyKoloruObrazOgólny);
impl fmt::Display for OptFormatyKoloruObrazOgólnyKrotkaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}
pub struct OptFormatyKoloruObrazOgólnyMidDisplay<'a>(&'a OptFormatyKoloruObrazOgólny);
impl fmt::Display for OptFormatyKoloruObrazOgólnyMidDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}

// 3. Implementacja Display dla wersji DŁUGIEJ
pub struct OptFormatyKoloruObrazOgólnyDlugaDisplay<'a>(&'a OptFormatyKoloruObrazOgólny);
impl fmt::Display for OptFormatyKoloruObrazOgólnyDlugaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy drugi element (index 1)
        write!(f, "{}", self.0.nazwy().1)
    }
}

// 4. Opcjonalnie: główny Display dla enuma (domyślnie krótka)
impl fmt::Display for OptFormatyKoloruObrazOgólny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.krotka().fmt(f)
    }
}
impl OptFormatyKoloruObrazuTga {
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Szary8  => ("8", "g8", "8-bit Luminance"),
            Self::HighColor16 => ("16", "HC 16", "16-bit Rgb + Alpha"),
            Self::TrueColor24  => ("24", "TC 24", "24-bit Rgb"),
            Self::TrueColorA32 => ("32", "TC32","32-bit Rgb + Alpha"),
        }
    }

    // Te metody zwracają "obiekty-wyświetlacze"
    pub fn krotka(&self) -> OptFormatyKoloruObrazuTgaKrotkaDisplay<'_> { OptFormatyKoloruObrazuTgaKrotkaDisplay(self) }
    pub fn mid(&self) -> OptFormatyKoloruObrazuTgaMidDisplay<'_> { OptFormatyKoloruObrazuTgaMidDisplay(self) }
    pub fn dluga(&self) -> OptFormatyKoloruObrazuTgaDlugaDisplay<'_> { OptFormatyKoloruObrazuTgaDlugaDisplay(self) }
}

// 2. Implementacja Display dla wersji KRÓTKIEJ
pub struct OptFormatyKoloruObrazuTgaKrotkaDisplay<'a>(&'a OptFormatyKoloruObrazuTga);
impl fmt::Display for OptFormatyKoloruObrazuTgaKrotkaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}
pub struct OptFormatyKoloruObrazuTgaMidDisplay<'a>(&'a OptFormatyKoloruObrazuTga);
impl fmt::Display for OptFormatyKoloruObrazuTgaMidDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}

// 3. Implementacja Display dla wersji DŁUGIEJ
pub struct OptFormatyKoloruObrazuTgaDlugaDisplay<'a>(&'a OptFormatyKoloruObrazuTga);
impl fmt::Display for OptFormatyKoloruObrazuTgaDlugaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy drugi element (index 1)
        write!(f, "{}", self.0.nazwy().1)
    }
}

// 4. Opcjonalnie: główny Display dla enuma (domyślnie krótka)
impl fmt::Display for OptFormatyKoloruObrazuTga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.krotka().fmt(f)
    }
}
impl OptFormatyKoloruObrazuAvif {
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::B8  => ("B8", "RGB 8bit", "8-bit Rgb"),
            Self::B8a => ("B8a", "RGBA 8bit","8-bit Rgb + Alpha"),
            Self::B10  => ("B10",  "RGB 10bit","10-bit Rgb"),
            Self::B10a => ("B10a", "RGBA 10bit","10-bit Rgb + Alpha"),
        }
    }

    // Te metody zwracają "obiekty-wyświetlacze"
    pub fn krotka(&self) -> OptFormatyKoloruObrazuAvifKrotkaDisplay<'_> { OptFormatyKoloruObrazuAvifKrotkaDisplay(self) }
    pub fn mid(&self) -> OptFormatyKoloruObrazuAvifMidDisplay<'_> { OptFormatyKoloruObrazuAvifMidDisplay(self) }
    pub fn dluga(&self) -> OptFormatyKoloruObrazuAvifDlugaDisplay<'_> { OptFormatyKoloruObrazuAvifDlugaDisplay(self) }
}

// 2. Implementacja Display dla wersji KRÓTKIEJ
pub struct OptFormatyKoloruObrazuAvifKrotkaDisplay<'a>(&'a OptFormatyKoloruObrazuAvif);
impl fmt::Display for OptFormatyKoloruObrazuAvifKrotkaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}
pub struct OptFormatyKoloruObrazuAvifMidDisplay<'a>(&'a OptFormatyKoloruObrazuAvif);
impl fmt::Display for OptFormatyKoloruObrazuAvifMidDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy pierwszy element (index 0)
        write!(f, "{}", self.0.nazwy().0)
    }
}

// 3. Implementacja Display dla wersji DŁUGIEJ
pub struct OptFormatyKoloruObrazuAvifDlugaDisplay<'a>(&'a OptFormatyKoloruObrazuAvif);
impl fmt::Display for OptFormatyKoloruObrazuAvifDlugaDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pobieramy krotkę i bierzemy drugi element (index 1)
        write!(f, "{}", self.0.nazwy().1)
    }
}

// 4. Opcjonalnie: główny Display dla enuma (domyślnie krótka)
impl fmt::Display for OptFormatyKoloruObrazuAvif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.krotka().fmt(f)
    }
}

impl fmt::Display for JpgSamplingFac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JpgSamplingFac::R444 => write!(f, "4:4:4 full"),
            JpgSamplingFac::R440 => write!(f, "4:4:0"),
            JpgSamplingFac::R441 => write!(f, "4:4:1"),
            JpgSamplingFac::R422 => write!(f, "4:2:2"),
            JpgSamplingFac::R420 => write!(f, "4:2:0 internet"),
            JpgSamplingFac::R421 => write!(f, "4:2:1"),
            JpgSamplingFac::R411 => write!(f, "4:1:1"),
            JpgSamplingFac::R410 => write!(f, "4:1:0"),
        }
    }
}
impl fmt::Display for JpgQuant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JpgQuant::Default => write!(f, "Default"),
            JpgQuant::Flat => write!(f, "Flat"),
            JpgQuant::CustomMsSsim => write!(f, "MS-SSIM"),
            JpgQuant::CustomPsnrHvs => write!(f, "PSNR-HVS"),
            JpgQuant::ImageMagick => write!(f, "ImageMagick table"),
            JpgQuant::KleinSilversteinCarney => write!(f, "JPEG-DCT"),
            JpgQuant::DentalXRays => write!(f, "X-Rays"),
            JpgQuant::VisualDetectionModel => write!(f, "DCT coefficient"),
            JpgQuant::ImprovedDetectionModel => write!(f, "Improved DCT coefficient"),
        }
    }
}
impl OptInterpolacja {
    pub const WSIOINTERPOLACJI: [Self; 5] = [
        Self::Nearest,
        Self::Triangle,
        Self::CatmullRom,
        Self::Gaussian,
        Self::Lanczos3,
    ];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Nearest => "OptInterpolacja_nearest",
            Self::Triangle => "OptInterpolacja_triangle",
            Self::CatmullRom => "OptInterpolacja_catmull",
            Self::Gaussian => "OptInterpolacja_gaussian",
            Self::Lanczos3 => "OptInterpolacja_lanczos",
        }
    }
}
#[allow(dead_code)]
impl OptKompresjaPlikówFiltracjaPlików {
    pub const ALL: [Self; 5] = [
        Self::Wszystkie,
        Self::Graficzne,
        Self::Audio,
        Self::Tekstowe,
        Self::Pdf,
    ];
}
impl fmt::Display for OptKompresjaPlikówFiltracjaPlików {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Możesz tu użyć metody .t() jeśli chcesz tłumaczyć nazwy filtrów

        write!(f, "{:?}", self)
        // f.write_str(match self {
        //     Self::Wszystkie => "Apple",
        //     Self::Graficzne => "Orange",
        //     Self::Audio => "Strawberry",
        //     Self::Tekstowe => "Tomato",
        //     Self::Pdf => "PDF",
        // })
    }
}

impl fmt::Display for OptKompresjaPlikówPoziomKompresjiZstd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
