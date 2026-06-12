use crate::ui::podstrony::settings::plik_ustawienia::{hex_to_color, plik_z_ustawieniami_popraw, plik_z_ustawieniami_wczytaj};
use crate::ui::program::Program;
use crate::ui::wiadomosci::message_enum::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::MergeMsg;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::BinPakMsg;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::BinUnpakMsg;
use chrono::{Local, Timelike};
use enumy::inne_ui::{ActProces, BtnState, ButtonType, DropdownType, ObecnyColorTheme, SliderType, TextInputType, UiPods};
use enumy::opcje::{OptInterpolacja, OptIstniejePlik, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::rozszerzenia::ext::{ImgExt, ImgExtSingle};
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForDds, ForDdsKompresja, ForExrKompresja, ForFfKompresja};
use enumy::wybranie_jezykowe::UstawieniaMenu::UstawieniaJęzyka;
use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::Task;
use iced_core::{Color, Event};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use strum::{Display, EnumString};
use enumy::log_file_gen::generuj_plik_logow;

#[derive(Debug, EnumString, Display)]
pub enum ElementyDoNazwIZmianUstawien{
    Language,
    ColorBinaryMenu,
    ColorConversionMenu,
    ColorMergingMenu,
    ColorDdsMenu,
    ColorSettingsMenu,
    ColorHint,
}


