use strum::{Display, EnumIter, EnumMessage, EnumString};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, EnumIter,Display, Default)]
pub enum ForJpgSamplingFac {
    R444,
    R440,
    R441,
    R422,
    #[default]
    R420,
    R421,
    R411,
    R410,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, EnumIter, EnumMessage, Display, Default)]
pub enum ForJpgQuant {
    #[default]
    #[strum(message = "def", detailed_message = "Default")]
    Default,
    #[strum(message = "flat", detailed_message = "Flat")]
    Flat,
    #[strum(message = "CMS", detailed_message = "CustomMsSsim")]
    CustomMsSsim,
    #[strum(message = "CPH", detailed_message = "CustomPsnrHvs")]
    CustomPsnrHvs,
    #[strum(message = "IM", detailed_message = "ImageMagick")]
    ImageMagick,
    #[strum(message = "KSC", detailed_message = "KleinSilversteinCarney")]
    KleinSilversteinCarney,
    #[strum(message = "DXR", detailed_message = "DentalXRays")]
    DentalXRays,
    #[strum(message = "VDM", detailed_message = "VisualDetectionModel")]
    VisualDetectionModel,
    #[strum(message = "IDM", detailed_message = "ImprovedDetectionModel")]
    ImprovedDetectionModel,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumIter, EnumString, Default)]
pub enum ForAvifChroma {
    C444,
    C422,
    #[default]
    C420,

}
pub struct DaneDodatkoweZdjec{
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}

#[derive(Clone, Debug)]
pub enum ColorProfilePhoto {
    ICC(Vec<u8>),
    NCLX(ColorNclx),
    Exr(PrzestrzeńExr),
    None,
}
#[derive(Clone, Debug)]
pub enum PrzestrzeńExr {
    LinearSRGB,
    LinearCustom([f32; 8]),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ColorNclx {
    pub primaries: libheif_rs::ColorPrimaries,
    pub transfer: libheif_rs::TransferCharacteristics,
    pub matrix: libheif_rs::MatrixCoefficients,
    pub full_range: bool,
}
