use crate::dane_do_przetwarzania::{DaneBinPak, DaneDdsPak, DaneDdsUnpak, DaneKonw, DaneMerge};
use crate::inne_ui::{ActProces, DropdownType};
use crate::opcje::{OptInterpolacja, OptIstniejePlik, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use crate::rozszerzenia::ext::{ImgExt, ImgExtSingle};
use crate::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForDds, ForDdsKompresja, ForExrKompresja, ForFfKompresja};
use std::any::Any;
use std::fmt::Debug;
use strum::IntoEnumIterator;


pub trait ElementyDropdown: Debug + Send + Sync{

    fn as_any(&self) -> &dyn Any;

}
impl dyn ElementyDropdown + Send + Sync {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
impl ElementyDropdown for ForAvifChroma {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ElementyDropdown for ForAvifKompresja {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ElementyDropdown for ForJpgSamplingFac {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ElementyDropdown for ForJpgQuant {
    fn as_any(&self) -> &dyn Any {self}
}
impl ElementyDropdown for ForExrKompresja{
    fn as_any(&self) -> &dyn Any {self}
}
impl ElementyDropdown for OptIstniejePlik{
    fn as_any(&self) -> &dyn Any {self}
}

pub trait DaneDropdown<T: ElementyDropdown + IntoEnumIterator> {
    type Opcja: AsRef<[T]>;

