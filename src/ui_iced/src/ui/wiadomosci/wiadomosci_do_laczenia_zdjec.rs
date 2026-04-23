use std::path::PathBuf;
use futures::channel::mpsc;
use iced::Task;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych};
use enumy::statusy::LogTxDoŁączeniaZdjęć;
use laczenie_plikow::laczenie_fot_struct_enums::fn_do_laczenia_fot;
use crate::ui::program::Program;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;

impl Program {
    pub fn update_message_łączenie_zdjęć(&mut self, msg: ŁączenieZdjęćMessage) -> Task<ŁączenieZdjęćMessage> {
        match msg {
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieR => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_temp_do_łączenia_zdjęć.sciezka_r = Some(path);
                }
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieG => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_temp_do_łączenia_zdjęć.sciezka_g = Some(path);
                }
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieB => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_temp_do_łączenia_zdjęć.sciezka_b = Some(path);
                }
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieA => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_temp_do_łączenia_zdjęć.sciezka_a = Some(path);
                }
            }
            ŁączenieZdjęćMessage::WybierzFolderOutFotoLaczenie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_łączenia_zdjęć.sciezka_out = path;
                }
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieRPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_temp_do_łączenia_zdjęć.sciezka_r = gfd;
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieGPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_temp_do_łączenia_zdjęć.sciezka_g = gfd;
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieBPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_temp_do_łączenia_zdjęć.sciezka_b = gfd;
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieAPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_temp_do_łączenia_zdjęć.sciezka_a = gfd;
            }
            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieOutPathChanged(s) => {
                self.dane_temp_do_łączenia_zdjęć.sciezka_out = PathBuf::from(s);
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(huehue) => match huehue.as_ref() {
                "jpg" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = true;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = false;
                }
                "png" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = true;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = false;
                }
                "webp" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = true;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = false;
                }
                "tga" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = true;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = false;
                }
                "ff" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = true;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = false;
                }
                "qoi" => {
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany = false;
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany = true;
                }
                _ => {}
            },

            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciJpg(procent) => {
                self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(lejlejlej) => match lejlejlej {
                OptFormatyKoloruObrazOgólny::B8a => {
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = true;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
                }
                OptFormatyKoloruObrazOgólny::B16 => {
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = true;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
                }
                OptFormatyKoloruObrazOgólny::B16a => {
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = true;
                }
                _ => {
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = true;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
                }
            },
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiPng(procent) => {
                self.stan_boolean_do_laczenia_zdjec.png_kompresja = procent;
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieWebp(lejlejlej) => match lejlejlej {
                OptFormatyKoloruObrazOgólny::B8a => {
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_rgb = false;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_alpha = true;
                }
                _ => {
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_rgb = true;
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_alpha = false;
                }
            },
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciWebp(procent) => {
                self.stan_boolean_do_laczenia_zdjec.webp_jakosc = procent;
            }

            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianalosslessWebp => {
                self.stan_boolean_do_laczenia_zdjec.webp_lossless =
                    !self.stan_boolean_do_laczenia_zdjec.webp_lossless
            }

            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieNazwaChanged(blob) => {
                self.dane_temp_do_łączenia_zdjęć.nazwa = blob;
            }

            ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec => {
                let png_depth = match (
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit,
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita,
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit,
                    self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita,
                ) {
                    (_, true, _, _) => OptFormatyKoloruObrazOgólny::B8a,
                    (_, _, true, _) => OptFormatyKoloruObrazOgólny::B16,
                    (_, _, _, true) => OptFormatyKoloruObrazOgólny::B16a,
                    (_, _, _, _) => OptFormatyKoloruObrazOgólny::B8,
                };
                let webp_depth = match (
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_rgb,
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany_alpha,
                ) {
                    (_, true) => OptFormatyKoloruObrazOgólny::B8a,
                    (_, _) => OptFormatyKoloruObrazOgólny::B8,
                };

                let rozszerzenie: OptRozszerzeniaPlikówZdjęciowych = match (
                    self.stan_boolean_do_laczenia_zdjec.jpg_wybrany,
                    self.stan_boolean_do_laczenia_zdjec.png_wybrany,
                    self.stan_boolean_do_laczenia_zdjec.webp_wybrany,
                    self.stan_boolean_do_laczenia_zdjec.tga_wybrany,
                    self.stan_boolean_do_laczenia_zdjec.ff_wybrany,
                    self.stan_boolean_do_laczenia_zdjec.qoi_wybrany,
                ) {
                    (_, true, _, _, _, _) => OptRozszerzeniaPlikówZdjęciowych::Png {
                        kompresja: self.stan_boolean_do_laczenia_zdjec.png_kompresja,
                        bit_depth: vec![png_depth],
                    },
                    (_, _, true, _, _, _) => OptRozszerzeniaPlikówZdjęciowych::Webp {
                        jakosc: self.stan_boolean_do_laczenia_zdjec.webp_jakosc,
                        lossless: false,
                        bit_depth: vec![webp_depth],
                    },
                    (_, _, _, true, _, _) => {
                        OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth: vec![] }
                    }
                    (_, _, _, _, true, _) => OptRozszerzeniaPlikówZdjęciowych::Ff {
                        metoda_kompresji: OptMetodaKompresjiZdjecia::Brak,
                    },
                    (_, _, _, _, _, true) => {
                        OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth: vec![] }
                    }
                    (_, _, _, _, _, _) => OptRozszerzeniaPlikówZdjęciowych::Jpg {
                        jakosc: self.stan_boolean_do_laczenia_zdjec.jpg_jakosc,
                        progresywny: false,
                        bit_depth: vec![OptFormatyKoloruObrazOgólny::B8],
                    },
                };
                let mut dane_do_obrobki = self.dane_temp_do_łączenia_zdjęć.clone();
                self.checker_bool_status_łączenie_zdjęć = true;
                // self.status_zmiany_fot_log = Default::default();
                // println!("ścieżka przekazywana to: {:?}", self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa);

                dane_do_obrobki.out_format = rozszerzenie;
                dbg!(&dane_do_obrobki.sciezka_r);
                dbg!(&dane_do_obrobki.sciezka_g);
                dbg!(&dane_do_obrobki.sciezka_b);
                dbg!(&dane_do_obrobki.sciezka_a);
                dbg!(&dane_do_obrobki.sciezka_out);
                dbg!(
                    "wysyłano tego struct [łączenie] (rozszerzenia_plików_zdjęciowych): \n {}",
                    &dane_do_obrobki.out_format
                );
                let (tx, rx) = mpsc::channel::<LogTxDoŁączeniaZdjęć>(100);

                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = fn_do_laczenia_fot(dane_do_obrobki, tx).await;
                            })
                            .await
                    },
                    |_| ŁączenieZdjęćMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, ŁączenieZdjęćMessage::PostepLaczeniaFot);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
                // let (tx, rx) = mpsc::channel::<LogTxDoŁączeniaZdjęć>(100);
                //
                //
                // self.checker_bool_status_łączenie_zdjęć = true;
                //
                // // 1. Task wykonujący pracę
                // let operacja = Task::perform(
                //     async move {
                //         // Po prostu wywołaj funkcję. Iced zajmie się resztą.
                //         let _ = fn_do_laczenia_fot(dane_do_obrobki, tx).await;
                //     },
                //     |_| ŁączenieZdjęćMessage::Nic,
                // );
                //
                // // 2. Task nasłuchujący postępu (Stream)
                // let nasluchiwanie = Task::run(rx, ŁączenieZdjęćMessage::PostepLaczeniaFot);
                //
                // // Łączymy oba i ZWRACAMY
                // Task::batch(vec![operacja, nasluchiwanie]);

            }
            ŁączenieZdjęćMessage::PostepLaczeniaFot(progress) => match progress {
                LogTxDoŁączeniaZdjęć::Start => {}
                LogTxDoŁączeniaZdjęć::Koniec => {
                    self.checker_bool_status_łączenie_zdjęć = false;
                }
                LogTxDoŁączeniaZdjęć::Błąd(_) => {
                    self.checker_bool_status_łączenie_zdjęć = false;
                }
            },
            _ => {}
        }
        Task::none()
    }
}