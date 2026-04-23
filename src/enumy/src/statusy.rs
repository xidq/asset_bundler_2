#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoKompresjiPliku {
    StatusKompresjaPlikówZnalezionePliki {
        pliki: Option<i32>,
    },
    StatusKompresjaPlikówPakowanie {
        aktualny: Option<i32>,
        suma: Option<i32>,
    },
    StatusKompresjaPlikówProcesKompresji {
        procent: Option<u8>,
    },
    StatusKompresjaPlikówProcesSzyfrowania {
        procent: Option<u8>,
    },
    StatusKompresjaPlikówBłąd(String),
    StatusKompresjaPlikówZakonczono {
        czas: String,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoDekompresjiPliku {
    StatusDekompresjaPlikówZbieraniePlików {
        procent: Option<u8>,
    },
    StatusDekompresjaPlikówDeszyfracja {
        procent: Option<u8>,
    },
    StatusDekompresjaPlikówDekompresja {
        pamięć: Option<u64>,
    },
    StatusDekompresjaPlikówRozpakowywanie {
        aktualny: Option<i32>,
        suma: Option<i32>,
    },
    StatusDekompresjaPlikówBłąd(String),
    StatusDekompresjaPlikówZakończenie {
        czas: String,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoBathKonwersjaZdjęć {
    StatusBathKonwersjaZdjęćStart,
    StatusBathKonwersjaZdjęćRozpoczęto(u32, u8),
    StatusBathKonwersjaZdjęćFiltrowaniePlików(u32),
    StatusBathKonwersjaZdjęćPominiętePliki { sciezka: String, powod: String },
    StatusBathKonwersjaZdjęćKoniec(String),
    StatusBathKonwersjaZdjęćBłąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoPakowanieDds {
    StatusPakowanieDdsStart,
    StatusPakowanieDdsWtrakcie(u8),
    StatusPakowanieDdsKoniec(String),
    StatusPakowanieDdsBłąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoRozpakowanieDds {
    StatusRozpakowanieDdsStart,
    StatusRozpakowanieDdsWtrakcie(u8),
    StatusRozpakowanieDdsKoniec(String),
    StatusRozpakowanieDdsBłąd(String),
}
#[derive(Clone, Debug)]
pub enum LogTxDoŁączeniaZdjęć {
    Start,
    Koniec,
    Błąd(String),
}
