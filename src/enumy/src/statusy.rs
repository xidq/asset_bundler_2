#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDoKompresjiPliku {
    StatusKompresjaPlikówZnalezionePliki {
        pliki: u32
    },
    StatusKompresjaPlikówPakowanie {
        aktualny: u32,
        suma: Option<u32>,
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
        current: u32,
        max: Option<u32>,
    },
    StatusDekompresjaPlikówDeszyfracja {
        current: u32,
        max: Option<u32>,
    },
    StatusDekompresjaPlikówDekompresja {
        pamięć: u64,
    },
    StatusDekompresjaPlikówRozpakowywanie {
        current: u32,
        max: Option<u32>,
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
    StatusBathKonwersjaZdjęćChecking(String),
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
