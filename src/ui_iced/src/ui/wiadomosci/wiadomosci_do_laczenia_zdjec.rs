use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::ActProces;
use enumy::opcje::{AvifChroma, AvifMetodaKompresji, JpgQuant, JpgSamplingFac, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowychPojedyncze, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::LogTxDoŁączeniaZdjęć;
use futures::channel::mpsc;
use iced::Task;
use laczenie_plikow::laczenie_fot_struct_enums::fn_do_laczenia_fot;
use std::mem::discriminant;
use std::path::PathBuf;

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
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(huehue) => match huehue {
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg {
                        jakosc:90,
                        progresywny:false,
                        bit_depth: OptFormatyKoloruObrazOgólny::B8,
                        sampling: JpgSamplingFac::R420,
                        quant: JpgQuant::Default,
                        scans: 4,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    };
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Png  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Png {
                        kompresja:3,
                        bit_depth: OptFormatyKoloruObrazOgólny::B8,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp {
                        jakosc: 90,
                        bit_depth: OptFormatyKoloruObrazOgólny::B8,
                        lossless: false,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Tga {
                        bit_depth: OptFormatyKoloruObrazuTga::TrueColor24,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff {
                        metoda_kompresji: OptMetodaKompresjiZdjecia::Brak,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi  => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Qoi {
                        bit_depth: OptFormatyKoloruObrazuQoi::Color24,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
                OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif => {
                    let piksidipsi = OptRozszerzeniaPlikówZdjęciowychPojedyncze::Avif {
                        chroma: AvifChroma::C420,
                        speed: 0,
                        metoda_kompresji: AvifMetodaKompresji::Av1,
                        lossy: Some(90),
                        bit_depth: OptFormatyKoloruObrazuAvif::B10,
                    };
                    if discriminant(&self.dane_temp_do_łączenia_zdjęć.out_format) != discriminant(&piksidipsi) {
                        self.dane_temp_do_łączenia_zdjęć.out_format = piksidipsi;
                        self.dane_temp_do_łączenia_zdjęć.tag = huehue;
                    }
                }
            },

            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciJpg(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg {ref mut jakosc,..} = self.dane_temp_do_łączenia_zdjęć.out_format{
                    *jakosc = procent
                };

            }
            ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgSampling(xx) => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg { ref mut sampling, .. } = self
                    .dane_temp_do_łączenia_zdjęć.out_format
                {
                    *sampling = xx;
                }

            }
            ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgQua(xx) => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg { ref mut quant, .. } = self
                    .dane_temp_do_łączenia_zdjęć.out_format
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *quant = xx;
                }

            }
            ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgScans(skany) => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg { ref mut scans, .. } = self
                    .dane_temp_do_łączenia_zdjęć.out_format
                {
                    *scans = skany; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(lejlejlej) =>  {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Png {ref mut bit_depth,..} = self.dane_temp_do_łączenia_zdjęć.out_format{
                    *bit_depth = lejlejlej;
                }

                }
            //     OptFormatyKoloruObrazOgólny::B8a => {
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = true;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
            //     }
            //     OptFormatyKoloruObrazOgólny::B16 => {
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = true;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
            //     }
            //     OptFormatyKoloruObrazOgólny::B16a => {
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = true;
            //     }
            //     _ => {
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bit = true;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_8bita = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bit = false;
            //         self.stan_boolean_do_laczenia_zdjec.png_wybrane_16bita = false;
            //     }
            // },
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiPng(procent) => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Png {ref mut kompresja,..} = self.dane_temp_do_łączenia_zdjęć.out_format{*kompresja = procent};
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieWebp(lejlejlej) =>  {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp{ref mut bit_depth,..} = self.dane_temp_do_łączenia_zdjęć.out_format{*bit_depth = lejlejlej;};
            },
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciWebp(procent) => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp { ref mut jakosc,.. } = self.dane_temp_do_łączenia_zdjęć.out_format{ *jakosc = procent; }
            }

            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianalosslessWebp => {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp {ref mut lossless,..} = self.dane_temp_do_łączenia_zdjęć.out_format{ *lossless = !*lossless}

            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieTga(lejlejlej) =>  {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Tga{ref mut bit_depth,..} = self.dane_temp_do_łączenia_zdjęć.out_format{*bit_depth = lejlejlej;};
            },
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(lejlejlej) =>  {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff{ref mut metoda_kompresji } = self.dane_temp_do_łączenia_zdjęć.out_format{*metoda_kompresji = lejlejlej;};
            },

            ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieNazwaChanged(blob) => {
                self.dane_temp_do_łączenia_zdjęć.nazwa = blob;
            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieQoi(lejlejlej) =>  {
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Qoi{ref mut bit_depth,..} = self.dane_temp_do_łączenia_zdjęć.out_format{*bit_depth = lejlejlej;};
            },

            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfZstd(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff {
                    metoda_kompresji:OptMetodaKompresjiZdjecia::Zstd(ref mut aktualny_procent), .. } = self.dane_temp_do_łączenia_zdjęć.out_format {

                        *aktualny_procent = procent;

                }

            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfBzip2(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff {
                    metoda_kompresji: OptMetodaKompresjiZdjecia::Bzip2(ref mut aktualny_procent), ..
                } = self.dane_temp_do_łączenia_zdjęć.out_format {

                        *aktualny_procent = procent;

                }

            }
            ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfXz(procent) => {

                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff {
                    metoda_kompresji: OptMetodaKompresjiZdjecia::Xz(ref mut aktualny_procent), ..
                } = self.dane_temp_do_łączenia_zdjęć.out_format {

                        *aktualny_procent = procent;

                }

            }

            ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec => {

                let dane_do_obrobki = self.dane_temp_do_łączenia_zdjęć.clone();
                self.temat.temp.aktywny_proces = ActProces::ŁączenieZdjęć;
                // self.status_zmiany_fot_log = Default::default();
                // println!("ścieżka przekazywana to: {:?}", self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa);


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
                    self.temat.temp.aktywny_proces = ActProces::Żodyn;
                }
                LogTxDoŁączeniaZdjęć::Błąd(_) => {
                    self.temat.temp.aktywny_proces = ActProces::Żodyn;
                }
            },
            _ => {}
        }
        Task::none()
    }
}