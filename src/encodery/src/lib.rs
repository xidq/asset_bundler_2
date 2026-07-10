use std::any::Any;
use std::path::PathBuf;
use image::DynamicImage;
use enumy::dane_do_przetwarzania::DaneDoObrbki;
use enumy::implementacje::DaneDropdown;
use enumy::inne_ui::{ActProces, DropdownType, WskaznikSzumu};
use enumy::opcje::{OptInterpolacja, OptIstniejePlik};
use enumy::rozszerzenia::kolor::{ColorProfilePhoto, ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszenienia_zdjec::{FormatyWyjściowe, ImgExtAvif, ImgExtExr, ImgExtFf, ImgExtJpg};

pub mod halper;
// pub mod wczytaj_foto;
pub mod check;
pub mod zapisywanie;
pub mod wczytywanie;
mod transform;
// mod transform;

pub struct DaneDoWczytywania{
    pub dane: DynamicImage,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneKonw {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub opcje_rozdzielczości: Vec<Rozdzielczości>,
    pub noising: WskaznikSzumu,
    // pub rozszerzenia: Vec<ImgExt>,
    // pub tag:Vec<ImgExtTag>,
    pub rozszerzenia: FormatyWyjściowe,
    pub inter: OptInterpolacja,
    pub alfa_rgb: (u16, u16, u16),
    pub exif: bool,
    pub istniejace_pliki: OptIstniejePlik,
}

impl Default for DaneKonw {
    fn default() -> Self {
        DaneKonw {
            ścieżka_wejściowa: PathBuf::new(),
            ścieżka_wyjściowa: PathBuf::new(),
            opcje_rozdzielczości: Vec::from([Rozdzielczości::Oryginalna]),
            noising: WskaznikSzumu::NoNoise,
            // rozszerzenia: Vec::from([ Default::default() ]),
            // tag: Vec::from([ ImgExtTag::Jpg ]),
            rozszerzenia: FormatyWyjściowe {
                jpg: Some(ImgExtJpg::default()),
                png: None,
                webp: None,
                tga: None,
                ff: None,
                qoi: None,
                avif: None,
                exr: None,
            },
            inter: OptInterpolacja::Lanczos3,
            alfa_rgb: (0, 0, 0),
            exif: false,
            istniejace_pliki: OptIstniejePlik::Zamień,
        }
    }
}

impl DaneDoObrbki for DaneKonw {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    // fn tag_master(&self) -> Vec<ImgExtTag> { self.tag.clone() }
}

// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgSamplingFac> for DaneKonw {

    type Opcja = Vec<ForJpgSamplingFac>;

    fn get_data(&self) -> Option<ForJpgSamplingFac> {
        if let Some(ImgExtJpg { sampling, .. }) = self.rozszerzenia.jpg {
            Some(sampling)
        } else {
            None
        }
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaJpgSample
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<OptIstniejePlik> for DaneKonw {

    type Opcja = Vec<OptIstniejePlik>;

    fn get_data(&self) -> Option<OptIstniejePlik> {
        Some(self.istniejace_pliki)
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaFileTreatment
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}

impl DaneDropdown<ForJpgQuant> for DaneKonw {

    type Opcja = Vec<ForJpgQuant>;

    fn get_data(&self) -> Option<ForJpgQuant> {

        if let Some(ImgExtJpg{ quant, .. }) = self.rozszerzenia.jpg {
            Some(quant)
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaJpgQuant
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifChroma> for DaneKonw {

    type Opcja = Vec<ForAvifChroma>;

    fn get_data(&self) -> Option<ForAvifChroma> {

        if let Some(ImgExtAvif { ref chroma, .. }) = self.rozszerzenia.avif {
            Some(chroma.clone())
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaAvifChroma
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifKompresja> for DaneKonw {

    type Opcja = Vec<ForAvifKompresja>;

    fn get_data(&self) -> Option<ForAvifKompresja> {

        if let Some(ImgExtAvif { ref metoda_kompresji, .. }) = self.rozszerzenia.avif {
            Some(metoda_kompresji.clone())
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaAvifKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<OptInterpolacja> for DaneKonw {

    type Opcja = Vec<OptInterpolacja>;

    fn get_data(&self) -> Option<OptInterpolacja> {
        Some(self.inter)
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaInterpolacja
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForFfKompresja> for DaneKonw {

    type Opcja = Vec<ForFfKompresja>;

    fn get_data(&self) -> Option<ForFfKompresja> {

        if let Some(ImgExtFf { metoda_kompresji }) = self.rozszerzenia.ff {
            Some(metoda_kompresji)
        } else {
            None
        }

    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaKompresjaFf
    }
    fn get_proces_name() -> ActProces {ActProces::Konw }

}
impl DaneDropdown<ForExrKompresja> for DaneKonw {

    type Opcja = Vec<ForExrKompresja>;

    fn get_data(&self) -> Option<ForExrKompresja> {

        if let Some(ImgExtExr { kompresja, .. }) = self.rozszerzenia.exr {
            Some(kompresja)
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaExrKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------