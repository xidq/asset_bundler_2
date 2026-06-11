use iced::Color;
use std::collections::HashMap;
use strum::{EnumIter, EnumMessage};

pub const KOLOR_TŁA: Color = Color::from_rgb(0.11, 0.11, 0.1);
// pub const KOLOR_FLIRT: Color = Color::from_rgb(162. / 255., 0., 109. / 255.);
pub const KOLOR_BRILIANT_CRIMSON: Color = Color::from_rgb8(231, 81, 119);
pub const KOLOR_PEACH_PUFF: Color = Color::from_rgb8(255, 218, 185);
pub const KOLOR_CRIMSON_GLORY: Color = Color::from_rgb8(190, 0, 50);
pub const KOLOR_SPANISH_ORANGE: Color = Color::from_rgb8(232, 97, 0);
pub const KOLOR_COTTON_CANDY: Color = Color::from_rgb8(255, 188, 217);
pub const KOLOR_LIGHT_PINK: Color = Color::from_rgb8(255,182,193);

pub const KOLOR_ERROR: Color = Color::from_rgb8(255, 20, 20);

pub const KOLOR_CZCIONKI_SREDNI: Color = Color::from_rgba(1., 1., 1., 0.6);
pub const KOLOR_CZCIONKI_JASNY: Color = Color::from_rgba(1., 1., 1., 0.8);
pub const KOLOR_OBRAMOWANIA_NIE_AKTYWNY: Color = Color::from_rgba(1., 1., 1., 0.2);

pub const WYSOKOSC_CZCIONEK_PRZYCISKI: f32 = 40.;
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
impl Default for UstawieniaThemeWsio {
    fn default() -> Self {
        Self{
            kolory: ObecnyColorTheme {
                binarka: KOLOR_BRILIANT_CRIMSON,
                konwersja: KOLOR_SPANISH_ORANGE,
                laczenie: KOLOR_PEACH_PUFF,
                dds: KOLOR_COTTON_CANDY,
                ustawienia: KOLOR_CRIMSON_GLORY,
                hint: KOLOR_LIGHT_PINK,
            },
            obecny_theme: ObecnyColorThemePrzezroczystosci {
                max: 0.9,
                hi: 0.7,
                mid: 0.5,
                low: 0.2,
                min: 0.0,
                kolor: Color::WHITE,
                err_font: Color::from_rgba(1.,0.5,0.5,0.8),
                bground: KOLOR_TŁA,
                bground_lewy: Color::from_rgb(0.1, 0.11, 0.13),
            },
            tekst: ObecnyColorCzcionkiPrzezroczystosci {
                max: 0.9,
                hi: 0.7,
                mid: 0.5,
                low: 0.3,
                min: 0.0,
                kolor: Color::WHITE,
            },
            ustawienia: Ustawienia {
                halp_menu: false,
                debug_menu: false,
            },
            temp: Temp {
                act_proc: None,
                act_window: UiPods::BinPak,
                start_btn_status: StartBtnStatus{
                    bin_pak: BtnState::LackData,
                    bin_unpak: BtnState::LackData,
                    konwersja: BtnState::LackData,
                    dds_pak: BtnState::LackData,
                    dds_unpak: BtnState::LackData,
                    laczenie: BtnState::LackData,
                },
            },
            btn_state: Default::default(),
        }
    }
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
    // KonwExrCompDwaa,
    // KonwExrCompDwab,
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
    KonwersjaExrKompresja,
    KonwersjaFileTreatment,
    MergeExrKompresja,
    DdsExrKompresja,
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
    KonwExifToggle,
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
impl Default for ObecnyColorTheme{
    fn default()->ObecnyColorTheme{
        ObecnyColorTheme{
            binarka: KOLOR_BRILIANT_CRIMSON,
            konwersja: KOLOR_SPANISH_ORANGE,
            laczenie: KOLOR_PEACH_PUFF,
            dds: KOLOR_COTTON_CANDY,
            ustawienia: KOLOR_CRIMSON_GLORY,
            hint: KOLOR_LIGHT_PINK,
        }
    }
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
