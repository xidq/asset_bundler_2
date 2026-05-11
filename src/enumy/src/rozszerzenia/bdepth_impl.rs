use std::any::Any;
use std::fmt;
use std::fmt::Display;
use strum::EnumMessage;
use crate::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp, TrybLączenia};
use crate::rozszerzenia::rozszerzenia::ImgExtTag;

pub trait BitDepth: std::fmt::Debug + Any + Send + Sync{
    fn label_min(&self) -> &'static str;
    fn label_max(&self) -> &'static str;
    fn tryb_laczenia(&self) -> TrybLączenia;
    fn jako_any(&self) -> &dyn Any;
    fn jest_rowny(&self, inny: &dyn Any) -> bool;
    fn format(&self) -> ImgExtTag;
}




impl BdepthJpg {
    pub const BDEPTHJPG: [Self; 2] = [
        Self::Luma8,
        Self::Rgb8
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Luma8  => ("L8", "Luma 8", "8-bit Luminance"),
            Self::Rgb8 => ("R8", "Rgb 8", "8-bit Rgb"),
        }
    }

}
impl BdepthPng {
    pub const BDEPTHPNG: [Self; 8] = [
        Self::Luma8  ,
        Self::Luma8Alpha ,
        Self::Rgb8 ,
        Self::Rgb8Alpha ,
        Self::Luma16 ,
        Self::Luma16Alpha ,
        Self::Rgb16 ,
        Self::Rgb16Alpha ,
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Luma8  => ("L8", "Luma 8", "8-bit Luminance"),
            Self::Luma8Alpha => ("L8a", "Luma 8a", "8-bit Luminance + Alpha"),
            Self::Rgb8 => ("R8", "Rgb 8", "8-bit Rgb"),
            Self::Rgb8Alpha => ("R8a", "Rgb 8a", "8-bit Rgb + Alpha"),
            Self::Luma16 => ("L16", "Luma 16", "16-bit Luminance"),
            Self::Luma16Alpha => ("L16a", "Luma 16a", "16-bit Luminance + Alpha"),
            Self::Rgb16 => ("R16", "Rgb 16", "16-bit Rgb"),
            Self::Rgb16Alpha => ("R16a", "Rgb 16a", "16-bit Rgb + Alpha"),
            // Self::F32 => ("F32","Float 32", "32-bit Float"),
            // Self::F32Alpha => ("F32a", "Float 32", "32-bit Float + Alpha"),
        }
    }

}
impl BdepthWebp {
    pub const BDEPTHWEBP: [Self; 2] = [
        Self::Rgb8,
        Self::Rgb8Alpha,
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Rgb8 => ("R8", "Rgb 8", "8-bit Rgb"),
            Self::Rgb8Alpha => ("R8a", "Rgb 8a", "8-bit Rgb + Alpha"),
        }
    }
}

impl BdepthAvif {
    pub const BDEPTHAVIF: [Self; 4] = [
        Self::Rgb8 ,
        Self::Rgb8Alpha ,
        Self::Rgb10 ,
        Self::Rgb10Alpha ,
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Rgb8 => ("R8", "Rgb 8", "8-bit Rgb"),
            Self::Rgb8Alpha => ("R8a", "Rgb 8a", "8-bit Rgb + Alpha"),
            Self::Rgb10 => ("R10", "Rgb 10", "10-bit Rgb"),
            Self::Rgb10Alpha => ("R10a", "Rgb 10a", "10-bit Rgb + Alpha"),
        }
    }

}
impl BdepthTga {
    pub const BDEPTHTGA: [Self; 4] = [
        Self::Luma8 ,
        Self::HighColor16 ,
        Self::TrueColor24 ,
        Self::TrueColorA32 ,
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Luma8  => ("L8", "Luma 8", "8-bit Luminance"),
            Self::HighColor16 => ("HC16", "High Color 16", "16-bit Rgb + Alpha"),
            Self::TrueColor24  => ("TC24", "TC 24", "24-bit Rgb"),
            Self::TrueColorA32 => ("TC32", "True Color 32", "32-bit Rgb + Alpha"),
        }
    }

}

impl BdepthQoi {
    pub const BDEPTHQOI: [Self; 2] = [
        Self::Color24 ,
        Self::Color32 ,
    ];
    fn nazwy(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Color24  => ("TC24", "TC 24", "24-bit Rgb"),
            Self::Color32 => ("TC32", "True Color 32", "32-bit Rgb + Alpha"),
        }
    }

}






impl BitDepth for BdepthJpg {

