use std::path::PathBuf;
use futures::channel::mpsc;
use iced::Task;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::inne_ui::CheckActiveProcess;
use enumy::opcje::{AvifChroma, AvifMetodaKompresji, FolderCzyPlik, JpgQuant, JpgSamplingFac, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use zbiorowa_konwersja_zdjec::zmiana_fot::ogarnianie_foto;
use crate::ui::program::Program;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

impl Program {
    pub fn update_message_zbiorowe_przetwarzanie_zdjec(&mut self, msg: ZbiorowePrzetwarzanieZdjęćMessage) -> Task<ZbiorowePrzetwarzanieZdjęćMessage> {
        match msg {
            ZbiorowePrzetwarzanieZdjęćMessage::WybierzPlikInFotoEdycjaPakowanie => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::WybierzFolderInFotoEdycjaPakowanie => {
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

            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderInPathChanged(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::from(s);
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ResetujStanWejsciowychSciezekEdycjaFoto => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::new();
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderOutPathTenSam(zdjecia_edycja_co_jest_na_out) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::WybierzFolderOutFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderOutPathChanged(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = PathBuf::from(s)
            }

            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKolorAlpha(indeks, wartosc) => {
                match indeks {
                    0 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.0 = wartosc, // Zmieniamy R
                    1 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.1 = wartosc, // Zmieniamy G
                    2 => self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.alfa_rgb.2 = wartosc, // Zmieniamy B
                    _ => {}
                }

            }

            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJakosciJpg(procent) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaProgresJpg => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJpgSampling(xx) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJpgQua(xx) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJpgScans(skany) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { scans, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                    *scans = skany; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjiPng(var) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Png { .. }))
                {
                    *kompresja = var; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJakosciWebp(procent) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Webp { .. }))
                {
                    *jakosc = procent; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaLosslessWebp => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaBitDepthTga(fdvcx) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaFF(metoda) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Ff { .. }))
                {
                    *metoda_kompresji = metoda;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaWartoscFF(var) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaBitDepthQoi(bdepth) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaBitDepthAvif(fdvcx) => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifSpeed(das) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { speed, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *speed = das; // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifToggleLossy => {
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
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifLossy(das) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { lossy, .. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *lossy = Some(das); // Jeśli znaleziono, aktualizujemy wartość
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifKompresja(metoda) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { metoda_kompresji,.. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *metoda_kompresji = metoda;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifChroma(chromchrom) => {
                if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { chroma,.. }) = self
                    .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter_mut() // Tworzymy mutowalny iterator
                    .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                    *chroma = chromchrom;
                }
            }
            ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaZaszumiania(procent) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.noising =
                    if procent == 0u8 { None } else { Some(procent) };
            }
            ZbiorowePrzetwarzanieZdjęćMessage::DopasujRozdzielczosci(khekhe) => {
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

            ZbiorowePrzetwarzanieZdjęćMessage::ZdjecieEdycjaZmianaWybraneToggleKolor(rozs, kolor) => {
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


            ZbiorowePrzetwarzanieZdjęćMessage::ZdjecieEdycjaZmianaWybraneToggleRozszerzenie(gwiazdek) => {
                let pozycja_tag = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .tag
                    .iter()
                    .position(|t| *t == gwiazdek);
                let pozycja = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                    .rozszerzenia_plików_zdjęciowych
                    .iter()
                    .position(|f| match (f, &gwiazdek) {
                        (OptRozszerzeniaPlikówZdjęciowych::Jpg {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Png {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Png) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Webp {..}, OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Tga { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Ff { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Qoi { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi) => true,
                        (OptRozszerzeniaPlikówZdjęciowych::Avif { .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif) => true,
                        _ => false
                    });
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
            ZbiorowePrzetwarzanieZdjęćMessage::WysylkaDanychDoObrobkiZdjec => {
                let dane_do_obrobki = self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.clone();
                self.checker_bool_status_procesow = CheckActiveProcess::ProcessKonwersjaZdjęć;
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

                let nasluchiwanie = Task::run(rx, ZbiorowePrzetwarzanieZdjęćMessage::PostepEdycjaFot);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            ZbiorowePrzetwarzanieZdjęćMessage::PostepEdycjaFot(progress) => {
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
                        self.checker_bool_status_procesow = CheckActiveProcess::ProcessŻodyn;
                    }
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(err) => {
                        self.status_zmiany_fot_log.błąd = format!("Błąd: {}", err);
                        self.checker_bool_status_procesow = CheckActiveProcess::ProcessŻodyn;
                    }
                }
            }

            _ => {}
        }
        Task::none()

    }
}