use crate::ui::program::Program;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::MergeMsg;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::ActProces;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::ext::{ImgExtSingle, ImgExtTag};
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForFfKompresja};
use enumy::statusy::LogTxMerge;
use file_merge::merge_main::fn_do_laczenia_fot;
use futures::channel::mpsc;
use iced::Task;
use std::mem::discriminant;
use std::path::PathBuf;

impl Program {
    pub fn update_message_łączenie_zdjęć(&mut self, msg: MergeMsg) -> Task<MergeMsg> {
        match msg {
            MergeMsg::WybierzPlikInFotoLaczenieR => {
                if let Some(ref mut path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_merge.sciezka_r = Some(path.clone());
                }
            }
            MergeMsg::WybierzPlikInFotoLaczenieG => {
                if let Some(ref mut path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_merge.sciezka_g = Some(path.clone());
                }
            }
            MergeMsg::WybierzPlikInFotoLaczenieB => {
                if let Some(ref mut path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_merge.sciezka_b = Some(path.clone());
                }
            }
            MergeMsg::WybierzPlikInFotoLaczenieA => {
                if let Some(ref mut path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_merge.sciezka_a = Some(path.clone());
                }
            }
            MergeMsg::WybierzFolderOutFotoLaczenie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_merge.sciezka_out = path;
                }
            }
            MergeMsg::WybierzPlikInFotoLaczenieRPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_merge.sciezka_r = gfd;
            }
            MergeMsg::WybierzPlikInFotoLaczenieGPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_merge.sciezka_g = gfd;
            }
            MergeMsg::WybierzPlikInFotoLaczenieBPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_merge.sciezka_b = gfd;
            }
            MergeMsg::WybierzPlikInFotoLaczenieAPathChanged(s) => {
                let gfd = if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                };
                self.dane_merge.sciezka_a = gfd;
            }
            MergeMsg::WybierzPlikInFotoLaczenieOutPathChanged(s) => {
                self.dane_merge.sciezka_out = PathBuf::from(s);
            }
            MergeMsg::ZdjeciaLaczenieZmianaWybraneRozszerzenie(huehue) => match huehue {
                ImgExtTag::Jpg  => {
                    let piksidipsi = ImgExtSingle::def_jpg();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    };
                }
                ImgExtTag::Png  => {
                    let piksidipsi = ImgExtSingle::def_png();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Webp  => {
                    let piksidipsi = ImgExtSingle::def_webp();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Tga  => {
                    let piksidipsi = ImgExtSingle::def_tga();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Ff  => {
                    let piksidipsi = ImgExtSingle::def_ff();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Qoi  => {
                    let piksidipsi = ImgExtSingle::def_qoi();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Avif => {
                    let piksidipsi = ImgExtSingle::def_avif();
                    if discriminant(&self.dane_merge.rozszerzenie) != discriminant(&piksidipsi) {
                        self.dane_merge.rozszerzenie = piksidipsi;
                        self.dane_merge.tag = huehue;
                    }
                }
                ImgExtTag::Unknown => {}
                ImgExtTag::Exr => {}
            },
            MergeMsg::Bdepth(rozs, kolor_rc) => {
                // dbg!("bdepth", &rozs, &kolor_rc);

                let kolor_any = kolor_rc.jako_any();

                match &mut self.dane_merge.rozszerzenie {
                    ImgExtSingle::Jpg { bit_depth, .. } if rozs == ImgExtTag::Jpg => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthJpg>() {
                            *bit_depth = *k;
                        }
                    }
                    ImgExtSingle::Png { bit_depth, .. } if rozs == ImgExtTag::Png => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthPng>() {
                            *bit_depth = *k;
                        }
                    }
                    ImgExtSingle::Webp { bit_depth, .. } if rozs == ImgExtTag::Webp => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthWebp>() {
                            *bit_depth = *k;
                        }
                    }
                    ImgExtSingle::Avif { bit_depth, .. } if rozs == ImgExtTag::Avif => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthAvif>() {
                            *bit_depth = *k;
                        }
                    }
                    ImgExtSingle::Qoi { bit_depth, .. } if rozs == ImgExtTag::Qoi => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthQoi>() {
                            *bit_depth = *k;
                        }
                    }
                    ImgExtSingle::Tga { bit_depth, .. } if rozs == ImgExtTag::Tga => {
                        if let Some(k) = kolor_any.downcast_ref::<BdepthTga>() {
                            *bit_depth = *k;
                        }
                    }

                    _ => {}
                }

            }

            MergeMsg::ZdjeciaLaczenieZmianaJakosciJpg(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let ImgExtSingle::Jpg {ref mut jakosc,..} = self.dane_merge.rozszerzenie {
                    *jakosc = procent
                };

            }
            MergeMsg::ZdjeciaEdycjaZmianaJpgSampling(xx) => {
                if let ImgExtSingle::Jpg { ref mut sampling, .. } = 
                    self.dane_merge.rozszerzenie
                {
                    *sampling = xx;
                }

            }
            MergeMsg::ZdjeciaEdycjaZmianaJpgQua(xx) => {
                if let ImgExtSingle::Jpg { ref mut quant, .. } = 
                    self.dane_merge.rozszerzenie
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *quant = xx;
                }

            }
            MergeMsg::ZdjeciaEdycjaZmianaJpgScans(skany) => {
                if let ImgExtSingle::Jpg { ref mut scans, .. } = 
                    self.dane_merge.rozszerzenie
                {
                    *scans = skany; // Jeśli znaleziono, aktualizujemy wartość
                }
            }



            MergeMsg::Rozszerzenia(gwiazdek) => {
                // dbg!("rozszerzenia", &gwiazdek);
                    let nowy_format = match gwiazdek {
                        ImgExtTag::Jpg => (ImgExtSingle::def_jpg()),
                        ImgExtTag::Png => (ImgExtSingle::def_png()),

                        ImgExtTag::Webp => (ImgExtSingle::def_webp()),
                        ImgExtTag::Tga => (ImgExtSingle::def_tga()),
                        ImgExtTag::Ff => (ImgExtSingle::def_ff()),
                        ImgExtTag::Qoi => (ImgExtSingle::def_qoi()),
                        ImgExtTag::Avif => (ImgExtSingle::def_avif()),
                        ImgExtTag::Unknown => (ImgExtSingle::def_jpg()),
                        ImgExtTag::Exr => ImgExtSingle::def_exr()
                    };
                    self.dane_merge.rozszerzenie = nowy_format;
                    self.dane_merge.tag = gwiazdek;

            }
            MergeMsg::ZdjeciaLaczenieZmianaKompresjiPng(procent) => {
                if let ImgExtSingle::Png {ref mut kompresja,..} = self.dane_merge.rozszerzenie {*kompresja = procent};
            }

            MergeMsg::ZdjeciaLaczenieZmianaJakosciWebp(procent) => {
                if let ImgExtSingle::Webp { ref mut jakosc,.. } = self.dane_merge.rozszerzenie { *jakosc = procent; }
            }

            MergeMsg::ZdjeciaLaczenieZmianalosslessWebp => {
                if let ImgExtSingle::Webp {ref mut lossless,..} = self.dane_merge.rozszerzenie { *lossless = !*lossless}

            }

            MergeMsg::ZdjeciaLaczenieZmianaRozszerzenieFf(lejlejlej) =>  {
                if let ImgExtSingle::Ff{ref mut metoda_kompresji } = self.dane_merge.rozszerzenie {*metoda_kompresji = lejlejlej;};
            },

            MergeMsg::WybierzPlikInFotoLaczenieNazwaChanged(blob) => {
                self.dane_merge.nazwa = blob;
            }

            MergeMsg::ZdjeciaLaczenieZmianaKompresjiFfZstd(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let ImgExtSingle::Ff {
                    metoda_kompresji: ForFfKompresja::Zstd(ref mut aktualny_procent), .. } = self.dane_merge.rozszerzenie {

                        *aktualny_procent = procent;

                }

            }
            MergeMsg::ZdjeciaLaczenieZmianaKompresjiFfBzip2(procent) => {
                // self.stan_boolean_do_laczenia_zdjec.jpg_jakosc = procent;
                if let ImgExtSingle::Ff {
                    metoda_kompresji: ForFfKompresja::Bzip2(ref mut aktualny_procent), ..
                } = self.dane_merge.rozszerzenie {

                        *aktualny_procent = procent;

                }

            }
            MergeMsg::ZdjeciaLaczenieZmianaKompresjiFfXz(procent) => {

                if let ImgExtSingle::Ff {
                    metoda_kompresji: ForFfKompresja::Xz(ref mut aktualny_procent), ..
                } = self.dane_merge.rozszerzenie {

                        *aktualny_procent = procent;

                }

            }

            MergeMsg::Uruchom => {

                let dane_do_obrobki = self.dane_merge.clone();
                self.temat.temp.act_proc = Some(ActProces::Merge);
                let _ = self.update(Message::ChckStatus);
                // fn check_send<T: Send>(_: &T) {}
                // check_send(&dane_do_obrobki);

                let (tx, rx) = mpsc::channel::<LogTxMerge>(100);

                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        handle
                            .spawn(async move {
                                let _ = fn_do_laczenia_fot(dane_do_obrobki, tx).await;
                            })
                            .await
                    },
                    |_| MergeMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, MergeMsg::PostepLaczeniaFot);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));


            }
            MergeMsg::PostepLaczeniaFot(progress) => match progress {
                LogTxMerge::Start => {}
                LogTxMerge::Finito(_) => {
                    self.temat.temp.act_proc = None;
                    let _ = self.update(Message::ChckStatus);

                }
                LogTxMerge::Błąd(_) => {
                    self.temat.temp.act_proc = None;
                    let _ = self.update(Message::ChckStatus);

                }
                LogTxMerge::Sprawdzanie(_) => {}
            },
            MergeMsg::JpgProg => {
                if let ImgExtSingle::Jpg { ref mut progresywny, .. } = self
                    .dane_merge.rozszerzenie
                {
                    *progresywny = !*progresywny;
                }
            }
            MergeMsg::AvifLossyToggle => {
                if let ImgExtSingle::Avif { ref mut lossy, .. } = self
                    .dane_merge
                    .rozszerzenie
                {

                    if lossy.is_some(){
                        *lossy = None;
                    }else{
                        *lossy = Some(90);
                    }

                }
            }
            _ => {}
        }
        Task::none()
    }
}