    fn format(&self) -> ImgExtTag { ImgExtTag::Jpg}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Jpg")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Jpg")}
    fn tryb_laczenia(&self) -> TrybLączenia {
        match self {
            Self::Luma8 => TrybLączenia::Luma8,
            Self::Rgb8 => TrybLączenia::Rgb8,
        }
    }
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}
impl BitDepth for BdepthPng {
    fn format(&self) -> ImgExtTag { ImgExtTag::Png}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Png")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Png")}
    fn tryb_laczenia(&self) -> TrybLączenia {
    match self {
        Self::Luma8 => TrybLączenia::Luma8,
        Self::Rgb8 => TrybLączenia::Rgb8,
        Self::Luma8Alpha => TrybLączenia::Luma8Alpha ,
        Self::Rgb8Alpha => TrybLączenia::Rgb8Alpha ,
        Self::Luma16 => TrybLączenia::Luma16 ,
        Self::Luma16Alpha => TrybLączenia::Luma16Alpha ,
        Self::Rgb16 => TrybLączenia::Rgb16 ,
        Self::Rgb16Alpha => TrybLączenia::Rgb16Alpha ,
    }
}
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}
impl BitDepth for BdepthWebp {
    fn format(&self) -> ImgExtTag { ImgExtTag::Webp}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Webp")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Webp")}
    fn tryb_laczenia(&self) -> TrybLączenia {
    match self {
        Self::Rgb8 => TrybLączenia::Rgb8,
        Self::Rgb8Alpha => TrybLączenia::Rgb8Alpha,
    }
}
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}
impl BitDepth for BdepthAvif {
    fn format(&self) -> ImgExtTag { ImgExtTag::Avif}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Avif")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Avif")}
    fn tryb_laczenia(&self) -> TrybLączenia {
    match self {
        Self::Rgb8 => TrybLączenia::Rgb8,
        Self::Rgb8Alpha => TrybLączenia::Rgb8Alpha,
        Self::Rgb10 => TrybLączenia::Rgb10,
        Self::Rgb10Alpha => TrybLączenia::Rgb10Alpha,
    }
}
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}
impl BitDepth for BdepthTga {
    fn format(&self) -> ImgExtTag { ImgExtTag::Tga}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Tga")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Tga")}
    fn tryb_laczenia(&self) -> TrybLączenia {
    match self {
        Self::Luma8 => TrybLączenia::Luma8,
        Self::HighColor16 => TrybLączenia::HighColor16,
        Self::TrueColor24 => TrybLączenia::TrueColor24,
        Self::TrueColorA32 => TrybLączenia::TrueColorA32,
        }
    }
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}
impl BitDepth for BdepthQoi {
    fn format(&self) -> ImgExtTag { ImgExtTag::Qoi}
    fn label_min(&self) -> &'static str {self.get_message().unwrap_or("brak danych bdepth msg Qoi")}
    fn label_max(&self) -> &'static str {self.get_detailed_message().unwrap_or("brak danych bdepth detailded msg Qoi")}
    fn tryb_laczenia(&self) -> TrybLączenia {
    match self {
        BdepthQoi::Color24 => TrybLączenia::Color24,
        BdepthQoi::Color32 => TrybLączenia::Color32,
    }
}
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
}

impl BdepthAvif {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Rgb8 => "btn_id_batch_avif_rgb8",
            Self::Rgb8Alpha => "btn_id_batch_avif_rgb8a",
            Self::Rgb10 => "btn_id_batch_avif_rgb10",
            Self::Rgb10Alpha => "btn_id_batch_avif_rgb10a",
        }
    }

    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Rgb8 => "R8",
            Self::Rgb8Alpha => "R8a",
            Self::Rgb10 => "R10",
            Self::Rgb10Alpha => "R10a",
        }
    }
}
impl BdepthJpg {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Rgb8 => "btn_id_batch_jpg_rgb8",
            Self::Luma8 => "btn_id_batch_jpg_luma8",
        }
    }
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Rgb8 => "R8",
            Self::Luma8 => "L8",
        }
    }
}
impl BdepthPng {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Luma8 => "btn_id_batch_png_luma8",
            Self::Luma8Alpha => "btn_id_batch_png_luma8a",
            Self::Rgb8 => "btn_id_batch_png_rgb8",
            Self::Rgb8Alpha => "btn_id_batch_png_rgb8a",
            Self::Luma16 => "btn_id_batch_png_luma16",
            Self::Luma16Alpha => "btn_id_batch_png_luma16a",
            Self::Rgb16 => "btn_id_batch_png_rgb16",
            Self::Rgb16Alpha => "btn_id_batch_png_rgb16a",
        }
    }
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Luma8 => "L8",
            Self::Luma8Alpha => "L8a",
            Self::Rgb8 => "R8",
            Self::Rgb8Alpha => "R8a",
            Self::Luma16 => "L16",
            Self::Luma16Alpha => "L16a",
            Self::Rgb16 => "R16",
            Self::Rgb16Alpha => "R16a",
        }
    }
}
impl BdepthQoi {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Color24 => "btn_id_batch_qoi_c24",
            Self::Color32 => "btn_id_batch_qoi_c32",
        }
    }
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Color24 => "C24",
            Self::Color32 => "C32",
        }
    }
}
impl BdepthTga {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Luma8 => "btn_id_batch_tga_l8",
            Self::HighColor16 => "btn_id_batch_tga_hc16",
            Self::TrueColor24 => "btn_id_batch_tga_tc24",
            Self::TrueColorA32 => "btn_id_batch_tga_tc32",
        }
    }
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Luma8 => "L8",
            Self::HighColor16 => "HC16",
            Self::TrueColor24 => "TC24",
            Self::TrueColorA32 => "TC32",
        }
    }
}
impl BdepthWebp {
    pub fn bath_konwersja_id(&self) -> &'static str {
        match self {
            Self::Rgb8 => "btn_id_batch_webp_rgb8",
            Self::Rgb8Alpha => "btn_id_batch_webp_rgb8a",
        }
    }
    pub fn maly_wariant(&self) -> &'static str {
        match self {
            Self::Rgb8 => "R8",
            Self::Rgb8Alpha => "R8a",
        }
    }
}