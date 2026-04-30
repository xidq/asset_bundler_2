use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::ActProces;
use enumy::opcje::{AvifChroma, AvifMetodaKompresji, JpgQuant, JpgSamplingFac, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptInterpolacja, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::channel::mpsc;
use iced::Task;
use std::path::PathBuf;
use zbiorowa_konwersja_zdjec::zmiana_fot::ogarnianie_foto;

impl Program {
    pub fn update_message_zbiorowe_przetwarzanie_zdjec(&mut self, msg: ZbiorowePrzetwarzanieZdjęćMessage) -> Task<ZbiorowePrzetwarzanieZdjęćMessage> {
        match msg {
            ZbiorowePrzetwarzanieZdjęćMessage::PathInFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    // 1. Zawsze aktualizujemy ścieżkę wejściową
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = path.clone();

                    // 2. Jeśli flaga auto-wyjścia jest włączona, ustawiamy ścieżkę wyjściową
                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            // Jeśli to plik, wyjście ustawiamy na folder, w którym on jest
                            if let Some(rodzic) = path.parent() {
                                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                // Jeśli nie ma rodzica (np. root), ustawiamy samą ścieżkę
                                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                            }
                        } else {
                            // Jeśli to już folder, po prostu kopiujemy
                            self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                        }
                    }
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PathInFolder => {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder()
                {
                    // 1. Zawsze aktualizujemy ścieżkę wejściową
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = path.clone();

                    // 2. Jeśli flaga auto-wyjścia jest włączona, ustawiamy ścieżkę wyjściową
                    if self.zdjecia_edycja_co_jest_na_out {
                        if path.is_file() {
                            // Jeśli to plik, wyjście ustawiamy na folder, w którym on jest
                            if let Some(rodzic) = path.parent() {
                                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = rodzic.to_path_buf();
                            } else {
                                // Jeśli nie ma rodzica (np. root), ustawiamy samą ścieżkę
                                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                            }
                        } else {
                            // Jeśli to już folder, po prostu kopiujemy
                            self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                        }
                    }
                }
                }

            ZbiorowePrzetwarzanieZdjęćMessage::PathInText(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::from(s);
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PathsReset => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::new();
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PathOutPathInBool(zdjecia_edycja_co_jest_na_out) => {
                self.zdjecia_edycja_co_jest_na_out = zdjecia_edycja_co_jest_na_out;
                let xoxo = if self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.is_file() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                        .ścieżka_wejściowa
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.clone())
                } else {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.clone()
                };
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = xoxo
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PathOutFolder => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PathOutText(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = PathBuf::from(s)
            }

            ZbiorowePrzetwarzanieZdjęćMessage::WypełnienieAlpha(indeks, wartosc) => {
                match indeks {
                    0 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.0 = wartosc, // Zmieniamy R
                    1 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.1 = wartosc, // Zmieniamy G
                    2 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.2 = wartosc, // Zmieniamy B
                    _ => {}
                }

            }

            ZbiorowePrzetwarzanieZdjęćMessage::JpgJakość(procent) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::JpgProg => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { progresywny, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *progresywny = !*progresywny;
                }

            }
            ZbiorowePrzetwarzanieZdjęćMessage::JpgSampling(xx) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg {sampling, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *sampling = xx;
                }

            }
            ZbiorowePrzetwarzanieZdjęćMessage::JpgQua(xx) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { quant, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *quant = xx;
                }

            }
            ZbiorowePrzetwarzanieZdjęćMessage::JpgScan(skany) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { scans, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    *scans = skany; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PngKompresja(var) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Png { .. }))
                {
                    *kompresja = var; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::Interpolacja(xx) => {
                
                let yy = match xx.as_str() {
                    "OptInterpolacja_nearest" => OptInterpolacja::Nearest,
                    "OptInterpolacja_triangle" => OptInterpolacja::Triangle,
                    "OptInterpolacja_catmull" => OptInterpolacja::CatmullRom,
                    "OptInterpolacja_gaussian" => OptInterpolacja::Gaussian,
                    "OptInterpolacja_lanczos" => OptInterpolacja::Lanczos3,
                    _ => {OptInterpolacja::Lanczos3}
                };
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.inter = yy;
            }
            ZbiorowePrzetwarzanieZdjęćMessage::WebpJakość(procent) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Webp { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::WebpLossless => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp { lossless, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Webp { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)
                    *lossless = !*lossless;
                }

            }
            ZbiorowePrzetwarzanieZdjęćMessage::TgaBdepth(fdvcx) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Tga { .. }))
                {
                    // 2. Szukamy, czy ten konkretny bit_depth już jest w wektorze TGA
                    let pozycja = bit_depth.iter().position(|x| *x == fdvcx);

                    match pozycja {
                        // Jeśli jest – usuwamy go (odznaczamy)
                        Some(index) => {
                            bit_depth.remove(index);
                        }
                        // Jeśli go nie ma – dodajemy go (zaznaczamy)
                        None => {
                            bit_depth.push(fdvcx);
                        }
                    }
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::FfKompresja(metoda) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Ff { .. }))
                {
                    *metoda_kompresji = metoda;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::FfKompresjaVal(var) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Ff { .. }))
                {
                    match metoda_kompresji {
                        OptMetodaKompresjiZdjecia::Zstd(v) |
                        OptMetodaKompresjiZdjecia::Bzip2(v) |
                        OptMetodaKompresjiZdjecia::Xz(v) => *v = var,

                        OptMetodaKompresjiZdjecia::Brak => {}
                    }
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::QoiBdepth(bdepth) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Qoi { .. }))
                {
                    let pozycja = bit_depth.iter().position(|x| *x == bdepth);

                    match pozycja {
                        // Jeśli jest – usuwamy go (odznaczamy)
                        Some(index) => {
                            bit_depth.remove(index);
                        }
                        // Jeśli go nie ma – dodajemy go (zaznaczamy)
                        None => {
                            bit_depth.push(bdepth);
                        }
                    }
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifBdepth(fdvcx) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { bit_depth, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    // 2. Szukamy, czy ten konkretny bit_depth już jest w wektorze TGA
                    let pozycja = bit_depth.iter().position(|x| *x == fdvcx);

                    match pozycja {
                        // Jeśli jest – usuwamy go (odznaczamy)
                        Some(index) => {
                            bit_depth.remove(index);
                        }
                        // Jeśli go nie ma – dodajemy go (zaznaczamy)
                        None => {
                            bit_depth.push(fdvcx);
                        }
                    }
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifSpeed(das) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { speed, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *speed = das; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifLossyToggle => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { lossy, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut()
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    // 2. lossless jest tutaj mutowalną referencją (&mut bool)

                    if lossy.is_some(){
                        *lossy = None;
                    }else{
                        *lossy = Some(90);
                    }
                }

            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifLossy(das) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { lossy, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *lossy = Some(das); // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifKompresja(metoda) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { metoda_kompresji,.. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *metoda_kompresji = metoda;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::AvifChroma(chromchrom) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { chroma,.. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *chroma = chromchrom;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::Noising(procent) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.noising =
                    if procent == 0u8 { None } else { Some(procent) };
            }
            ZbiorowePrzetwarzanieZdjęćMessage::Rozdzielczość(khekhe) => {
                // 1. Szukamy pozycji konkretnej rozdzielczości w wektorze
                let pozycja = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .opcje_rozdzielczości
                    .iter()
                    .position(|x| *x == khekhe);

                match pozycja {
                    // 2. Jeśli jest – usuwamy (odznaczamy przycisk)
                    Some(index) => {
                        self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.opcje_rozdzielczości.remove(index);
                    }
                    // 3. Jeśli nie ma – dodajemy (zaznaczamy przycisk)
                    None => {
                        self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.opcje_rozdzielczości.push(khekhe);
                    }
                }
            }

            ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(rozs, kolor) => {
                // 1. Szukamy pozycji rozszerzenia w wektorze tagów
                if let Some(index) = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .tag
                    .iter()
                    .position(|t| *t == rozs)
                {
                    // 2. Pobieramy mutowalną referencję do danych tego formatu (korzystając z tego samego indeksu)
                    if let Some(format_danych) = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                        .rozszerzenia_plików_zdjęciowych
                        .get_mut(index)
                    {
                        // 3. Wyciągamy wektor bit_depth z odpowiedniego wariantu
                        let bit_depth_vec = match format_danych {
                            OptRozszerzeniaPlikówZdjęciowych::Jpg { bit_depth, .. } => Some(bit_depth),
                            OptRozszerzeniaPlikówZdjęciowych::Png { bit_depth, .. } => Some(bit_depth),
                            OptRozszerzeniaPlikówZdjęciowych::Webp { bit_depth, .. } => Some(bit_depth),
                            _ => None, // TGA i QOI ignorujemy, bo mają inne enumy kolorów
                        };

                        // 4. Jeśli wariant ma bit_depth, robimy Toggle
                        if let Some(vec) = bit_depth_vec {
                            if let Some(pos) = vec.iter().position(|x| *x == kolor) {
                                vec.remove(pos); // Jeśli kolor już był -> usuń go
                            } else {
                                vec.push(kolor); // Jeśli go nie było -> dodaj go
                            }
                        }
                    }
                }
            }


            ZbiorowePrzetwarzanieZdjęćMessage::Rozszerzenia(gwiazdek) => {
                let pozycja_tag = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .tag
                    .iter()
                    .position(|t| *t == gwiazdek);
                let pozycja = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter()
                    .position(|f| matches! ((f, &gwiazdek),
                        (OptRozszerzeniaPlikówZdjęciowych::Jpg {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg) |
                        (OptRozszerzeniaPlikówZdjęciowych::Png {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Png) |
                        (OptRozszerzeniaPlikówZdjęciowych::Webp {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp) |
                        (OptRozszerzeniaPlikówZdjęciowych::Tga { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga) |
                        (OptRozszerzeniaPlikówZdjęciowych::Ff { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff) |
                        (OptRozszerzeniaPlikówZdjęciowych::Qoi { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi) |
                        (OptRozszerzeniaPlikówZdjęciowych::Avif { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif)
                    ));
                if let Some(idx) = pozycja_tag {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.tag.remove(idx);
                }

                if let Some(index) = pozycja {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.rozszerzenia_plików_zdjęciowych.remove(index);
                } else {
                    let (nowy_format, nowy_tag) = match gwiazdek {
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg => (OptRozszerzeniaPlikówZdjęciowych::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                            sampling: JpgSamplingFac::R420,
                            quant: JpgQuant::Default,
                            scans: 4,
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg),
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Png => (OptRozszerzeniaPlikówZdjęciowych::Png {
                            kompresja: 3,
                            bit_depth: vec![OptFormatyKoloruObrazOgólny::B8],
                        },OptRozszerzeniaPlikówZdjęciowychZnacznik::Png),

                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp => (OptRozszerzeniaPlikówZdjęciowych::Webp{
                            jakosc: 90,
                            lossless: false,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp),
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga => (OptRozszerzeniaPlikówZdjęciowych::Tga{
                            bit_depth: Vec::from([OptFormatyKoloruObrazuTga::TrueColor24])
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga),
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff => (OptRozszerzeniaPlikówZdjęciowych::Ff{
                            metoda_kompresji: OptMetodaKompresjiZdjecia::Brak
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff),
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi => (OptRozszerzeniaPlikówZdjęciowych::Qoi{
                            bit_depth: Vec::from([OptFormatyKoloruObrazuQoi::Color24])
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi),
                        OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif => (OptRozszerzeniaPlikówZdjęciowych::Avif {
                            chroma: AvifChroma::C420,
                            speed: 3,
                            metoda_kompresji: AvifMetodaKompresji::Av1,
                            lossy: Some(90),
                            bit_depth: Vec::from([OptFormatyKoloruObrazuAvif::B10])
                        }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif)
                    };
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.rozszerzenia_plików_zdjęciowych.push(nowy_format);
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.tag.push(nowy_tag);
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::Uruchom => {
                let dane_do_obrobki = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.clone();
                self.temat.temp.aktywny_proces = ActProces::KonwersjaZdjęć;
                self.status_zmiany_fot_log = Default::default();
                dbg!(
                    "ścieżka przekazywana to: {:?}",
                    &self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                );

                dbg!(
                    "wysyłanko tego struct(rozszerzenia_plików_zdjęciowych): \n {}",
                    &dane_do_obrobki.rozszerzenia_plików_zdjęciowych
                );
                let (tx, rx) = mpsc::channel::<LogTxDoBathKonwersjaZdjęć>(100);

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
                    |_| ZbiorowePrzetwarzanieZdjęćMessage::Nic,
                );

                let nasluchiwanie = Task::run(rx, ZbiorowePrzetwarzanieZdjęćMessage::Log);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            ZbiorowePrzetwarzanieZdjęćMessage::Log(progress) => {
                match progress {
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćStart => {
                        self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćChecking(gsd) => {
                        self.status_zmiany_fot_log.msg_walidacja = gsd;
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćRozpoczęto(
                        _,
                        procent,
                    ) => {
                        // println!("Update dostał procent: {}", procent); // <-- DEBUG
                        self.status_zmiany_fot_log.plik_procent = procent;
                        self.status_zmiany_fot_log.msg_proces = format!("{}", procent);
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćFiltrowaniePlików(
                        xoxo,
                    ) => {
                        if xoxo > 0 {
                            self.status_zmiany_fot_log.plik_początek =
                                format!("Zebrano {} plików", xoxo);
                        }
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćPominiętePliki {
                        sciezka,
                        powod,
                    } => {
                        self.log_prawe_okno.push(format!(
                            "[Obrazy] Pominięto plik z:\n{}\n z powodu: {}",
                            sciezka, powod
                        ));
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćKoniec(czas) => {
                        self.status_zmiany_fot_log.msg_end =
                            format!("Zakończono w czasie: {}", czas);
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(err) => {
                        self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        self.temat.temp.aktywny_proces = ActProces::Żodyn;
                    }
                }
            }

            _ => {}
        }
        Task::none()

    }
}