#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxBinPak {
    StatusZnaleziono {
        pliki: u32
    },
    SPakowanie {
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
    StatusBathKonwersjaZdjęćStart,
    StatusBathKonwersjaZdjęćChecking(String),
    StatusBathKonwersjaZdjęćRozpoczęto(u32, Option<u32>),
    StatusBathKonwersjaZdjęćFiltrowaniePlików(Option<u32>),
    StatusBathKonwersjaZdjęćPominiętePliki { sciezka: String, powod: String },
    StatusBathKonwersjaZdjęćKoniec(String),
    StatusBathKonwersjaZdjęćBłąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsPak {
    StatusPakowanieDdsStart,
    StatusPakowanieDdsWtrakcie(u8),
    StatusPakowanieDdsKoniec(String),
    StatusPakowanieDdsBłąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsUnpak {
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