impl Program {
    fn wczytaj_ustawienia(&mut self, mapa: HashMap<String, String>){
        mapa.into_iter().for_each(|(key, val)| {
            match ElementyDoNazwIZmianUstawien::from_str(&key){
                Ok(ElementyDoNazwIZmianUstawien::Language) => {
                    match val.to_lowercase().as_str() {
                        "pl-pl" | "pl" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::PL },
                        "en-en" | "en" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::EN },
                        "de-de" | "de" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::DE },
                        "hu-hu" | "hu" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::HU },
                        "es-es" | "es" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::ES },
                        "kr-kr" | "kr" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::KR },
                        "jp-jp" | "jp" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::JP },
                        "th-th" | "th" => self.ui_ustawienia = UstawieniaJęzyka{ jezyk: WybórJęzyka::TH },
                        _ => {}
                    }
                },
                Ok(ElementyDoNazwIZmianUstawien::ColorBinaryMenu) => {self.temat.kolory.binarka = hex_to_color(val)}
                Ok(ElementyDoNazwIZmianUstawien::ColorConversionMenu) => {self.temat.kolory.konwersja = hex_to_color(val)}
                Ok(ElementyDoNazwIZmianUstawien::ColorMergingMenu) => {self.temat.kolory.laczenie= hex_to_color(val)}
                Ok(ElementyDoNazwIZmianUstawien::ColorDdsMenu) => {self.temat.kolory.dds = hex_to_color(val)}
                Ok(ElementyDoNazwIZmianUstawien::ColorSettingsMenu) => {self.temat.kolory.ustawienia = hex_to_color(val)}
                Ok(ElementyDoNazwIZmianUstawien::ColorHint) => {self.temat.kolory.hint = hex_to_color(val)}
                _ => {}
            }
        })
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let aktualny_jezyk = match self.ui_ustawienia {
            UstawieniaMenu::UstawieniaJęzyka { jezyk } => jezyk,
            _ => WybórJęzyka::EN,
        };

        match message {
            Message::InitLogStartowy => {
                let powitanie = format!(
                    "{}!!!!!\n {}: {}\n  {}: {}\n   {}, \n    {}: {}\n{}\n---------------------------------------",
                    aktualny_jezyk.t(if Local::now().hour() < 6 {
                        "log_status_welcome_msg_welcome_morning"
                    } else if Local::now().hour() > 19 {
                        "log_status_welcome_msg_welcome_evening"
                    } else {
                        "log_status_welcome_msg_welcome_day"
                    }),
                    aktualny_jezyk.t("log_status_welcome_msg_today"),
                    Local::now().format("%d.%m.%Y"),
                    aktualny_jezyk.t("log_status_welcome_msg_time"),
                    Local::now().format("%H:%M:%S"),
                    aktualny_jezyk.t("log_status_welcome_msg_sys_rdy"),
                    aktualny_jezyk.t("log_status_welcome_msg_lang_detected"),
                    self.startowy_jezyk,
                    aktualny_jezyk.t("log_help_menu")
                );
                self.log_prawe_okno.push(powitanie);
            }
            Message::InitUstawienia => {
                let ggg = plik_z_ustawieniami_wczytaj(false).unwrap_or(None);
                if let Some(dane) = ggg {
                    self.wczytaj_ustawienia(dane);
                }
            }
            Message::DevCustomLog
                if !self.temat.temp.custom_log.is_empty() => {
                    generuj_plik_logow(self.temat.temp.custom_log.clone());
                }

            Message::DevCustomLogText(log) => {
                self.temat.temp.custom_log = log;
            }

            Message::UsuńLogi => self.log_prawe_okno = Vec::new(),
            Message::DevZmienJezyk(nowy) => {
                self.ui_ustawienia = UstawieniaMenu::UstawieniaJęzyka { jezyk: nowy };
                if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::Language.to_string(), nowy.to_string() ){
                    eprintln!("Błąd zapisu ustawień: {}", e);
                };
            }

            Message::DevResetUstawien => {
                _ = plik_z_ustawieniami_wczytaj(true);
                self.temat.kolory = ObecnyColorTheme::default();
            }

            Message::DevZmienKolory(typ, kanal,nazwa_koloru) => {
                // let kolor = Color::from_rgb8(nazwa_koloru_r.parse().unwrap(), nazwa_koloru_g.parse().unwrap(), nazwa_koloru_b.parse().unwrap(),);
                let zmien_kolor = |xx:Color| -> Color{match kanal.as_str(){
                    "r" => Color::from_rgb8(nazwa_koloru, (xx.g * 255.) as u8, (xx.b * 255.) as u8),
                    "g" => Color::from_rgb8((xx.r * 255.) as u8, nazwa_koloru,(xx.b * 255.) as u8),
                    "b" => Color::from_rgb8((xx.r * 255.) as u8, (xx.g * 255.) as u8, nazwa_koloru, ),
                    _ => Color::BLACK
                }};
                match typ {
                    UiPods::BinPak|UiPods::BinUnpak => {
                        let kolor = zmien_kolor(self.temat.kolory.binarka);
                        self.temat.kolory.binarka = kolor;
                        if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::ColorBinaryMenu.to_string(), kolor.to_string() ){
                            eprintln!("Błąd zapisu ustawień: {}", e);
                        };
                    }
                    UiPods::KonwPath|UiPods::KonwExt|UiPods::KonwRes|UiPods::KonwEtc => {
                        let kolor = zmien_kolor(self.temat.kolory.konwersja);
                        self.temat.kolory.konwersja = kolor;
                        if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::ColorConversionMenu.to_string(), kolor.to_string() ){
                            eprintln!("Błąd zapisu ustawień: {}", e);
                        };
                    }
                    UiPods::Merge|UiPods::MergeExt => {
                        let kolor = zmien_kolor(self.temat.kolory.laczenie);
                        self.temat.kolory.laczenie = kolor;
                        if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::ColorMergingMenu.to_string(), kolor.to_string() ){
                            eprintln!("Błąd zapisu ustawień: {}", e);
                        };
                    }
                    UiPods::DdsPak|UiPods::DdsUnpak|UiPods::DdsExt => {
                        let kolor = zmien_kolor(self.temat.kolory.dds);
                        self.temat.kolory.dds = kolor;
                        if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::ColorDdsMenu.to_string(), kolor.to_string() ){
                            eprintln!("Błąd zapisu ustawień: {}", e);
                        };
                    }
                    UiPods::Ustawienia => {
                        let kolor = zmien_kolor(self.temat.kolory.ustawienia);
                        self.temat.kolory.ustawienia = kolor;
                        if let Err(e) = plik_z_ustawieniami_popraw(ElementyDoNazwIZmianUstawien::ColorHint.to_string(), kolor.to_string() ){
                            eprintln!("Błąd zapisu ustawień: {}", e);
                        };
                    }
                }
            }

            Message::TextInputHandling(string, typ) => {
                match typ {
                    TextInputType::BinKompPathIn => {
                        self.dane_bin_pak.ścieżka_in = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinKompPathOut => {
                        self.dane_bin_pak.ścieżka_out = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinKompNazwa => {
                        self.dane_bin_pak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinDekompPathIn => {
                        self.dane_bin_unpak.ścieżka_pliku = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinDekompPathOut => {
                        self.dane_bin_unpak.ścieżka_docelowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::KonwPathIn => {
                        self.dane_konw.ścieżka_wejściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::KonwPathOut => {
                        self.dane_konw.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInR => {
                        self.dane_merge.sciezka_r =
                            if !string.is_empty() {
                                Some(PathBuf::from(string))
                            } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInG => {
                        self.dane_merge.sciezka_g =
                            if !string.is_empty() {
                                Some(PathBuf::from(string))
                            } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInB => {
                        self.dane_merge.sciezka_b =
                            if !string.is_empty() {
                                Some(PathBuf::from(string))
                            } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInA => {
                        self.dane_merge.sciezka_a =
                            if !string.is_empty() {
                                Some(PathBuf::from(string))
                            } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergePathOut => {
                        self.dane_merge.sciezka_out = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergeNazwa => {
                        self.dane_merge.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsPathOut => {
                        self.dane_dds_pak.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsNazwa => {
                        self.dane_dds_pak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozPathIn => {
                        self.dane_dds_rozpak.ścieżka_wejściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozPathOut => {
                        self.dane_dds_rozpak.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozNazwa => {
                        self.dane_dds_rozpak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                }
            }

            Message::Dropdown(wybrane, proces) => {
                match proces {
                    DropdownType::KonwersjaInterpolacja => {
                        if let Some(v) = wybrane.downcast_ref::<OptInterpolacja>() {
                            self.dane_konw.inter = *v;
                        }
                    }
                    DropdownType::BinFilter => {
                        if let Some(v) = wybrane.downcast_ref::<OptKompresjaPlikówFiltracjaPlików>() {
                            self.dane_bin_pak.filtracja = *v;
                        }
                    }
                    DropdownType::BinKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<OptKompresjaPlikówPoziomKompresjiZstd>() {
                            self.dane_bin_pak.kompresja = *v;
                        }
                    }
                    DropdownType::KonwersjaJpgQuant => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>()
                            && let Some(ImgExt::Jpg { quant, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            *quant = *v;
                        }
                    }
                    DropdownType::KonwersjaJpgSample => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>()
                            && let Some(ImgExt::Jpg { sampling, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            *sampling = *v;
                        }
                    }
                    DropdownType::KonwersjaKompresjaFf => {
                        if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>()
                            && let Some(ImgExt::Ff { metoda_kompresji, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            *metoda_kompresji = v.ustaw_domyslny_poziom();
                        }
                    }
                    DropdownType::KonwersjaAvifKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>()
                            && let Some(ImgExt::Avif { metoda_kompresji, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            *metoda_kompresji = v.clone();
                        }
                    }
                    DropdownType::KonwersjaAvifChroma => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>()
                            && let Some(ImgExt::Avif { chroma, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            *chroma = v.clone();
                        }
                    }
                    DropdownType::MergeJpgSample => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>()
                            && let ImgExtSingle::Jpg { ref mut sampling, .. } = self.dane_merge.rozszerzenie
                        {
                            *sampling = *v;
                        }
                    }
                    DropdownType::MergeJpgQuant => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>()
                            && let ImgExtSingle::Jpg { ref mut quant, .. } = self.dane_merge.rozszerzenie
                        {
                            *quant = *v;
                        }
                    }
                    DropdownType::MergeAvifChroma => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>()
                            && let ImgExtSingle::Avif { ref mut chroma, .. } = self.dane_merge.rozszerzenie
                        {
                            *chroma = v.clone();
                        }
                    }
                    DropdownType::MergeAvifKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>()
                            && let ImgExtSingle::Avif { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie
                        {
                            *metoda_kompresji = v.clone();
                        }
                    }
                    DropdownType::MergeKompresjaFf => {
                        if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>()
                            && let ImgExtSingle::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie
                        {
                            *metoda_kompresji = v.ustaw_domyslny_poziom();
                        }
                    }
                    DropdownType::DdsPakComp => {
                        if let Some(v) = wybrane.downcast_ref::<ForDdsKompresja>() {
                            self.dane_dds_pak.kompresja = *v;
                        }
                    }
                    DropdownType::DdsPakFormat => {
                        if let Some(v) = wybrane.downcast_ref::<ForDds>() {
                            self.dane_dds_pak.format = *v;
                        }
                    }
                    DropdownType::DdsKompresjaFf => {
                        if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>()
                            && let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            *metoda_kompresji = v.ustaw_domyslny_poziom();
                        }
                    }
                    DropdownType::DdsAvifKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>()
                            && let ImgExt::Avif { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            *metoda_kompresji = v.clone();
                        }
                    }
                    DropdownType::DdsAvifChroma => {
                        if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>()
                            && let ImgExt::Avif { ref mut chroma, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            *chroma = v.clone();
                        }
                    }
                    DropdownType::DdsJpgQuant => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>()
                            && let ImgExt::Jpg { ref mut quant, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            *quant = *v;
                        }
                    }
                    DropdownType::DdsJpgSample => {
                        if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>()
                            && let ImgExt::Jpg { ref mut sampling, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            *sampling = *v;
                        }
                    }
                    DropdownType::KonwersjaExrKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<ForExrKompresja>()
                            && let Some(ImgExt::Exr { kompresja, .. }) = self.dane_konw.rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Exr { .. }))
                        {
                            let inny_wariant =v;
                            *kompresja = *inny_wariant;
                            // *kompresja = match v {
                            //     // ForExrKompresja::Dwaa(_) => ForExrKompresja::Dwaa(Some(45.)), // Dostosuj typ (np. 7 jeśli u8/u32)
                            //     // ForExrKompresja::Dwab(_) => ForExrKompresja::Dwab(Some(45.)),
                            //     inny_wariant => *inny_wariant, // Reszta (Brak, Rle, Zip itd.) zostaje jak była
                            // };
                        }
                    }
                    DropdownType::KonwersjaFileTreatment => {
                        if let Some(v) = wybrane.downcast_ref::<OptIstniejePlik>() {
                            self.dane_konw.istniejace_pliki = *v;
                        }
                    }
                    DropdownType::MergeExrKompresja => {
                        if let Some(ghgh) = wybrane.downcast_ref::<ForExrKompresja>() &&
                            let ImgExtSingle::Exr{ref mut kompresja, ..} = self.dane_merge.rozszerzenie{
                            *kompresja = *ghgh
                        }
                    }
                    DropdownType::DdsExrKompresja => {
                        if let Some(v) = wybrane.downcast_ref::<ForExrKompresja>()
                            && let ImgExt::Exr {ref mut kompresja, .. } = self.dane_dds_rozpak.rozszerzenie
                        {
                            let inny_wariant =v;
                            *kompresja = *inny_wariant;
                        }

                    }
                }
            }

            Message::Przyciski(typ) => {
                return match typ {
                    ButtonType::BinKompPathIn => {
                        self.update_message_pakowanie_binarki(BinPakMsg::InputPath)
                            .map(Message::PakowanieBinarki)
                    }
                    ButtonType::BinKompPathOut => {
                        self.update_message_pakowanie_binarki(BinPakMsg::OutputPath)
                            .map(Message::PakowanieBinarki)
                    }
                    ButtonType::BinDekompPathIn => {
                        self.update_message_rozpakowanie_binarki(BinUnpakMsg::InputFile)
                            .map(Message::RozpakowanieBinarki)
                    }
                    ButtonType::BinDekompPathOut => {
                        self.update_message_rozpakowanie_binarki(BinUnpakMsg::OutputPath)
                            .map(Message::RozpakowanieBinarki)
                    }
                    ButtonType::KonwPathInFile => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathInFile)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwPathInFolder => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathInFolder)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwPathOut => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathOutFolder)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwRozszerzenia => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::Nic)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwJpgProg => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::JpgProg)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwAvifLoss => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::AvifLossyToggle)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwWebpLoss => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::WebpLossless)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::MergPathInR => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieR)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInG => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieG)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInB => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieB)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInA => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieA)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeRozszerzenia => {
                        self.update_message_łączenie_zdjęć(MergeMsg::Nic)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeJpgProg => {
                        self.update_message_łączenie_zdjęć(MergeMsg::JpgProg)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeAvifLoss => {
                        self.update_message_łączenie_zdjęć(MergeMsg::AvifLossyToggle)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergePathOut => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzFolderOutFotoLaczenie)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeWebpLoss => {
                        self.update_message_łączenie_zdjęć(MergeMsg::ZdjeciaLaczenieZmianalosslessWebp)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::DdsPathInFiles => {
                        self.update_message_dds(DdsMsg::PakowaniePathInFiles)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsPathInFolders => {
                        self.update_message_dds(DdsMsg::PakowaniePathInFolders)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsPathOut => {
                        self.update_message_dds(DdsMsg::PakowaniePathOutBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozPathIn => {
                        self.update_message_dds(DdsMsg::RozpakInPathBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozPathOut => {
                        self.update_message_dds(DdsMsg::RozpakOutPathBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozszerzenia => {
                        self.update_message_dds(DdsMsg::Nic)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsJpgProg => {
                        self.update_message_dds(DdsMsg::JpgProg)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsWebpLoss => {
                        self.update_message_dds(DdsMsg::WebpLoss)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsAvifLoss => {
                        self.update_message_dds(DdsMsg::AvifLoss)
                            .map(Message::Dds)
                    }
                    ButtonType::KonwExifToggle => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::ExifToggle)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                }
            }

            Message::Slidery(typ, wartość) => {
                match typ {
                    SliderType::KonwJpgQuality => {
                        if let Some(ImgExt::Jpg { jakosc, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::KonwersjaJpgScans => {
                        if let Some(ImgExt::Jpg { scans, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            *scans = wartość as u8;
                        }
                    }
                    SliderType::KonwersjaAvifSpeed => {
                        if let Some(ImgExt::Avif { speed, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            *speed = wartość;
                        }
                    }
                    SliderType::KonwersjaAvifQuality => {
                        if let Some(ImgExt::Avif { lossy, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            *lossy = Some(wartość as u8);
                        }
                    }
                    SliderType::KonwersjaPngKompresja => {
                        if let Some(ImgExt::Png { kompresja, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Png { .. }))
                        {
                            *kompresja = wartość as u8;
                        }
                    }
                    SliderType::KonwersjaWebpJakosc => {
                        if let Some(ImgExt::Webp { jakosc, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Webp { .. }))
                        {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::KonwersjaFfZstd => {
                        if let Some(ImgExt::Ff { metoda_kompresji, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                        }
                    }
                    SliderType::KonwersjaFfBzip2 => {
                        if let Some(ImgExt::Ff { metoda_kompresji, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                        }
                    }
                    SliderType::KonwersjaFfXz => {
                        if let Some(ImgExt::Ff { metoda_kompresji, .. }) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                        }
                    }
                    SliderType::KonwersjaNoising => {
                        if wartość == 0 {
                            self.dane_konw.noising = None;
                        } else {
                            self.dane_konw.noising = Some(wartość as u8);
                        }
                    }
                    SliderType::MergeJpgQuality => {
                        if let ImgExtSingle::Jpg { ref mut jakosc, .. } = self.dane_merge.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::MergeJpgScans => {
                        if let ImgExtSingle::Jpg { ref mut scans, .. } = self.dane_merge.rozszerzenie {
                            *scans = wartość as u8;
                        }
                    }
                    SliderType::MergeAvifSpeed => {
                        if let ImgExtSingle::Avif { ref mut speed, .. } = self.dane_merge.rozszerzenie {
                            *speed = wartość;
                        }
                    }
                    SliderType::MergeAvifQuality => {
                        if let ImgExtSingle::Avif { ref mut lossy, .. } = self.dane_merge.rozszerzenie {
                            *lossy = Some(wartość as u8);
                        }
                    }
                    SliderType::MergePngKompresja => {
                        if let ImgExtSingle::Png { ref mut kompresja, .. } = self.dane_merge.rozszerzenie {
                            *kompresja = wartość as u8;
                        }
                    }
                    SliderType::MergeWebpJakosc => {
                        if let ImgExtSingle::Webp { ref mut jakosc, .. } = self.dane_merge.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::MergeFfZstd => {
                        if let ImgExtSingle::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                        }
                    }
                    SliderType::MergeFfBzip2 => {
                        if let ImgExtSingle::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                        }
                    }
                    SliderType::MergeFfXz => {
                        if let ImgExtSingle::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                        }
                    }
                    SliderType::DdsJpgScans => {
                        if let ImgExt::Jpg { ref mut scans, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *scans = wartość as u8;
                        }
                    }
                    SliderType::DdsJpgQuality => {
                        if let ImgExt::Jpg { ref mut jakosc, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::DdsWebpJakosc => {
                        if let ImgExt::Webp { ref mut jakosc, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::DdsFfBzip2 => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                        }
                    }
                    SliderType::DdsFfZstd => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                        }
                    }
                    SliderType::DdsFfXz => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                        }
                    }
                    SliderType::DdsAvifSpeed => {
                        if let ImgExt::Avif { ref mut speed, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *speed = wartość;
                        }
                    }
                    SliderType::DdsAvifQuality => {
                        if let ImgExt::Avif { ref mut lossy, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *lossy = lossy.as_mut().map(|_| wartość as u8)
                        }
                    }
                    SliderType::DdsPngKompresja => {
                        if let ImgExt::Png { ref mut kompresja, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *kompresja = wartość as u8;
                        }
                    }
                    // SliderType::KonwExrCompDwaa => {
                    //     if let Some(ImgExt::Exr { kompresja, .. }) = self.dane_konw
                    //         .rozszerzenia
                    //         .iter_mut()
                    //         .find(|f| matches!(f, ImgExt::Exr { .. })) &&
                    //         let ForExrKompresja::Dwaa(Some(poziom)) = kompresja {
                    //         *poziom = wartość as f32;
                    //     }
                    // }
                    // SliderType::KonwExrCompDwab => {
                    //     if let Some(ImgExt::Exr { kompresja, .. }) = self.dane_konw
                    //         .rozszerzenia
                    //         .iter_mut()
                    //         .find(|f| matches!(f, ImgExt::Exr { .. })) &&
                    //         let ForExrKompresja::Dwab(Some(poziom)) = kompresja {
                    //         *poziom = wartość as f32;
                    //     }
                    // }
                }
            }

            Message::Startujemy(proces) => {
                match proces {
                    ActProces::BinPak => {
                        return self.update_message_pakowanie_binarki(BinPakMsg::Uruchom)
                            .map(Message::PakowanieBinarki);
                    }
                    ActProces::BinUnpak => {
                        return self.update_message_rozpakowanie_binarki(BinUnpakMsg::Uruchom)
                            .map(Message::RozpakowanieBinarki);
                    }
                    ActProces::DdsPak => {
                        return self.update_message_dds(DdsMsg::PakowanieStart)
                            .map(Message::Dds);
                    }
                    ActProces::DdsUnpak => {
                        return self.update_message_dds(DdsMsg::RozpakStart)
                            .map(Message::Dds);
                    }
                    ActProces::Merge => {
                        return self.update_message_łączenie_zdjęć(MergeMsg::Uruchom)
                            .map(Message::ŁączenieZdjęć);
                    }
                    ActProces::Konw => {
                        return self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::Uruchom)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć);
                    }
                }
            }

            Message::ZmienWariant(w) => { self.temat.temp.act_window = w; }

            Message::PakowanieBinarki(msg) => {
                return self.update_message_pakowanie_binarki(msg)
                    .map(Message::PakowanieBinarki);
            },
            Message::RozpakowanieBinarki(msg) => {
                return self.update_message_rozpakowanie_binarki(msg)
                    .map(Message::RozpakowanieBinarki);
            },
            Message::ZbiorowePrzetwarzanieZdjęć(msg) => {
                return self.update_message_zbiorowe_przetwarzanie_zdjec(msg)
                    .map(Message::ZbiorowePrzetwarzanieZdjęć);
            },
            Message::ŁączenieZdjęć(msg) => {
                return self.update_message_łączenie_zdjęć(msg)
                    .map(Message::ŁączenieZdjęć);
            },
            Message::Dds(msg) => {
                return self.update_message_dds(msg)
                    .map(Message::Dds);
            },

            Message::ChckStatus => {
                // bin kompresja
                let check_bin_kompresja = self.dane_bin_pak.ścieżka_in.exists() &&
                    self.dane_bin_pak.ścieżka_out.exists() &&
                    !self.dane_bin_pak.nazwa.is_empty();

                self.temat.temp.start_btn_status.bin_pak = match (check_bin_kompresja, self.temat.temp.act_proc.clone()) {
                    (true, None) => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::BinUnpak)) => BtnState::Processing,
                    (_, Some(_)) => BtnState::Disabled,
                };

                // bin dekompresja
                let check_bin_dekompresja = self.dane_bin_unpak.ścieżka_pliku.is_file() &&
                    self.dane_bin_unpak.ścieżka_docelowa.exists();

                self.temat.temp.start_btn_status.bin_unpak = match (check_bin_dekompresja, self.temat.temp.act_proc.clone()) {
                    (true, None) => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::BinPak)) => BtnState::Processing,
                    (_, Some(_)) => BtnState::Disabled,
                };

                // konwersja
                let konwersja_bdepth_check = self.dane_konw.rozszerzenia
                    .iter()
                    .all(|format| format.ma_wybrany_bit_depth());
                let check_konwersja = self.dane_konw.ścieżka_wejściowa.exists() &&
                    self.dane_konw.ścieżka_wyjściowa.exists() &&
                    !self.dane_konw.opcje_rozdzielczości.is_empty() &&
                    !self.dane_konw.rozszerzenia.is_empty() &&
                    konwersja_bdepth_check;

                self.temat.temp.start_btn_status.konwersja = match (check_konwersja, self.temat.temp.act_proc.clone()) {
                    (true, None) => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::Konw)) => BtnState::Processing,
                    (_, Some(_)) => BtnState::Disabled,
                };

                //łączenie fot
                let check_laczenie =
                    (
                        self.dane_merge.sciezka_r.is_some() ||
                            self.dane_merge.sciezka_g.is_some() ||
                            self.dane_merge.sciezka_b.is_some() ||
                            self.dane_merge.sciezka_a.is_some()
                    ) &&
                        !self.dane_merge.nazwa.is_empty() &&
                        self.dane_merge.sciezka_out.exists();

                self.temat.temp.start_btn_status.laczenie = match (check_laczenie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::Merge)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                // dds pakowanie
                let check_dds_pakowanie =
                    self.dane_dds_pak.ścieżka_wejściowa.as_ref().is_some_and(|xx| !xx.is_empty()) &&
                        self.dane_dds_pak.ścieżka_wyjściowa.exists() &&
                        !self.dane_dds_pak.nazwa.is_empty();

                self.temat.temp.start_btn_status.dds_pak = match ( check_dds_pakowanie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::DdsPak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                //dds rozpakowanie

                let check_dds_rozpakowanie =
                    self.dane_dds_rozpak.ścieżka_wejściowa.is_file() &&
                        self.dane_dds_rozpak.ścieżka_wyjściowa.exists() &&
                        !self.dane_dds_rozpak.nazwa.is_empty();

                self.temat.temp.start_btn_status.dds_unpak = match ( check_dds_rozpakowanie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::DdsUnpak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };
            }
            // Message::EventOccurred(Event::Mouse(iced::mouse::Event::ButtonPressed(_0))) => {
            //
            // }
            Message::EventOccurred(Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                                       key,
                                                       modifiers,
                                                       ..
                                                   })) => {


                if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                    self.temat.ustawienia.debug_menu = !self.temat.ustawienia.debug_menu;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno
                        .push(format!("Dev Mode: {}", self.temat.ustawienia.debug_menu));
                }
                if modifiers.control() && key == iced::keyboard::Key::Character("h".into()) {
                    self.temat.ustawienia.halp_menu = !self.temat.ustawienia.halp_menu;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno.push(format!("Halp Mode: {}", self.temat.ustawienia.halp_menu));
                }
            }

            _ => {}
        }
        Task::none()
    }
}