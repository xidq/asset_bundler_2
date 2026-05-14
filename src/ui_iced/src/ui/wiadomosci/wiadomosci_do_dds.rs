use crate::ui::program::Program;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;
use dds_ops::dds_import::dds_to_image;
use dds_ops::dds_wczytywanie_zdjec::image_to_dds;
use enumy::inne_ui::ActProces;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForFfKompresja};
use enumy::rozszerzenia::ext::{ImgExt, ImgExtTag, ImgExtSingle};
use enumy::statusy::{LogTxDdsPak, LogTxDdsUnpak};
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;
use enumy::enums_structs_io::FILTERFOTO;

fn toggle_w_vec<T: PartialEq + Clone>(vec: &mut Vec<T>, element: &T) {
    if let Some(pos) = vec.iter().position(|x| x == element) {
        vec.remove(pos);
    } else {
        vec.push(element.clone());
    }
}

impl Program {
    pub fn update_message_dds(&mut self, msg: DdsMsg) -> Task<DdsMsg> {
        match msg {
            DdsMsg::PakowaniePathInFiles => {

                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_files()

                {
                    self.dane_dds_pak.ścieżka_wejściowa =
                        if path.is_empty(){
                            None
                        }else{
                            Some(path)
                        };
                }
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::PakowaniePathInFolders => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folders()

                {
                    self.dane_dds_pak.ścieżka_wejściowa =
                        if path.is_empty(){
                            None
                        }else{
                            Some(path)
                        };
                }
                let _ = self.update(Message::ChckStatus);

            }

            DdsMsg::PakowaniePathOutBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_dds_pak.ścieżka_wyjściowa = path;
                }
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::PakowaniePathOut(ścieżka) => {
                self.dane_dds_pak.ścieżka_wyjściowa = PathBuf::from(ścieżka);
                let _ = self.update(Message::ChckStatus);

            }

            DdsMsg::PakowanieFormat(nowy) => {
                self.dane_dds_pak.format = nowy;
            }
            DdsMsg::PakowanieKompresja(nowa) => {
                self.dane_dds_pak.kompresja = nowa;
            }
            DdsMsg::PakowanieNazwa(lel) => {
                self.dane_dds_pak.nazwa = lel;
            }

            DdsMsg::PakowanieStart => {
                let dane_do_pakowania_dds = self.dane_dds_pak.clone();
                self.temat.temp.act_proc = Some(ActProces::DdsPak);
                let _ = self.update(Message::ChckStatus);

                dbg!(&dane_do_pakowania_dds);
                // self.status_zmiany_fot_log = Default::default();
                // println!(
                //     "ścieżka przekazywana to: {:?}",
                //     self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa
                // );



                let (tx, rx) = mpsc::channel::<LogTxDdsPak>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = image_to_dds(dane_do_pakowania_dds, tx).await;
                            })
                            .await
                    },
                    |_| DdsMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, DdsMsg::PakowaniePostęp);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            DdsMsg::ZdjeciaLaczenieZmianaRozszerzenieFf(lejlejlej) =>  {
                if let ImgExtSingle::Ff{ref mut metoda_kompresji } = self.dane_merge.rozszerzenie {*metoda_kompresji = lejlejlej;};
            },
            DdsMsg::PakowaniePostęp(progress) => {
                match progress {
                    LogTxDdsPak::Start => {
                        // self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxDdsPak::Pending(procent) => {
                        self.status_dds_pakowanie.w_trakcie = procent;
                    }


                    LogTxDdsPak::Finito(czas) => {
                        self.log_prawe_okno.push(czas.clone());

                        // self.status_zmiany_fot_log.msg_end =
                        //     format!("Zakończono w czasie: {}", czas);
                        self.status_dds_pakowanie.koniec = czas;
                        self.temat.temp.act_proc = None;
                        let _ = self.update(Message::ChckStatus);

                    }
                    LogTxDdsPak::Błąd(err) => {
                        self.log_prawe_okno.push(err.clone());

                        self.status_dds_pakowanie.err = err;
                        // self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        // self.checker_bool_status_zbiorowe_przetwarzanie_zdjęć = false;
                        self.temat.temp.act_proc = None;
                        let _ = self.update(Message::ChckStatus);

                    }
                }
            }
            DdsMsg::RozpakInPathBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("dds", &["dds"])
                    .pick_file()
                {
                    self.dane_dds_rozpak.ścieżka_wejściowa = path;
                }
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::RozpakInPath(ścieżka) => {
                self.dane_dds_rozpak.ścieżka_wejściowa = PathBuf::from(ścieżka);
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::RozpakOutPathBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_dds_rozpak.ścieżka_wyjściowa = path;
                }
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::RozpakOutPath(ścieżka) => {
                self.dane_dds_rozpak.ścieżka_wyjściowa = PathBuf::from(ścieżka);
                let _ = self.update(Message::ChckStatus);

            }
            DdsMsg::Bdepth(rozs, kolor_arc) => {
                let format = &mut self.dane_dds_rozpak.rozszerzenie;

                let kolor_any = kolor_arc.jako_any();

                match (format, &rozs) {
                    (ImgExt::Jpg { bit_depth, .. }, ImgExtTag::Jpg) => {
                                                if let Some(k) = kolor_any.downcast_ref::<BdepthJpg>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    (ImgExt::Png { bit_depth, .. }, ImgExtTag::Png) => {
                                                if let Some(k) = kolor_any.downcast_ref::<BdepthPng>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    (ImgExt::Webp { bit_depth, .. }, ImgExtTag::Webp) => {
                        // Zakładam, że JPG/PNG/WebP używają typu BdepthOgolny
                        if let Some(k) = kolor_any.downcast_ref::<BdepthWebp>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    (ImgExt::Qoi { bit_depth, .. }, ImgExtTag::Qoi) => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthQoi>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    (ImgExt::Avif { bit_depth, .. }, ImgExtTag::Avif) => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthAvif>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    (ImgExt::Tga { bit_depth, .. }, ImgExtTag::Tga) => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthTga>() {
                            toggle_w_vec(bit_depth, k);
                        }
                    }
                    _ => {}
                }
            }
            DdsMsg::RozpakExtDane(huehue) => {
                match huehue{
                    ImgExt::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans } => {
                        self.dane_dds_rozpak.rozszerzenie=
                            ImgExt::Jpg{
                                jakosc,
                                progresywny,
                                bit_depth,
                                sampling,
                                quant,
                                scans,
                            }

                    }
                    ImgExt::Png { kompresja, bit_depth } => {
                        self.dane_dds_rozpak.rozszerzenie=
                            ImgExt::Png{
                                kompresja,
                                bit_depth,
                            }

                    }
                    ImgExt::Webp { jakosc, lossless, bit_depth } => {
                        self.dane_dds_rozpak.rozszerzenie=
                        ImgExt::Webp{
                            jakosc,
                            lossless,
                            bit_depth,
                        }
                    }
                    ImgExt::Tga { bit_depth } => {
                        self.dane_dds_rozpak.rozszerzenie=
                            ImgExt::Tga{
                                bit_depth,
                            }
                    }

                    ImgExt::Ff { metoda_kompresji } => {
                        self.dane_dds_rozpak.rozszerzenie=
                            ImgExt::Ff{ metoda_kompresji }
                    }
                    ImgExt::Qoi { bit_depth } => {
                        self.dane_dds_rozpak.rozszerzenie=
                            ImgExt::Qoi{ bit_depth }
                    }
                    ImgExt::Avif { .. } => {}
                };
            }
            DdsMsg::RozpakStart => {
                let dane_do_rozpakowania_dds = self.dane_dds_rozpak.clone();
                self.temat.temp.act_proc = Some(ActProces::DdsUnpak);
                let _ = self.update(Message::ChckStatus);

                dbg!(&dane_do_rozpakowania_dds);
                // self.status_zmiany_fot_log = Default::default();
                // println!(
                //     "ścieżka przekazywana to: {:?}",
                //     self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa
                // );



                let (tx, rx) = mpsc::channel::<LogTxDdsUnpak>(100);

                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = dds_to_image(dane_do_rozpakowania_dds, tx).await;
                            })
                            .await
                    },
                    |_| DdsMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, DdsMsg::RozpakPostęp);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            DdsMsg::RozpakPostęp(progress) => {
                match progress {
                    LogTxDdsUnpak::Start => {}
                    LogTxDdsUnpak::Pending(_) => {}
                    LogTxDdsUnpak::Finito(fin) => {
                        self.temat.temp.act_proc = None;
                        let _ = self.update(Message::ChckStatus);
                        self.log_prawe_okno.push(fin)
                    }
                    LogTxDdsUnpak::Błąd(err) => {
                        self.temat.temp.act_proc = None;
                        let _ = self.update(Message::ChckStatus);
                        self.log_prawe_okno.push(err)

                    }
                    LogTxDdsUnpak::Sprawdzanie(_) => {}
                }
            }
            DdsMsg::JpgProg => {
                if let ImgExt::Jpg { ref mut progresywny, .. } = self
                    .dane_dds_rozpak
                    .rozszerzenie
                {
                    *progresywny = !*progresywny;
                }

            }
            DdsMsg::WebpLoss => {
                if let ImgExt::Webp { ref mut lossless, .. } = self
                    .dane_dds_rozpak
                    .rozszerzenie
                {
                    *lossless = !*lossless;
                }
            }
            DdsMsg::AvifLoss => {
                if let ImgExt::Avif { ref mut lossy, .. } = self
                    .dane_dds_rozpak
                    .rozszerzenie
                {
                    if lossy.is_some(){
                        *lossy = None;
                    }else{
                        *lossy = Some(90);
                    }
                }

            }
            DdsMsg::Rozszerzenia(gwiazdek) => {
                self.dane_dds_rozpak.tag = gwiazdek.clone();

                self.dane_dds_rozpak.rozszerzenie =
                    match gwiazdek {
                        ImgExtTag::Jpg => ImgExt::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: Vec::from([BdepthJpg::Rgb8]),
                            sampling: ForJpgSamplingFac::R420,
                            quant: ForJpgQuant::Default,
                            scans: 4,
                        },
                        ImgExtTag::Png => ImgExt::Png {
                            kompresja: 3,
                            bit_depth: Vec::from([BdepthPng::Rgb8]),
                        },

                        ImgExtTag::Webp => ImgExt::Webp{
                            jakosc: 90,
                            lossless: false,
                            bit_depth: Vec::from([BdepthWebp::Rgb8]),
                        },
                        ImgExtTag::Tga => ImgExt::Tga{
                            bit_depth: Vec::from([BdepthTga::TrueColor24])
                        },
                        ImgExtTag::Ff => ImgExt::Ff{
                            metoda_kompresji: ForFfKompresja::Brak
                        },
                        ImgExtTag::Qoi => ImgExt::Qoi{
                            bit_depth: Vec::from([BdepthQoi::Color24])
                        },
                        ImgExtTag::Avif => ImgExt::Avif {
                            chroma: ForAvifChroma::C420,
                            speed: 3,
                            metoda_kompresji: ForAvifKompresja::Av1,
                            lossy: Some(90),
                            bit_depth: Vec::from([BdepthAvif::Rgb10])
                        }
                    };



                let _ = self.update(Message::ChckStatus);
            }
            _ => {}
        }

        Task::none()
    }
}