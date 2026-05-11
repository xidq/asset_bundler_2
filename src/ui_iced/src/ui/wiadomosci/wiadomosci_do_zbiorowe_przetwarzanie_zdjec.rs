use crate::ui::program::Program;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::{ActProces, BtnState};
use enumy::opcje::OptInterpolacja;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForFfKompresja};
use enumy::rozszerzenia::ext::{ImgExt, ImgExtTag};
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use zbiorowa_konwersja_zdjec::zmiana_fot::ogarnianie_foto;

fn toggle_w_vec<T: PartialEq + Clone>(vec: &mut Vec<T>, element: &T) {
    if let Some(pos) = vec.iter().position(|x| x == element) {
        vec.remove(pos);
    } else {
        vec.push(element.clone());
    }
}

impl Program {
    pub fn update_message_zbiorowe_przetwarzanie_zdjec(&mut self, msg: KonwMsg) -> Task<KonwMsg> {
        match msg {
            KonwMsg::PathInFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_konw.ścieżka_wejściowa = path.clone();

                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            if let Some(rodzic) = path.parent() {
                                self.dane_konw.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                self.dane_konw.ścieżka_wyjściowa = path;
                            }
                        } else {
                            self.dane_konw.ścieżka_wyjściowa = path;
                        }
                    }
                }

                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::PathInFolder => {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    self.dane_konw.ścieżka_wejściowa = path.clone();

                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            if let Some(rodzic) = path.parent() {
                                self.dane_konw.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                self.dane_konw.ścieżka_wyjściowa = path;
                            }
                        } else {
                            self.dane_konw.ścieżka_wyjściowa = path;
                        }
                    }

                }

                let _ = self.update(Message::ChckStatus);
            }

            KonwMsg::PathInText(s) => {
                self.dane_konw.ścieżka_wejściowa = PathBuf::from(s);
                let _ = self.update(Message::ChckStatus);

            }
            KonwMsg::PathsReset => {
                self.dane_konw.ścieżka_wejściowa = PathBuf::new();
                let _ = self.update(Message::ChckStatus);

            }
            KonwMsg::PathOutPathInBool(zdjecia_edycja_co_jest_na_out) => {
                self.zdjecia_edycja_co_jest_na_out = zdjecia_edycja_co_jest_na_out;
                let xoxo = if self.dane_konw.ścieżka_wejściowa.is_file() {
                    self.dane_konw
                        .ścieżka_wejściowa
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.dane_konw.ścieżka_wejściowa.clone())
                } else {
                    self.dane_konw.ścieżka_wejściowa.clone()
                };
                self.dane_konw.ścieżka_wyjściowa = xoxo

            }
            KonwMsg::PathOutFolder => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_konw.ścieżka_wyjściowa = path;
                }

                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::PathOutText(s) => {
                self.dane_konw.ścieżka_wyjściowa = PathBuf::from(s)
            }

            KonwMsg::WypełnienieAlpha(indeks, wartosc) => {
                match indeks {
                    0 => self.dane_konw.alfa_rgb.0 = wartosc, // Zmieniamy R
                    1 => self.dane_konw.alfa_rgb.1 = wartosc, // Zmieniamy G
                    2 => self.dane_konw.alfa_rgb.2 = wartosc, // Zmieniamy B
                    _ => {}
                }
            }

            KonwMsg::JpgJakość(procent) => {
                if let Some(ImgExt::Jpg { jakosc, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::JpgProg=> {
                if let Some(ImgExt::Jpg { progresywny, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut()
                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                {
                    *progresywny = !*progresywny;
                }

            }
            KonwMsg::JpgSampling(xx) => {
                if let Some(ImgExt::Jpg { sampling, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut()
                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *sampling = xx;
                }
            }
            KonwMsg::JpgQua(xx) => {
                if let Some(ImgExt::Jpg { quant, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut()
                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *quant = xx;
                }
            }
            KonwMsg::JpgScan(skany) => {
                if let Some(ImgExt::Jpg { scans, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                {
                    *scans = skany; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::PngKompresja(var) => {
                if let Some(ImgExt::Png { kompresja, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Png { .. }))
                {
                    *kompresja = var; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::Interpolacja(xx) => {
                let yy = match xx.as_str() {
                    "OptInterpolacja_nearest" => OptInterpolacja::Nearest,
                    "OptInterpolacja_triangle" => OptInterpolacja::Triangle,
                    "OptInterpolacja_catmull" => OptInterpolacja::CatmullRom,
                    "OptInterpolacja_gaussian" => OptInterpolacja::Gaussian,
                    "OptInterpolacja_lanczos" => OptInterpolacja::Lanczos3,
                    _ => { OptInterpolacja::Lanczos3 }
                };
                self.dane_konw.inter = yy;
            }
            KonwMsg::WebpJakość(procent) => {
                if let Some(ImgExt::Webp { jakosc, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Webp { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::WebpLossless => {
                if let Some(ImgExt::Webp { lossless, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut()
                    .find(|f| matches!(f, ImgExt::Webp { .. }))
                {
                    *lossless = !*lossless;
                }
            }
            
            KonwMsg::FfKompresja(metoda) => {
                if let Some(ImgExt::Ff { metoda_kompresji }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Ff { .. }))
                {
                    *metoda_kompresji = metoda;
                }

            }
            KonwMsg::FfKompresjaVal(var) => {
                if let Some(ImgExt::Ff { metoda_kompresji }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Ff { .. }))
                {
                    match metoda_kompresji {
                        ForFfKompresja::Zstd(v) |
                        ForFfKompresja::Bzip2(v) |
                        ForFfKompresja::Xz(v) => *v = var,

                        ForFfKompresja::Brak => {}
                    }
                }
            }
            
            KonwMsg::AvifSpeed(das) => {
                if let Some(ImgExt::Avif { speed, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                {
                    *speed = das; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::AvifLossyToggle => {
                if let Some(ImgExt::Avif { lossy, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut()
                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                {

                    if lossy.is_some(){
                        *lossy = None;
                    }else{
                        *lossy = Some(90);
                    }

                }

            }
            KonwMsg::AvifLossy(das) => {
                if let Some(ImgExt::Avif { lossy, .. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                {
                    *lossy = Some(das); // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            KonwMsg::AvifKompresja(metoda) => {
                if let Some(ImgExt::Avif { metoda_kompresji,.. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                {
                    *metoda_kompresji = metoda;
                }
            }
            KonwMsg::AvifChroma(chromchrom) => {
                if let Some(ImgExt::Avif { chroma,.. }) = self
                    .dane_konw
                    .rozszerzenia
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                {
                    *chroma = chromchrom;
                }
            }
            KonwMsg::Noising(procent) => {
                self.dane_konw.noising =
                    if procent == 0u8 { None } else { Some(procent) };
            }
            KonwMsg::Rozdzielczość(khekhe) => {
                let pozycja = self.dane_konw
                    .opcje_rozdzielczości
                    .iter()
                    .position(|x| *x == khekhe);

                match pozycja {
                    Some(index) => {
                        self.dane_konw.opcje_rozdzielczości.remove(index);
                    }
                    None => {
                        self.dane_konw.opcje_rozdzielczości.push(khekhe);

                    }
                }

                let _ = self.update(Message::ChckStatus);

            }
            
            KonwMsg::Bdepth(rozs, kolor_arc) => {
                // 1. Szukamy indeksu na podstawie znacznika
                if let Some(index) = self.dane_konw
                    .tag
                    .iter()
                    .position(|t| *t == rozs)
                {
                    // 2. Pobieramy mutowalną referencję do formatu
                    if let Some(format_danych) = self.dane_konw
                        .rozszerzenia
                        .get_mut(index)
                    {
                        // 3. Używamy jako_any(), aby móc porównać dyn z konkretnym typem w Vec
                        let kolor_any = kolor_arc.jako_any();
                        

                        match format_danych {
                            // Grupa JPG, PNG, WebP (jeśli używają tego samego typu enumu)
                            ImgExt::Jpg { bit_depth, .. } => {
                                if let Some(k) = kolor_any.downcast_ref::<BdepthJpg>() {
                                    toggle_w_vec(bit_depth, k);


                                    for xx in BdepthJpg::iter(){

                                        if !bit_depth.contains(&xx) {
                                            self.temat.btn_state.remove(xx.bath_konwersja_id());
                                        } else {
                                            self.temat.btn_state.insert(xx.bath_konwersja_id(), BtnState::Active);
                                        }

                                    }


                                }
                            }
                            ImgExt::Png { bit_depth, .. } => {
                                if let Some(k) = kolor_any.downcast_ref::<BdepthPng>() {
                                    toggle_w_vec(bit_depth, k);

                                }
                            }
                            ImgExt::Webp { bit_depth, .. } => {
                                // Próbujemy rzutować Arc na konkretny typ siedzący w tym Vec
                                if let Some(k) = kolor_any.downcast_ref::<BdepthWebp>() {
                                    toggle_w_vec(bit_depth, k);

                                }
                            }
                            ImgExt::Qoi { bit_depth, .. } => {
                                if let Some(k) = kolor_any.downcast_ref::<BdepthQoi>() {
                                    toggle_w_vec(bit_depth, k);

                                }
                            }
                            ImgExt::Avif { bit_depth, .. } => {
                                if let Some(k) = kolor_any.downcast_ref::<BdepthAvif>() {
                                    toggle_w_vec(bit_depth, k);

                                }
                            }
                            ImgExt::Tga { bit_depth, .. } => {
                                if let Some(k) = kolor_any.downcast_ref::<BdepthTga>() {
                                    toggle_w_vec(bit_depth, k);

                                }
                            }
                            _ => {}
                        }
                    }
                }
                let _ = self.update(Message::ChckStatus);

            }


            KonwMsg::Rozszerzenia(gwiazdek) => {
                let pozycja_tag = self.dane_konw
                    .tag
                    .iter()
                    .position(|t| *t == gwiazdek);
                let pozycja = self.dane_konw.rozszerzenia.iter().position(|f| {
                    matches!(
                        (f, &gwiazdek),
                        (ImgExt::Jpg {..}, ImgExtTag::Jpg) |
                        (ImgExt::Png {..}, ImgExtTag::Png) |
                        (ImgExt::Webp {..}, ImgExtTag::Webp) |
                        (ImgExt::Tga {..}, ImgExtTag::Tga) |
                        (ImgExt::Ff {..}, ImgExtTag::Ff) |
                        (ImgExt::Qoi {..}, ImgExtTag::Qoi) |
                        (ImgExt::Avif {..}, ImgExtTag::Avif)
                    )
                });
                if let Some(idx) = pozycja_tag {
                    self.dane_konw.tag.remove(idx);
                    self.temat.btn_state.insert(gwiazdek.bath_konwersja_id(), BtnState::Active);
                }

                if let Some(index) = pozycja {
                    self.dane_konw.rozszerzenia.remove(index);
                } else {
                    let (nowy_format, nowy_tag) = match gwiazdek {
                        ImgExtTag::Jpg => (ImgExt::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: Vec::from([BdepthJpg::Rgb8]),
                            sampling: ForJpgSamplingFac::R420,
                            quant: ForJpgQuant::Default,
                            scans: 4,
                        }, ImgExtTag::Jpg),
                        ImgExtTag::Png => (ImgExt::Png {
                            kompresja: 3,
                            bit_depth: Vec::from([BdepthPng::Rgb8]),
                        }, ImgExtTag::Png),

                        ImgExtTag::Webp => (ImgExt::Webp{
                            jakosc: 90,
                            lossless: false,
                            bit_depth: Vec::from([BdepthWebp::Rgb8]),
                        }, ImgExtTag::Webp),
                        ImgExtTag::Tga => (ImgExt::Tga{
                            bit_depth: Vec::from([BdepthTga::TrueColor24])
                        }, ImgExtTag::Tga),
                        ImgExtTag::Ff => (ImgExt::Ff{
                            metoda_kompresji: ForFfKompresja::Brak
                        }, ImgExtTag::Ff),
                        ImgExtTag::Qoi => (ImgExt::Qoi{
                            bit_depth: Vec::from([BdepthQoi::Color24])
                        }, ImgExtTag::Qoi),
                        ImgExtTag::Avif => (ImgExt::Avif {
                            chroma: ForAvifChroma::C420,
                            speed: 3,
                            metoda_kompresji: ForAvifKompresja::Av1,
                            lossy: Some(90),
                            bit_depth: Vec::from([BdepthAvif::Rgb10])
                        }, ImgExtTag::Avif)
                    };

                    self.dane_konw.rozszerzenia.push(nowy_format);
                    self.dane_konw.tag.push(nowy_tag.clone());

                }
                let _ = self.update(Message::ChckStatus);
            }
            KonwMsg::Uruchom => {
                let dane_do_obrobki = self.dane_konw.clone();
                self.temat.temp.act_proc = Some(ActProces::Konw);
                let _ = self.update(Message::ChckStatus);

                self.status_zmiany_fot_log = Default::default();
                dbg!(
                    "ścieżka przekazywana to: {:?}",
                    &self.dane_konw
                );

                dbg!(
                    "wysyłanko tego struct(rozszerzenia_plików_zdjęciowych): \n {}",
                    &dane_do_obrobki.rozszerzenia
                );
                let (tx, rx) = mpsc::channel::<LogTxKonw>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = ogarnianie_foto(dane_do_obrobki, tx).await;
                            })
                            .await
                    },
                    |_| KonwMsg::Nic,
                );

                let nasluchiwanie = Task::run(rx, KonwMsg::Log);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            KonwMsg::Log(progress) => {
                match progress {
                    LogTxKonw::Start => {
                        self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxKonw::Sprawdzanie(gsd) => {
                        self.status_zmiany_fot_log.msg_walidacja = gsd;
                    }
                    LogTxKonw::Rozpoczęto(
                        wartość,
                        suma,
                    ) => {
                        // println!("Update dostał procent: {}", procent); // <-- DEBUG
                        self.status_zmiany_fot_log.plik_procent = (wartość,suma);
                    }
                    LogTxKonw::FiltrowaniePlików(xoxo) => {

                        self.status_zmiany_fot_log.plik_początek = xoxo;

                    }
                    LogTxKonw::Pominięte {
                        sciezka,
                        powod,
                    } => {
                        self.log_prawe_okno.push(format!(
                            "[Obrazy] Pominięto plik z:\n{}\n z powodu: {}",
                            sciezka, powod
                        ));
                    }
                    LogTxKonw::Finito(czas) => {
                        self.status_zmiany_fot_log.msg_end =
                            format!("Zakończono w czasie: {}", czas);
                        self.temat.temp.act_proc = None;

                        let _ = self.update(Message::ChckStatus);
                    }
                    LogTxKonw::Błąd(err) => {
                        self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
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