use strum::{Display, EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum TrybLączenia {
    Luma8,
    Luma8Alpha,
    Rgb8,
    Rgb8Alpha,
    Rgb10,
    Rgb10Alpha,
    Luma16,
    Luma16Alpha,
    Rgb16,
    Rgb16Alpha,
    HighColor16,
    Color24,
    TrueColor24,
    Color32,
    TrueColorA32,
    Zstd,
    Bzip2,
    Xz,
    Brak,
    Rgb12,
    Rgb12Alpha,
    F16,
    F32,
    F16Half,
    F32Half,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthJpg{
  //  #[strum(serialize = "Głębia 8 bitów", serialize = "8b")] //to jest nadpisanie automatycznie zaimplementowanego display co mogę usunąć w sumie... tylko jak dobrać się do pól
    //Luma8, //no i ofc jest dodany iter do enuma aby nie trzeba było ręcznie wsio wypisywać znowu...
    #[strum(default_with = "L8",message = "Luma8", detailed_message = "Luma 8 bit (BW)")]
    Luma8,
    #[strum(default_with = "R8",message = "Rgb8", detailed_message = "Rgb 8 bit (Color)")]
    Rgb8
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthWebp{
    #[strum(default_with = "R8",message = "Rgb8", detailed_message = "Rgb 8 bit (Color)")]
    Rgb8,
    #[strum(default_with = "R8a",message = "Rgb8a", detailed_message = "Rgb 8 bit + Alpha (Color/w Alpha)")]
    Rgb8Alpha,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthPng{
    #[strum(default_with = "L8",message = "Luma8", detailed_message = "Luma 8 bit (BW)")]
    Luma8,
    #[strum(default_with = "L8a",message = "Luma8a", detailed_message = "Luma 8 bit + Alpha (BW/w Alpha)")]
    Luma8Alpha,
    #[strum(default_with = "R8",message = "Rgb8", detailed_message = "Rgb 8 bit (Color)")]
    Rgb8,
    #[strum(default_with = "R8a",message = "Rgb8a", detailed_message = "Rgb 8 bit + Alpha (Color/w Alpha)")]
    Rgb8Alpha,
    #[strum(default_with = "L16",message = "Luma16", detailed_message = "Luma 16 bit (BW)")]
    Luma16,
    #[strum(default_with = "L16a",message = "Luma16a", detailed_message = "Luma 16 bit + Alpha (BW/w Alpha)")]
    Luma16Alpha,
    #[strum(default_with = "R16",message = "Rgb16", detailed_message = "Rgb 16 bit (Color)")]
    Rgb16,
    #[strum(default_with = "R16a",message = "Rgb16a", detailed_message = "Rgb 16 bit + Alpha (Color/w Alpha)")]
    Rgb16Alpha,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthAvif{
    #[strum(default_with = "R8",message = "Rgb8", detailed_message = "Rgb 8 bit (Color)")]
    Rgb8,
    #[strum(default_with = "R8a",message = "Rgb8a", detailed_message = "Rgb 8 bit + Alpha (Color/w Alpha)")]
    Rgb8Alpha,
    #[strum(default_with = "R10",message = "Rgb10", detailed_message = "Rgb 10 bit (Color)")]
    Rgb10,
    #[strum(default_with = "R10a",message = "Rgb10a", detailed_message = "Rgb 10 bit + Alpha (Color/w Alpha)")]
    Rgb10Alpha,
    #[strum(default_with = "R12",message = "Rgb12", detailed_message = "Rgb 12 bit (Color)")]
    Rgb12,
    #[strum(default_with = "R12a",message = "Rgb12a", detailed_message = "Rgb 12 bit + Alpha (Color/w Alpha)")]
    Rgb12Alpha,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthQoi{
    #[strum(default_with = "C24",message = "Color24", detailed_message = "Rgb 8 bit (Color)")]
    Color24,
    #[strum(default_with = "C32",message = "Color32", detailed_message = "Rgb 8 bit + Alpha (Color/w Alpha)")]
    Color32,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display, Default)]
pub enum BdepthTga{
    #[strum(default_with = "L8",message = "Luma8", detailed_message = "Luma 8 bit (BW)")]
    Luma8,
    #[strum(default_with = "HC16",message = "HC16", detailed_message = "High Color 16 = Rgb 5 bit + Alpha(bool) (Color/w Alpcha 0-1)")]
    HighColor16,
    #[default]
    #[strum(default_with = "TC24",message = "Color24", detailed_message = "True Color 24 = Rgb 8 bit (Color)")]
    TrueColor24,
    #[strum(default_with = "TC32",message = "Color32", detailed_message = "True Color 32 = Rgb 8 bit + Alpha (Color/w Alpha)")]
    TrueColorA32,
}
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumMessage, Display)]
pub enum BdepthExr{
    #[strum(default_with = "F16",message = "F16", detailed_message = "F16")]
    F16,
    #[strum(default_with = "F32",message = "F32", detailed_message = "F32")]
    F32,
    #[strum(default_with = "F16H",message = "F16H", detailed_message = "F16Half")]
    F16Half,
    #[strum(default_with = "F32H",message = "F32H", detailed_message = "F32Half")]
    F32Half,
}

