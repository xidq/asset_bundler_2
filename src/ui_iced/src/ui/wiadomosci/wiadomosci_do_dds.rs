use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use dds_ops::dds_wczytywanie_zdjec::dds_ogarnij_ze_zdjec_do_paczki;
use enumy::statusy::LogTxDoPakowanieDds;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;

impl Program {
    pub fn update_message_dds(&mut self, msg: DdsMessage) -> Task<DdsMessage> {
        match msg {
            DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowejWybór => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_pakowania_dds.ścieżka_wejściowa = path;
                }
            }
            DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowej(ścieżka) => {
                self.dane_temp_do_pakowania_dds.ścieżka_wejściowa = PathBuf::from(ścieżka);
            }
            DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowejWybór => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_pakowania_dds.ścieżka_wyjściowa = path;
                }
            }
            DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowej(ścieżka) => {
                self.dane_temp_do_pakowania_dds.ścieżka_wyjściowa = PathBuf::from(ścieżka);
            }

            DdsMessage::ZmienMenuDds(nowy_stan_menu) => self.ui_dds_podmenu = nowy_stan_menu,
            DdsMessage::DDS_Pakowanie_ZmianaWybranegoFormatu(nowy) => {
                self.dane_temp_do_pakowania_dds.format = nowy;
            }
            DdsMessage::DDS_Pakowanie_ZmianaWybranejKompresji(nowa) => {
                self.dane_temp_do_pakowania_dds.kompresja = nowa;
            }
            DdsMessage::DdsPakowanieZmianaNazwy(lel) => {
                self.dane_temp_do_pakowania_dds.nazwa = lel;
            }

            DdsMessage::DdsPakowanieWysylanieDanych => {
                let dane_do_pakowania_dds = self.dane_temp_do_pakowania_dds.clone();
                self.checker_bool_status_dds = (true,false);
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

                let nasluchiwanie = Task::run(rx, DdsMessage::DdsPakowaniePostep);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            DdsMessage::DdsPakowaniePostep(progress) => {
                match progress {
                    LogTxDoPakowanieDds::StatusPakowanieDdsStart => {
                        // self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxDoPakowanieDds::StatusPakowanieDdsWtrakcie(procent) => {
                        // println!("Update dostał procent: {}", procent); // <-- DEBUG
                        // self.status_zmiany_fot_log.plik_procent = procent;
                        // self.status_zmiany_fot_log.msg_proces = format!("{}", procent);
                    }


                    LogTxDoPakowanieDds::StatusPakowanieDdsKoniec(czas) => {
                        // self.status_zmiany_fot_log.msg_end =
                        //     format!("Zakończono w czasie: {}", czas);
                        self.checker_bool_status_dds = (false,false);
                    }
                    LogTxDoPakowanieDds::StatusPakowanieDdsBłąd(err) => {
                        // self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        // self.checker_bool_status_zbiorowe_przetwarzanie_zdjęć = false;
                        self.checker_bool_status_dds = (false,false);
                    }
                }
            }
            DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowejWybór => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_rozpakowywania_dds.ścieżka_wejściowa = path;
                }
            }
            DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowej(ścieżka) => {
                self.dane_temp_do_rozpakowywania_dds.ścieżka_wejściowa = PathBuf::from(ścieżka);
            }
            DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowejWybór => {

                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_rozpakowywania_dds.ścieżka_wyjściowa = path;
                }
            }
            DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowej(ścieżka) => {
                self.dane_temp_do_rozpakowywania_dds.ścieżka_wyjściowa = PathBuf::from(ścieżka);
            }
            _ => {}
        }
        Task::none()
    }
}