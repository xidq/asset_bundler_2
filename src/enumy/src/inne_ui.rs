use iced::Color;
use std::collections::HashMap;
use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Eq)]
pub enum UiPods {
    BinPak,
    BinUnpak,
    KonwPath,
    KonwExt,
    KonwRes,
    KonwEtc,
    Merge,
    MergeExt,
    DdsPak,
    DdsUnpak,
    DdsExt,
    Ustawienia,
}
pub struct UstawieniaThemeWsio{
    pub kolory:ObecnyColorTheme,
    pub obecny_theme:ObecnyColorThemePrzezroczystosci,
    pub tekst:ObecnyColorCzcionkiPrzezroczystosci,
    pub ustawienia: Ustawienia,
    pub temp: Temp,
    pub btn_state:HashMap<&'static str,BtnState>,
}
pub struct Temp{
    pub act_proc: Option<ActProces>,
    pub act_window: UiPods,
    pub start_btn_status: StartBtnStatus,
}
#[derive(Clone, Debug, EnumMessage, PartialEq)]
pub enum TextInputType{
    #[strum(message = "mgt_input_folder")]
    BinKompPathIn,
    #[strum(message = "mgt_output_folder")]
    BinKompPathOut,
    #[strum(message = "mgt_file_name")]
    BinKompNazwa,
    #[strum(message = "mgt_input_folder")]
    BinDekompPathIn,
    #[strum(message = "mgt_output_folder")]
    BinDekompPathOut,
    #[strum(message = "mgt_input_folder_or_file")]
    KonwPathIn,
    #[strum(message = "mgt_output_folder")]
    KonwPathOut,
    #[strum(message = "mgt_input_file_r")]
    MergPathInR,
    #[strum(message = "mgt_input_file_g")]
    MergPathInG,
    #[strum(message = "mgt_input_file_b")]
    MergPathInB,
    #[strum(message = "mgt_input_file_a")]
    MergPathInA,
    #[strum(message = "mgt_output_folder")]
    MergePathOut,
    #[strum(message = "mgt_file_name")]
    MergeNazwa,
    #[strum(message = "mgt_output_folder")]
    DdsPathOut,
    #[strum(message = "mgt_file_name")]
    DdsNazwa,
    #[strum(message = "mgt_input_file")]
    DdsRozPathIn,
    #[strum(message = "mgt_output_folder")]
    DdsRozPathOut,
    #[strum(message = "mgt_file_name")]
    DdsRozNazwa,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SliderType{
    KonwJpgQuality,
    KonwersjaJpgScans,
    KonwersjaAvifSpeed,
    KonwersjaAvifQuality,
    KonwersjaPngKompresja,
    KonwersjaWebpJakosc,
    KonwersjaFfZstd,
    KonwersjaFfBzip2,
    KonwersjaFfXz,
    KonwersjaNoising,
    MergeJpgQuality,
    MergeJpgScans,
    MergeAvifSpeed,
    MergeAvifQuality,
    MergePngKompresja,
    MergeWebpJakosc,
    MergeFfZstd,
    MergeFfBzip2,
    MergeFfXz,
    DdsJpgScans,
    DdsJpgQuality,
    DdsWebpJakosc,
    DdsFfBzip2,
    DdsFfZstd,
    DdsFfXz,
    DdsAvifSpeed,
    DdsAvifQuality,
    DdsPngKompresja,
}
#[derive(Clone, Debug)]
pub enum DropdownType{
    KonwersjaInterpolacja,
    BinFilter,
    BinKompresja,
    KonwersjaKompresjaFf,
    KonwersjaJpgQuant,
    KonwersjaJpgSample,
    KonwersjaAvifKompresja,
    KonwersjaAvifChroma,
    MergeJpgSample,
    MergeJpgQuant,
    MergeAvifChroma,
    MergeAvifKompresja,
    MergeKompresjaFf,
    DdsPakComp,
    DdsPakFormat,
    DdsKompresjaFf,
    DdsAvifKompresja,
    DdsAvifChroma,
    DdsJpgQuant,
    DdsJpgSample,
}
#[derive(Clone, Debug)]

pub enum PrzyciskiGlowneMenu{
    Binarka,
    Konwersja,
    Łączenie,
    Dds,
    Ustawienia,
}
#[derive(Clone, Debug, EnumMessage,)]
pub enum ButtonType{
    #[strum(message = "mgt_input_folder")]
    BinKompPathIn,
    #[strum(message = "mgt_output_folder")]
    BinKompPathOut,
    #[strum(message = "mgt_output_folder")]
    BinDekompPathIn,
    #[strum(message = "mgt_input_folder")]
    BinDekompPathOut,
    #[strum(message = "mgt_input_file")]
    KonwPathInFile,
    #[strum(message = "mgt_input_folder")]
    KonwPathInFolder,
    #[strum(message = "mgt_output_folder")]
    KonwPathOut,
    KonwRozszerzenia,
    #[strum(message = "hint_conversion_jpg_prog")]
    KonwJpgProg,
    #[strum(message = "hint_conversion_avif_lossless")]
    KonwAvifLoss,
    #[strum(message = "hint_conversion_webp_losless")]
    KonwWebpLoss,
    #[strum(message = "mgt_input_file_r")]
    MergPathInR,
    #[strum(message = "mgt_input_file_g")]
    MergPathInG,
    #[strum(message = "mgt_input_file_b")]
    MergPathInB,
    #[strum(message = "mgt_input_file_a")]
    MergPathInA,
    MergeRozszerzenia,
    #[strum(message = "hint_conversion_jpg_prog")]
    MergeJpgProg,
    #[strum(message = "hint_conversion_avif_lossless")]
    MergeAvifLoss,
    #[strum(message = "mgt_output_folder")]
    MergePathOut,
    #[strum(message = "hint_conversion_webp_losless")]
    MergeWebpLoss,
    #[strum(message = "mgt_input_file_multiple")]
    DdsPathInFiles,
    #[strum(message = "mgt_input_folder_multiple")]
    DdsPathInFolders,
    #[strum(message = "mgt_output_folder")]
    DdsPathOut,
    #[strum(message = "mgt_input_file")]
    DdsRozPathIn,
    #[strum(message = "mgt_output_folder")]
    DdsRozPathOut,
    DdsRozszerzenia,
    #[strum(message = "hint_conversion_avif_lossless")]
    DdsJpgProg,
    #[strum(message = "hint_conversion_webp_losless")]
    DdsWebpLoss,
    #[strum(message = "hint_conversion_avif_lossless")]
    DdsAvifLoss,
}
pub struct StartBtnStatus{
    pub bin_pak: BtnState,
    pub bin_unpak: BtnState,
    pub konwersja: BtnState,
    pub dds_pak: BtnState,
    pub dds_unpak: BtnState,
    pub laczenie: BtnState,
}
pub struct Ustawienia{
    pub halp_menu:bool,
    pub debug_menu:bool,

}
pub struct  ObecnyColorTheme{
    pub binarka: Color,
    pub konwersja:Color,
    pub laczenie:Color,
    pub dds:Color,
    pub ustawienia:Color,
    pub hint:Color,
}
pub struct ObecnyColorThemePrzezroczystosci{
    pub max:f32,
    pub hi:f32,
    pub mid:f32,
    pub low:f32,
    pub min:f32,
    pub kolor:Color,
    pub err_font:Color,
    pub bground: Color,
    pub bground_lewy: Color,
}
pub struct ObecnyColorCzcionkiPrzezroczystosci{
    pub max:f32,
    pub hi:f32,
    pub mid:f32,
    pub low:f32,
    pub min:f32,
    pub kolor:Color,
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RodzajeContainer{
    Góra,
    Dół,
    Oba,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum ActProces {
    BinPak,
    BinUnpak,
    DdsPak,
    DdsUnpak,
    Merge,
    Konw,
}
#[derive(Debug, Clone,PartialEq)]
pub enum BtnState{
    Active,
    Disabled,
    Processing,
    LackData
}