    fn get_opcje(&self) -> Vec<T> {
        T::iter().collect::<Vec<_>>()
    }
    // fn get_opcje(&self) -> Self::Opcja;
    fn get_data(&self) -> Option<T>;
    fn get_dropdown_type() -> DropdownType;
    fn get_proces_name() -> ActProces;

}

impl OptInterpolacja {

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
impl ElementyDropdown for OptInterpolacja{
    fn as_any(&self) -> &dyn Any { self }
}
impl ElementyDropdown for OptKompresjaPlikówFiltracjaPlików{
    fn as_any(&self) -> &dyn Any { self }
}
impl ElementyDropdown for ForFfKompresja {
    fn as_any(&self) -> &dyn Any { self }
}
impl ElementyDropdown for OptKompresjaPlikówPoziomKompresjiZstd {
    fn as_any(&self) -> &dyn Any { self }
}
impl ElementyDropdown for ForDdsKompresja {
    fn as_any(&self) -> &dyn Any { self }
}
impl ElementyDropdown for ForDds {
    fn as_any(&self) -> &dyn Any { self }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgSamplingFac> for DaneKonw {

    type Opcja = Vec<ForJpgSamplingFac>;

    fn get_data(&self) -> Option<ForJpgSamplingFac> {
        self.rozszerzenia
            .iter()
            .find_map(|f| {
                if let ImgExt::Jpg { sampling, .. } = f {
                    Some(*sampling)
                } else {
                    None
                }
            })

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
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgSamplingFac> for DaneDdsUnpak {

    type Opcja = Vec<ForJpgSamplingFac>;

    fn get_data(&self) -> Option<ForJpgSamplingFac> {
        
        if let ImgExt::Jpg { ref sampling, .. } = self.rozszerzenie {
            Some(*sampling)
        } else {
            None
        }
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsJpgSample
    }

    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForDdsKompresja> for DaneDdsPak {

    type Opcja = Vec<ForDdsKompresja>;

    fn get_data(&self) -> Option<ForDdsKompresja> {
        Some(self.kompresja)
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsPakComp
    }

    fn get_proces_name() -> ActProces {ActProces::DdsPak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForDds> for DaneDdsPak {

    type Opcja = Vec<ForDds>;

    fn get_data(&self) -> Option<ForDds> {
        Some(self.format)
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsPakFormat
    }

    fn get_proces_name() -> ActProces {ActProces::DdsPak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgSamplingFac> for DaneMerge {

    type Opcja = Vec<ForJpgSamplingFac>;

    fn get_data(&self) -> Option<ForJpgSamplingFac> {

        if let ImgExtSingle::Jpg { sampling, .. } = self.rozszerzenie {
            Some(sampling)
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeJpgSample
    }

    fn get_proces_name() -> ActProces {ActProces::Merge }
}

// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgQuant> for DaneKonw {

    type Opcja = Vec<ForJpgQuant>;

    fn get_data(&self) -> Option<ForJpgQuant> {
        self.rozszerzenia
            .iter()
            .find_map(|f| {
                if let ImgExt::Jpg { quant, .. } = f {
                    Some(*quant)
                } else {
                    None
                }
            })

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaJpgQuant
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgQuant> for DaneDdsUnpak {

    type Opcja = Vec<ForJpgQuant>;

    fn get_data(&self) -> Option<ForJpgQuant> {

        if let ImgExt::Jpg { ref quant, .. } = self.rozszerzenie {
            Some(*quant)
        } else {
            None
        }
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsJpgQuant
    }

    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForJpgQuant> for DaneMerge {

    type Opcja = Vec<ForJpgQuant>;

    fn get_data(&self) -> Option<ForJpgQuant> {

        if let ImgExtSingle::Jpg { quant, .. } = self.rozszerzenie {
            Some(quant)
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeJpgQuant
    }

    fn get_proces_name() -> ActProces {ActProces::Merge }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifChroma> for DaneKonw {

    type Opcja = Vec<ForAvifChroma>;

    fn get_data(&self) -> Option<ForAvifChroma> {
        self.rozszerzenia
            .iter()
            .find_map(|f| {
                if let ImgExt::Avif { chroma, .. } = f {
                    Some(chroma.clone())
                } else {
                    None
                }
            })

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaAvifChroma
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifChroma> for DaneMerge {

    type Opcja = Vec<ForAvifChroma>;

    fn get_data(&self) -> Option<ForAvifChroma> {

        if let ImgExtSingle::Avif { ref chroma, .. } = self.rozszerzenie {
            Some(chroma.clone())
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeAvifChroma
    }

    fn get_proces_name() -> ActProces {ActProces::Merge }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifChroma> for DaneDdsUnpak {

    type Opcja = Vec<ForAvifChroma>;

    fn get_data(&self) -> Option<ForAvifChroma> {

        if let ImgExt::Avif { ref chroma, .. } = self.rozszerzenie {
            Some(chroma.clone())
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsAvifChroma
    }

    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifKompresja> for DaneKonw {

    type Opcja = Vec<ForAvifKompresja>;

    fn get_data(&self) -> Option<ForAvifKompresja> {
        self.rozszerzenia
            .iter()
            .find_map(|f| {
                if let ImgExt::Avif { metoda_kompresji, .. } = f {
                    Some(metoda_kompresji.clone())
                } else {
                    None
                }
            })

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaAvifKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifKompresja> for DaneDdsUnpak {

    type Opcja = Vec<ForAvifKompresja>;

    fn get_data(&self) -> Option<ForAvifKompresja> {
        
                if let ImgExt::Avif { ref metoda_kompresji, .. } = self.rozszerzenie {
                    Some(metoda_kompresji.clone())
                } else {
                    None
                }
    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsAvifKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForAvifKompresja> for DaneMerge {

    type Opcja = Vec<ForAvifKompresja>;

    fn get_data(&self) -> Option<ForAvifKompresja> {

        if let ImgExtSingle::Avif { ref metoda_kompresji, .. } = self.rozszerzenie {
            Some(metoda_kompresji.clone())
        } else {
            None
        }

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeAvifKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Merge }
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

        self.rozszerzenia
            .iter()
            .find_map(|r|
                {
                    if let ImgExt::Ff { metoda_kompresji } = r {
                        Some(*metoda_kompresji)
                    } else {
                        None
                    }
                }
            )

    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaKompresjaFf
    }
    fn get_proces_name() -> ActProces {ActProces::Konw }

}
impl DaneDropdown<ForFfKompresja> for DaneDdsUnpak {

    type Opcja = Vec<ForFfKompresja>;

    fn get_data(&self) -> Option<ForFfKompresja> {

        if let ImgExt::Ff { ref metoda_kompresji } = self.rozszerzenie {
            Some(*metoda_kompresji)
        } else {
            None
        }

    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsKompresjaFf
    }
    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }

}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForFfKompresja> for DaneMerge {

    type Opcja = Vec<ForFfKompresja>;

    fn get_data(&self) -> Option<ForFfKompresja> {
        
        if let ImgExtSingle::Ff { ref metoda_kompresji } = self.rozszerzenie {
            Some(*metoda_kompresji)
        } else {
            None
        }
        
    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeKompresjaFf
    }
    fn get_proces_name() -> ActProces {ActProces::Merge }

}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<OptKompresjaPlikówFiltracjaPlików> for DaneBinPak {

    type Opcja = Vec<OptKompresjaPlikówFiltracjaPlików>;

    fn get_data(&self) -> Option<OptKompresjaPlikówFiltracjaPlików> {

        Some(self.filtracja)

    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::BinFilter
    }
    fn get_proces_name() -> ActProces {ActProces::BinPak }

}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<OptKompresjaPlikówPoziomKompresjiZstd> for DaneBinPak {

    type Opcja = Vec<OptKompresjaPlikówPoziomKompresjiZstd>;

    fn get_data(&self) -> Option<OptKompresjaPlikówPoziomKompresjiZstd> {

        Some(self.kompresja)

    }
    fn get_dropdown_type() -> DropdownType {
        DropdownType::BinKompresja
    }
    fn get_proces_name() -> ActProces {ActProces::BinPak }


}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForExrKompresja> for DaneKonw {

    type Opcja = Vec<ForExrKompresja>;

    fn get_data(&self) -> Option<ForExrKompresja> {
        self.rozszerzenia
            .iter()
            .find_map(|f| {
                if let ImgExt::Exr { kompresja, .. } = f {
                    Some(*kompresja)
                } else {
                    None
                }
            })

    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::KonwersjaExrKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Konw }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForExrKompresja> for DaneMerge {

    type Opcja = Vec<ForExrKompresja>;

    fn get_data(&self) -> Option<ForExrKompresja> {
        if let ImgExtSingle::Exr {kompresja, .. } = self.rozszerzenie{
            Some(kompresja)
        } else {None}


    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::MergeExrKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::Merge }
}
// -------------------------------------------------------------------------------------------------
impl DaneDropdown<ForExrKompresja> for DaneDdsUnpak {

    type Opcja = Vec<ForExrKompresja>;

    fn get_data(&self) -> Option<ForExrKompresja> {
        if let ImgExt::Exr {kompresja, .. } = self.rozszerzenie{
            Some(kompresja)
        } else {None}


    }

    fn get_dropdown_type() -> DropdownType {
        DropdownType::DdsExrKompresja
    }

    fn get_proces_name() -> ActProces {ActProces::DdsUnpak }
}
// -------------------------------------------------------------------------------------------------


