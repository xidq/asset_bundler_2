use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use dds_ops::dds_import::export_dds_array_to_jpg;
use dds_ops::dds_wczytywanie_zdjec::dds_ogarnij_ze_zdjec_do_paczki;
use enumy::inne_ui::ActProces;
use enumy::opcje::{OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::{LogTxDoPakowanieDds, LogTxDoRozpakowanieDds};
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;

impl Program {
    pub fn update_message_dds(&mut self, msg: DdsMessage) -> Task<DdsMessage> {
        match msg {
            DdsMessage::PakowaniePathInFiles => {

                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Tekstury DDS", &["dds"])
                    .pick_files()

                {
                    self.dane_temp_do_pakowania_dds.ścieżka_wejściowa =
                        if path.is_empty(){
                            None
                        }else{
                            Some(path)
                        };
                }
            }
            DdsMessage::PakowaniePathInFolders => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folders()

                {
                    self.dane_temp_do_pakowania_dds.ścieżka_wejściowa =
                        if path.is_empty(){
                            None
                        }else{
                            Some(path)
                        };
                }
            }

            DdsMessage::PakowaniePathOutBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_pakowania_dds.ścieżka_wyjściowa = path;
                }
            }
            DdsMessage::PakowaniePathOut(ścieżka) => {
                self.dane_temp_do_pakowania_dds.ścieżka_wyjściowa = PathBuf::from(ścieżka);
            }

            DdsMessage::ZmienMenuDds(nowy_stan_menu) => self.ui_dds_podmenu = nowy_stan_menu,
            DdsMessage::PakowanieFormat(nowy) => {
                self.dane_temp_do_pakowania_dds.format = nowy;
            }
            DdsMessage::PakowanieKompresja(nowa) => {
                self.dane_temp_do_pakowania_dds.kompresja = nowa;
            }
            DdsMessage::PakowanieNazwa(lel) => {
                self.dane_temp_do_pakowania_dds.nazwa = lel;
            }

            DdsMessage::PakowanieStart => {
                let dane_do_pakowania_dds = self.dane_temp_do_pakowania_dds.clone();
                self.temat.temp.aktywny_proces = ActProces::DdsPakowanie;
                dbg!(&dane_do_pakowania_dds);
                // self.status_zmiany_fot_log = Default::default();
                // println!(
                //     "ścieżka przekazywana to: {:?}",
                //     self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa
                // );



                let (tx, rx) = mpsc::channel::<LogTxDoPakowanieDds>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = dds_ogarnij_ze_zdjec_do_paczki(dane_do_pakowania_dds, tx).await;
                            })
                            .await
                    },
                    |_| DdsMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, DdsMessage::PakowaniePostęp);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            DdsMessage::PakowaniePostęp(progress) => {
                match progress {
                    LogTxDoPakowanieDds::StatusPakowanieDdsStart => {
                        // self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxDoPakowanieDds::StatusPakowanieDdsWtrakcie(procent) => {
                        self.status_dds_pakowanie.w_trakcie = procent;
                    }


                    LogTxDoPakowanieDds::StatusPakowanieDdsKoniec(czas) => {
                        // self.status_zmiany_fot_log.msg_end =
                        //     format!("Zakończono w czasie: {}", czas);
                        self.status_dds_pakowanie.koniec = czas;
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                    }
                    LogTxDoPakowanieDds::StatusPakowanieDdsBłąd(err) => {
                        self.status_dds_pakowanie.err = err;
                        // self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        // self.checker_bool_status_zbiorowe_przetwarzanie_zdjęć = false;
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                    }
                }
            }
            DdsMessage::RozpakInPathBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_file()
                {
                    self.dane_temp_do_rozpakowywania_dds.ścieżka_wejściowa = path;
                }
            }
            DdsMessage::RozpakInPath(ścieżka) => {
                self.dane_temp_do_rozpakowywania_dds.ścieżka_wejściowa = PathBuf::from(ścieżka);
            }
            DdsMessage::RozpakOutPathBtn => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_rozpakowywania_dds.ścieżka_wyjściowa = path;
                }
            }
            DdsMessage::RozpakOutPath(ścieżka) => {
                self.dane_temp_do_rozpakowywania_dds.ścieżka_wyjściowa = PathBuf::from(ścieżka);
            }
            DdsMessage::RozkapExt(huehue) => {
                self.dane_temp_do_rozpakowania_dds_formaty_zdjec = huehue

            },
            DdsMessage::RozpakBitDepth(rozs, kolor) => {

                // 1. Dobieramy się bezpośrednio do pola (to nie jest Vec, więc bez iteracji)
                let format = &mut self.dane_temp_do_rozpakowywania_dds.rozszerzenie;

                // 2. Sprawdzamy, czy aktualnie wybrany format pasuje do klikniętego znacznika
                // i od razu wyciągamy referencję do wektora kolorów (bit_depth)
                let bdepth_vec = match (format, &rozs) {
                    (OptRozszerzeniaPlikówZdjęciowych::Jpg { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg) => Some(bit_depth),
                    (OptRozszerzeniaPlikówZdjęciowych::Png { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Png) => Some(bit_depth),
                    (OptRozszerzeniaPlikówZdjęciowych::Webp { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp) => Some(bit_depth),
                    _ => None, // Formaty bez bit_depth lub niezgodność znacznika
                };

                // 3. Skoro bit_depth to JEDYNY VEC, to tutaj robimy standardowy toggle
                if let Some(vec) = bdepth_vec {
                    if let Some(pos) = vec.iter().position(|x| *x == kolor) {
                        vec.remove(pos); // Usuń jeśli istnieje
                    } else {
                        vec.push(kolor); // Dodaj jeśli nie ma
                    }
                }

            }
            DdsMessage::RozpakExtDane(huehue) => {
                match huehue{
                    OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                            OptRozszerzeniaPlikówZdjęciowych::Jpg{
                                jakosc,
                                progresywny,
                                bit_depth,
                                sampling,
                                quant,
                                scans,
                            }

                    }
                    OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, bit_depth } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                            OptRozszerzeniaPlikówZdjęciowych::Png{
                                kompresja,
                                bit_depth,
                            }

                    }
                    OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc, lossless, bit_depth } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                        OptRozszerzeniaPlikówZdjęciowych::Webp{
                            jakosc,
                            lossless,
                            bit_depth,
                        }
                    }
                    OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                            OptRozszerzeniaPlikówZdjęciowych::Tga{
                                bit_depth,
                            }
                    }

                    OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                            OptRozszerzeniaPlikówZdjęciowych::Ff{ metoda_kompresji }
                    }
                    OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth } => {
                        self.dane_temp_do_rozpakowywania_dds.rozszerzenie=
                            OptRozszerzeniaPlikówZdjęciowych::Qoi{ bit_depth }
                    }
                    OptRozszerzeniaPlikówZdjęciowych::Avif { .. } => {}
                };
            }
            DdsMessage::RozpakStart => {
                let dane_do_rozpakowania_dds = self.dane_temp_do_rozpakowywania_dds.clone();
                self.temat.temp.aktywny_proces = ActProces::DdsRozpakowanie;
                dbg!(&dane_do_rozpakowania_dds);
                // self.status_zmiany_fot_log = Default::default();
                // println!(
                //     "ścieżka przekazywana to: {:?}",
                //     self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa
                // );



                let (tx, rx) = mpsc::channel::<LogTxDoRozpakowanieDds>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = export_dds_array_to_jpg(dane_do_rozpakowania_dds, tx).await;
                            })
                            .await
                    },
                    |_| DdsMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, DdsMessage::RozpakPostęp);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            DdsMessage::RozpakPostęp(progress) => {
                match progress {
                    LogTxDoRozpakowanieDds::StatusRozpakowanieDdsStart => {}
                    LogTxDoRozpakowanieDds::StatusRozpakowanieDdsWtrakcie(_) => {}
                    LogTxDoRozpakowanieDds::StatusRozpakowanieDdsKoniec(_) => {
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                    }
                    LogTxDoRozpakowanieDds::StatusRozpakowanieDdsBłąd(_) => {
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;

                    }
                }
            }
            _ => {}
        }
        Task::none()
    }
}