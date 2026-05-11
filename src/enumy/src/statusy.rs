#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxBinPak {
    StatusZnaleziono {
        pliki: u32
    },
    Pakowanie
    {
        aktualny: u32,
        suma: Option<u32>,
    },
    Kompresja {
        aktualny: u32,
        suma: Option<u32>,
    },
    Szyfrowanie {
        aktualny: u32,
        suma: Option<u32>,
    },
    Błąd(String),
    Finito {
        czas: String,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxBinUnpak {
    Zbieranie {
        current: u32,
        max: Option<u32>,
    },
    Deszyfracja {
        current: u32,
        max: Option<u32>,
    },
    Dekompresja {
        pamięć: u64,
    },
    Rozpakowywanie {
        current: u32,
        max: Option<u32>,
    },
    Błąd(String),
    Finito {
        czas: String,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxKonw {
    Start,
    Sprawdzanie(String),
    Rozpoczęot(u32, Option<u32>),
    FiltrowaniePlików(Option<u32>),
    Pominięte { sciezka: String, powod: String },
    Finito(String),
    Błąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsPak {
    Start,
    Pending(u8),
    Finito(String),
    Błąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsUnpak {
    Start,
    Pending(u8),
    Finito(String),
    Błąd(String),
}
#[derive(Clone, Debug)]
pub enum LogTxDoŁączeniaZdjęć {
    Start,
    Finito,
    Błąd(String),
}
