use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::BinPakMsg;
use binarka::pakowanie_plikow::ogarnianie_eksportu;
use chrono::Local;
use enumy::enums_structs_io::LogPakowanie;
use enumy::inne_ui::{ActProces, BtnState};
use enumy::statusy::LogTxDoKompresjiPliku;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::rozszerzenia::{ImgExt, ImgExtTag};
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::BinUnpakMsg;

impl Program {
    pub fn update_message_pakowanie_binarki(&mut self, msg: BinPakMsg) -> Task<BinPakMsg> {
        match msg {
            BinPakMsg::InputPath => {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_bin_pak.ścieżka_in= path;
                }

                let _ = self.update(Message::ChckStatus);
            }
            BinPakMsg::OutputPath => {
                
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_bin_pak.ścieżka_out = path;
                }

                let _ = self.update(Message::ChckStatus);

            }
            BinPakMsg::Filtr(filtr) => {

                self.dane_bin_pak.filtracja = filtr;
            }
            BinPakMsg::Kompresja(poziom) => {
                self.dane_bin_pak.kompresja = poziom;
            }
            BinPakMsg::Uruchom => {

                self.status_pakowanie_log = LogPakowanie::default();
                self.temat.temp.aktywny_proces = Some(ActProces::BinPak);
                let _ = self.update(Message::ChckStatus);

                let zestaw = self.dane_bin_pak.clone();
                dbg!(&zestaw);
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
                    |_| BinPakMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, BinPakMsg::LogProcesu);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            BinPakMsg::LogProcesu(progres) => {

                match progres {
                    
                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZnalezionePliki { pliki } => {
                        
                        self.status_pakowanie_log.zbieranie_plików = pliki;
                        
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówPakowanie { aktualny, suma } => {

                        self.status_pakowanie_log.pakowanie = (aktualny, suma)
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji {  aktualny, suma} => {

                        self.status_pakowanie_log.kompresja = (aktualny, suma);

                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesSzyfrowania {  aktualny, suma } => {
                        self.status_pakowanie_log.szyfrowanie = (aktualny, suma);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZakonczono { czas } => {

                        self.temat.temp.aktywny_proces = None;


                        self.log_prawe_okno.push(format!(
                            "{} [Pakowanie] Zakończone, minęło: {}",
                            Local::now(),
                            czas
                        ));
                        self.status_pakowanie_log.koniec =czas;
                        let _ = self.update(Message::ChckStatus);

                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(err) => {
                        
                        self.temat.temp.aktywny_proces = None;
                        self.log_prawe_okno.push(format!(
                            "!!! {}: {} !!!",
                            "log_status_critical_error",
                            err
                        ));
                        self.status_pakowanie_log.błąd = err;
                        let _ = self.update(Message::ChckStatus);

                    }

                    
                }

            }
            _ => {}
            
        }
        Task::none()
    }
}