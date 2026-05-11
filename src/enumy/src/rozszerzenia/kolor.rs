use strum::{Display, EnumIter, EnumMessage, EnumString};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, EnumIter,Display)]
pub enum ForJpgSamplingFac {
    R444,
    R440,
    R441,
    R422,
    R420,
    R421,
    R411,
    R410,
}
impl Default for ForJpgSamplingFac {fn default() -> ForJpgSamplingFac { ForJpgSamplingFac::R420}}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, EnumIter, EnumMessage, Display)]
pub enum ForJpgQuant {
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
impl Default for ForJpgQuant {fn default() -> Self { ForJpgQuant::Default}}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, EnumIter, EnumString)]
pub enum ForAvifChroma {
    C444,
    C422,
    C420,

}
impl Default for ForAvifChroma {fn default() -> Self {
        ForAvifChroma::C420
    }}