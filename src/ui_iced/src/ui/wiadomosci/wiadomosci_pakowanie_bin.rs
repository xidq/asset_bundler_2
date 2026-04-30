use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use binarka::pakowanie_plikow::ogarnianie_eksportu;
use chrono::Local;
use enumy::enums_structs_io::LogPakowanie;
use enumy::inne_ui::ActProces;
use enumy::statusy::LogTxDoKompresjiPliku;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;

impl Program {
    pub fn update_message_pakowanie_binarki(&mut self, msg: PakowanieBinarkiMessage) -> Task<PakowanieBinarkiMessage> {
        match msg {
            PakowanieBinarkiMessage::InputPathText(s) => {
                self.dane_temp_do_kompresji_plików.ścieżka_in = PathBuf::from(s)
            }
            PakowanieBinarkiMessage::InputPath => {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_temp_do_kompresji_plików.ścieżka_in= path;
                }
            }
            PakowanieBinarkiMessage::OutputPathText(s) => {
                self.dane_temp_do_kompresji_plików.ścieżka_out = PathBuf::from(s)
            }
            PakowanieBinarkiMessage::OutputPath => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_kompresji_plików.ścieżka_out = path;
                }
            }
            PakowanieBinarkiMessage::Nazwa(xx) => {
                self.dane_temp_do_kompresji_plików.nazwa = xx;
            }
            PakowanieBinarkiMessage::Filtr(filtr) => {

                self.dane_temp_do_kompresji_plików.filtracja = filtr;
            }
            PakowanieBinarkiMessage::Kompresja(poziom) => {
                self.dane_temp_do_kompresji_plików.kompresja = poziom;
            }
            PakowanieBinarkiMessage::Uruchom => {
                self.status_pakowanie_log = LogPakowanie::default();
                self.temat.temp.aktywny_proces = ActProces::PakowaniePliku;
                let zestaw = self.dane_temp_do_kompresji_plików.clone();

                let (tx, rx) = mpsc::channel::<LogTxDoKompresjiPliku>(100);
                
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {
                        handle
                            .spawn(async move {
                                let _ = ogarnianie_eksportu(zestaw, tx).await;
                            })
                            .await
                    },
                    |_| PakowanieBinarkiMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, PakowanieBinarkiMessage::LogProcesu);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            PakowanieBinarkiMessage::LogProcesu(progres) => {

                match progres {
                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZnalezionePliki { pliki } => {
                        
                            self.status_pakowanie_log.zbieranie_plików = pliki;
                        
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówPakowanie { aktualny, suma } => {
                        
                            self.status_pakowanie_log.pakowanie = (aktualny, suma)
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji { procent } => {
                        
                            self.status_pakowanie_log.kompresja = procent;

                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesSzyfrowania { procent } => {
                            self.status_pakowanie_log.szyfrowanie = procent;
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZakonczono { czas } => {
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;

                        self.log_prawe_okno.push(format!(
                            "{} [Pakowanie] Zakończone, minęło: {}",
                            Local::now(),
                            czas
                        ));
                        self.status_pakowanie_log.koniec =czas;
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(err) => {
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                        self.log_prawe_okno.push(format!(
                            "!!! {}: {} !!!",
                            "log_status_critical_error",
                            err
                        ));
                        self.status_pakowanie_log.błąd = err;
                    }

                    
                }

            }
            _ => {}
        }
        Task::none()
    }
}