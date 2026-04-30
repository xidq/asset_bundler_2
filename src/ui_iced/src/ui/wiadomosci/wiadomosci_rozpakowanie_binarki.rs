use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;
use binarka::rozpakowywanie_plikow::ogarnianie_dekompresji;
use enumy::inne_ui::ActProces;
use enumy::statusy::LogTxDoDekompresjiPliku;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;

impl Program {
    pub fn update_message_rozpakowanie_binarki(&mut self, msg: RozpakowanieBinarkiMessage) -> Task<RozpakowanieBinarkiMessage> {
        match msg {
            RozpakowanieBinarkiMessage::InputFileText(s) => {
                self.dane_temp_do_dekompresji_plików.ścieżka_pliku = PathBuf::from(s)
            }
            RozpakowanieBinarkiMessage::InputFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Plik .jrzs", &["jrzs"])
                    .pick_file()
                {
                    self.dane_temp_do_dekompresji_plików.ścieżka_pliku = path;
                }
            }
            RozpakowanieBinarkiMessage::OutputPathText(s) => {
                self.dane_temp_do_dekompresji_plików.ścieżka_docelowa = PathBuf::from(s)
            }
            RozpakowanieBinarkiMessage::OutputPath => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_dekompresji_plików.ścieżka_docelowa = path;
                }
            }
            RozpakowanieBinarkiMessage::Uruchom => {
                let _ = self.temat.temp.aktywny_proces == ActProces::RozpakowaniePliku;
                let zestaw = self.dane_temp_do_dekompresji_plików.clone();

                let (tx, rx) = mpsc::channel::<LogTxDoDekompresjiPliku>(100);
                
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {

                        handle
                            .spawn(async move {
                                let _ = ogarnianie_dekompresji(zestaw, tx).await;
                            })
                            .await
                    },
                    |_| RozpakowanieBinarkiMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, RozpakowanieBinarkiMessage::LogProcesu);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            
            RozpakowanieBinarkiMessage::LogProcesu(progres) => {
                match progres {
                    
                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówZbieraniePlików { current, max } => {

                        self.status_rozpakowywania_log.kontrola_pliku = (current, max);
                        
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDeszyfracja { current, max } => {
                       
                        self.status_rozpakowywania_log.deszyfrowanie = (current, max);
                        
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDekompresja {
                        pamięć
                        
                    } => {

                        self.status_rozpakowywania_log.dekompresja = pamięć;
                        
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówRozpakowywanie {
                        current, 
                        max,
                    } => {

                        self.status_rozpakowywania_log.rozpakowanie = (current, max);
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówZakończenie { czas } => {

                            self.status_rozpakowywania_log.czas = czas;
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówBłąd(err) => {

                        self.status_rozpakowywania_log.błąd = err;
                        
                    }
                }
                
            }
        _ => {}
        }
    Task::none()
    }
}
    