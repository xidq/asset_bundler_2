use crate::ui::program::Program;
use crate::ui::wiadomosci::message_enum::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::ActProces;
use enumy::opcje::OptInterpolacja;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::rozszenienia_zdjec::{ImgExtAvif, ImgExtExr, ImgExtFf, ImgExtJpg, ImgExtPng, ImgExtQoi, ImgExtTga, ImgExtWebp};
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc;
use iced::Task;
use image_conversion::zmiana_fot::main_fn_konwersja;
use std::path::PathBuf;

fn toggle_w_vec<T: PartialEq + Clone>(vec: &mut Vec<T>, element: &T) {
    if let Some(pos) = vec.iter().position(|x| x == element) {
        vec.remove(pos);
    } else {
        vec.push(element.clone());
    }
}

impl Program {
    pub fn update_message_zbiorowe_przetwarzanie_zdjec(&mut self, msg: KonwMsg) -> Task<KonwMsg> {
        match msg {
            KonwMsg::PathInFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_konw.ścieżka_wejściowa = path.clone();

                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            if let Some(rodzic) = path.parent() {
                                self.dane_konw.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                self.dane_konw.ścieżka_wyjściowa = path;
                            }
                        } else {
                            self.dane_konw.ścieżka_wyjściowa = path;
                        }
                    }
                }

                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::PathInFolder => {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_konw.ścieżka_wejściowa = path.clone();

                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            if let Some(rodzic) = path.parent() {
                                self.dane_konw.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                self.dane_konw.ścieżka_wyjściowa = path;
                            }
                        } else {
                            self.dane_konw.ścieżka_wyjściowa = path;
                        }
                    }

                }

                let _ = self.update(Message::ChckStatus);
            }

            KonwMsg::PathInText(s) => {
                self.dane_konw.ścieżka_wejściowa = PathBuf::from(s);
                let _ = self.update(Message::ChckStatus);

            }
            KonwMsg::PathsReset => {
                self.dane_konw.ścieżka_wejściowa = PathBuf::new();
                let _ = self.update(Message::ChckStatus);

            }
            KonwMsg::PathOutPathInBool(zdjecia_edycja_co_jest_na_out) => {
                self.zdjecia_edycja_co_jest_na_out = zdjecia_edycja_co_jest_na_out;
                let xoxo = if self.dane_konw.ścieżka_wejściowa.is_file() {
                    self.dane_konw
                        .ścieżka_wejściowa
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.dane_konw.ścieżka_wejściowa.clone())
                } else {
                    self.dane_konw.ścieżka_wejściowa.clone()
                };
                self.dane_konw.ścieżka_wyjściowa = xoxo

            }
            KonwMsg::PathOutFolder => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_konw.ścieżka_wyjściowa = path;
                }

                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::PathOutText(s) => {
                self.dane_konw.ścieżka_wyjściowa = PathBuf::from(s)
            }

            KonwMsg::WypełnienieAlpha(indeks, wartosc) => {
                match indeks {
                    0 => self.dane_konw.alfa_rgb.0 = wartosc, // Zmieniamy R
                    1 => self.dane_konw.alfa_rgb.1 = wartosc, // Zmieniamy G
                    2 => self.dane_konw.alfa_rgb.2 = wartosc, // Zmieniamy B
                    _ => {}
                }
            }

            KonwMsg::JpgJakość(procent) => {
                if let Some(ImgExtJpg { ref mut jakosc, .. }) = self
                    .dane_konw
                    .rozszerzenia.jpg
                {
                    *jakosc = procent;
                }
            }
            KonwMsg::JpgProg=> {
                if let Some(ImgExtJpg { ref mut progresywny, .. }) = self
                    .dane_konw
                    .rozszerzenia.jpg
                {
                    *progresywny = !*progresywny;
                }

            }
            KonwMsg::JpgSampling(xx) => {
                if let Some(ImgExtJpg { ref mut sampling, .. }) = self
                    .dane_konw
                    .rozszerzenia.jpg
                {
                    *sampling = xx;
                }
            }
            KonwMsg::JpgQua(xx) => {
                if let Some(ImgExtJpg { ref mut quant, .. }) = self
                    .dane_konw
                    .rozszerzenia.jpg
                {
                    *quant = xx;
                }
            }
            KonwMsg::JpgScan(skany) => {
                if let Some(ImgExtJpg { ref mut scans, .. }) = self
                    .dane_konw
                    .rozszerzenia.jpg
                {
                    *scans = skany;
                }
            }
            KonwMsg::PngKompresja(var) => {
                if let Some(ImgExtPng { ref mut kompresja, .. }) = self
                    .dane_konw
                    .rozszerzenia.png
                {
                    *kompresja = var;
                }
            }
            KonwMsg::Interpolacja(xx) => {
                let yy = match xx.as_str() {
                    "OptInterpolacja_nearest" => OptInterpolacja::Nearest,
                    "OptInterpolacja_triangle" => OptInterpolacja::Triangle,
                    "OptInterpolacja_catmull" => OptInterpolacja::CatmullRom,
                    "OptInterpolacja_gaussian" => OptInterpolacja::Gaussian,
                    "OptInterpolacja_lanczos" => OptInterpolacja::Lanczos3,
                    _ => { OptInterpolacja::Lanczos3 }
                };
                self.dane_konw.inter = yy;
            }
            KonwMsg::WebpJakość(procent) => {
                if let Some(ImgExtWebp { ref mut jakosc, .. }) = self
                    .dane_konw
                    .rozszerzenia.webp
                {
                    *jakosc = procent;
                }
            }
            KonwMsg::WebpLossless => {
                if let Some(ImgExtWebp { ref mut lossless, .. }) = self
                    .dane_konw
                    .rozszerzenia.webp
                {
                    *lossless = !*lossless;
                }
            }
            
            KonwMsg::FfKompresja(metoda) => {
                if let Some(ImgExtFf { ref mut metoda_kompresji }) = self
                    .dane_konw
                    .rozszerzenia.ff
                {
                    *metoda_kompresji = metoda;
                }

            }
            KonwMsg::FfKompresjaVal(var) => {
                if let Some(ImgExtFf { ref mut metoda_kompresji }) = self
                    .dane_konw
                    .rozszerzenia.ff
                {
                    match metoda_kompresji {
                        ForFfKompresja::Zstd(v) |
                        ForFfKompresja::Bzip2(v) |
                        ForFfKompresja::Xz(v) => *v = var,

                        ForFfKompresja::Brak => {}
                    }
                }
            }
            
            KonwMsg::AvifSpeed(das) => {
                if let Some(ImgExtAvif { ref mut speed, .. }) = self
                    .dane_konw
                    .rozszerzenia.avif
                {
                    *speed = das;
                }
            }
            KonwMsg::AvifLossyToggle => {
                if let Some(ImgExtAvif { ref mut lossy, .. }) = self
                    .dane_konw
                    .rozszerzenia.avif
                {

                    if lossy.is_some(){
                        *lossy = None;
                    }else{
                        *lossy = Some(90);
                    }

                }

            }
            KonwMsg::AvifLossy(das) => {
                if let Some(ImgExtAvif { ref mut lossy, .. }) = self
                    .dane_konw
                    .rozszerzenia.avif
                {
                    *lossy = Some(das);
                }
            }
            KonwMsg::AvifKompresja(metoda) => {
                if let Some(ImgExtAvif { ref mut metoda_kompresji,.. }) = self
                    .dane_konw
                    .rozszerzenia.avif
                {
                    *metoda_kompresji = metoda;
                }
            }
            KonwMsg::AvifChroma(chromchrom) => {
                if let Some(ImgExtAvif { ref mut chroma,.. }) = self
                    .dane_konw
                    .rozszerzenia.avif
                {
                    *chroma = chromchrom;
                }
            }
            KonwMsg::Noising(procent) => {
                self.dane_konw.noising =
                    if procent == 0u8 { None } else { Some(procent) };
            }
            KonwMsg::Rozdzielczość(khekhe) => {
                let pozycja = self.dane_konw
                    .opcje_rozdzielczości
                    .iter()
                    .position(|x| *x == khekhe);

                match pozycja {
                    Some(index) => {
                        self.dane_konw.opcje_rozdzielczości.remove(index);
                    }
                    None => {
                        self.dane_konw.opcje_rozdzielczości.push(khekhe);

                    }
                }

                let _ = self.update(Message::ChckStatus);

            }
            
            KonwMsg::Bdepth(kolor_arc) => {
                let kolor_any = kolor_arc.jako_any();
                let f = &mut self.dane_konw.rozszerzenia;


                if let Some(k) = kolor_any.downcast_ref::<BdepthJpg>() &&
                    let Some(cfg) = &mut f.jpg {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthPng>() &&
                    let Some(cfg) = &mut f.png {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthWebp>() &&
                    let Some(cfg) = &mut f.webp {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthTga>() &&
                    let Some(cfg) = &mut f.tga {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthQoi>() &&
                    let Some(cfg) = &mut f.qoi {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthAvif>() &&
                    let Some(cfg) = &mut f.avif {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                } else if let Some(k) = kolor_any.downcast_ref::<BdepthExr>() &&
                    let Some(cfg) = &mut f.exr {
                        toggle_w_vec(&mut cfg.bit_depth, k);

                }
                // Format 'Ff' pomijamy, bo nie ma pola bit_depth
                // // 1. Szukamy indeksu na podstawie znacznika
                // if let Some(index) = self.dane_konw
                //     .tag
                //     .iter()
                //     .position(|t| *t == rozs)
                // {
                //     // 2. Pobieramy mutowalną referencję do formatu
                //     if let Some(format_danych) = self.dane_konw
                //         .rozszerzenia
                //         .get_mut(index)
                //     {
                //         // 3. Używamy jako_any(), aby móc porównać dyn z konkretnym typem w Vec
                //         let kolor_any = kolor_arc.jako_any();
                //
                //
                //         match format_danych {
                //             ImgExt::Jpg { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthJpg>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Png { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthPng>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Webp { bit_depth, .. } => {
                //                 // Próbujemy rzutować Arc na konkretny typ siedzący w tym Vec
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthWebp>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Qoi { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthQoi>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Avif { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthAvif>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Tga { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthTga>() {
                //                     toggle_w_vec(bit_depth, k);
                //
                //                 }
                //             }
                //             ImgExt::Exr { bit_depth, .. } => {
                //                 if let Some(k) = kolor_any.downcast_ref::<BdepthExr>() {
                //                     toggle_w_vec(bit_depth, k);
                //                 }
                //             }
                //             _ => {}
                //         }
                //     }
                // }
                let _ = self.update(Message::ChckStatus);

            }


            KonwMsg::Rozszerzenia(gwiazdek) => {

                let f = &mut self.dane_konw.rozszerzenia;

                match gwiazdek {
                    ImgExtTag::Jpg => {if f.jpg.take().is_none() { f.jpg = Some(ImgExtJpg::default()); }}
                    ImgExtTag::Png => {if f.png.take().is_none() { f.png = Some(ImgExtPng::default()); }}
                    ImgExtTag::Webp => {if f.webp.take().is_none() { f.webp = Some(ImgExtWebp::default()); }}
                    ImgExtTag::Tga => {if f.tga.take().is_none() { f.tga = Some(ImgExtTga::default()); }}
                    ImgExtTag::Ff => {if f.ff.take().is_none() { f.ff = Some(ImgExtFf::default()); }}
                    ImgExtTag::Qoi => {if f.qoi.take().is_none() { f.qoi = Some(ImgExtQoi::default()); }}
                    ImgExtTag::Avif => {if f.avif.take().is_none() { f.avif = Some(ImgExtAvif::default()); }}
                    ImgExtTag::Exr => {if f.exr.take().is_none() { f.exr = Some(ImgExtExr::default()); }}
                    ImgExtTag::Unknown => {}
                }
                // let pozycja_tag = self.dane_konw
                //     .tag
                //     .iter()
                //     .position(|t| *t == gwiazdek);
                // let pozycja = self.dane_konw.rozszerzenia.iter().position(|f| {
                //     matches!(
                //         (f, &gwiazdek),
                //         (ImgExt::Jpg {..}, ImgExtTag::Jpg) |
                //         (ImgExt::Png {..}, ImgExtTag::Png) |
                //         (ImgExt::Webp {..}, ImgExtTag::Webp) |
                //         (ImgExt::Tga {..}, ImgExtTag::Tga) |
                //         (ImgExt::Ff {..}, ImgExtTag::Ff) |
                //         (ImgExt::Qoi {..}, ImgExtTag::Qoi) |
                //         (ImgExt::Avif {..}, ImgExtTag::Avif)
                //     )
                // });
                // if let Some(idx) = pozycja_tag {
                //     self.dane_konw.tag.remove(idx);
                // }
                //
                // if let Some(index) = pozycja {
                //     self.dane_konw.rozszerzenia.remove(index);
                // } else {
                //     let nowy_format = match gwiazdek {
                //         ImgExtTag::Jpg => ImgExt::def_jpg(),
                //         ImgExtTag::Png => ImgExt::def_png(),
                //         ImgExtTag::Webp => ImgExt::def_webp(),
                //         ImgExtTag::Tga => ImgExt::def_tga(),
                //         ImgExtTag::Ff => ImgExt::def_ff(),
                //         ImgExtTag::Qoi => ImgExt::def_qoi(),
                //         ImgExtTag::Avif => ImgExt::def_avif(),
                //         ImgExtTag::Unknown => ImgExt::def_exr(),
                //         ImgExtTag::Exr => ImgExt::def_exr()
                //     };
                //
                //     self.dane_konw.rozszerzenia.push(nowy_format);
                //     self.dane_konw.tag.push(gwiazdek.clone());
                //
                // }
                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::ExifToggle => {
                self.dane_konw.exif = !self.dane_konw.exif;
            }
            KonwMsg::Uruchom => {
                let dane_do_obrobki = self.dane_konw.clone();
                self.temat.temp.act_proc = Some(ActProces::Konw);
                let _ = self.update(Message::ChckStatus);

                self.status_zmiany_fot_log = Default::default();
                // dbg!(
                //     "ścieżka przekazywana to: {:?}",
                //     &self.dane_konw
                // );
                //
                // dbg!(
                //     "wysyłanko tego struct(rozszerzenia_plików_zdjęciowych): \n {}",
                //     &dane_do_obrobki.rozszerzenia
                // );
                let (tx, rx) = mpsc::channel::<LogTxKonw>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = main_fn_konwersja(dane_do_obrobki, tx).await;
                            })
                            .await
                    },
                    |_| KonwMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, KonwMsg::Log);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            KonwMsg::Log(progress) => {
                match progress {
                    LogTxKonw::Start => {
                        self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxKonw::Sprawdzanie(gsd) => {
                        self.status_zmiany_fot_log.msg_walidacja = gsd;
                    }
                    LogTxKonw::Rozpoczęto(
                        wartość,
                        suma,
                    ) => {
                        // println!("Update dostał procent: {}", procent); // <-- DEBUG
                        self.status_zmiany_fot_log.plik_procent = (wartość,suma);
                    }
                    LogTxKonw::FiltrowaniePlików(xoxo) => {

                        self.status_zmiany_fot_log.plik_początek = xoxo;

                    }
                    LogTxKonw::Pominięte {
                        sciezka,
                        powod,
                    } => {
                        self.log_prawe_okno.push(format!(
                            "[Obrazy] Pominięto plik z:\n{}\n z powodu: {}",
                            sciezka, powod
                        ));
                    }
                    LogTxKonw::Finito(czas) => {
                        self.status_zmiany_fot_log.msg_end =
                            format!("Zakończono w czasie: {}", czas);
                        self.temat.temp.act_proc = None;

                        let _ = self.update(Message::ChckStatus);
                    }
                    LogTxKonw::Błąd(err) => {
                        self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        self.temat.temp.act_proc = None;

                        let _ = self.update(Message::ChckStatus);
                    }
                }
            }




            _ => {}
        }
        Task::none()

    }
}