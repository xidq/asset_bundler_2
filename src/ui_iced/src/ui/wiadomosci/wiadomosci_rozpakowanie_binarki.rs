use std::default;
use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::BinUnpakMsg;
use binarka::rozpakowywanie_plikow::ogarnianie_dekompresji;
use enumy::inne_ui::{ActProces, BtnState, UiPods};
use enumy::statusy::LogTxBinUnpak;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;
use crate::ui::wiadomosci::message_ui::Message;

impl Program {
    pub fn update_message_rozpakowanie_binarki(&mut self, msg: BinUnpakMsg) -> Task<BinUnpakMsg> {
        match msg {
            BinUnpakMsg::InputFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Plik .jrzs", &["jrzs"])
                    .pick_file()
                {
                    self.dane_bin_unpak.ścieżka_pliku = path;
                }

                let _ = self.update(Message::ChckStatus);

            }
            BinUnpakMsg::OutputPath => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_bin_unpak.ścieżka_docelowa = path;
                }

                let _ = self.update(Message::ChckStatus);

            }
            BinUnpakMsg::Uruchom => {
                self.status_rozpakowywania_log = Default::default();
                self.temat.temp.act_proc = Some(ActProces::BinUnpak);
                let _ = self.update(Message::ChckStatus);

                // self.temat.btn_state.insert(UiPodstrony::BinRozpakowanie.get_id_child(), BtnState::Processing);
                let zestaw = self.dane_bin_unpak.clone();

                let (tx, rx) = mpsc::channel::<LogTxBinUnpak>(100);
                
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {

                        handle
                            .spawn(async move {
                                let _ = ogarnianie_dekompresji(zestaw, tx).await;
                            })
                            .await
                    },
                    |_| BinUnpakMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, BinUnpakMsg::LogProcesu);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            
            BinUnpakMsg::LogProcesu(progres) => {
                match progres {
                    
                    LogTxBinUnpak::Zbieranie { current, max } => {

                        self.status_rozpakowywania_log.kontrola_pliku = (current, max);
                        
                    }

                    LogTxBinUnpak::Deszyfracja { current, max } => {
                       
                        self.status_rozpakowywania_log.deszyfrowanie = (current, max);
                        
                    }

                    LogTxBinUnpak::Dekompresja {

                        pamięć
                        
                    } => {

                        self.status_rozpakowywania_log.dekompresja = pamięć;
                        
                    }

                    LogTxBinUnpak::Rozpakowywanie {

                        current, 
                        max,

                    } => {

                        self.status_rozpakowywania_log.rozpakowanie = (current, max);
                    }

                    LogTxBinUnpak::Finito { czas } => {

                        self.status_rozpakowywania_log.czas = czas;
                        let _ = self.update(Message::UpdateProcesUiBtnPost);
                        self.temat.temp.act_proc = None;
                        let _ = self.update(Message::ChckStatus);


                    }

                    LogTxBinUnpak::Błąd(err) => {

                        self.status_rozpakowywania_log.błąd = err;

                        let _ = self.update(Message::UpdateProcesUiBtnPost);
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
    