
pub trait Logi: Send + 'static {
    // Wspólne dla wszystkich
    fn start() -> Self;
    fn status(_msg: (&'static str,String)) -> Self;
    fn blad(msg: String) -> Self;
    fn finito(msg: Option<String>) -> Self;


    fn postep_liczbowy(_aktualny: u32, _max: Option<u32>) -> Option<Self> where Self: Sized { None }
    fn postep_ilosc(_postep: Option<u32>) -> Option<Self> where Self: Sized { None }
    fn znalezione_pliki(_pliki: u32) -> u32 {0_u32}
    fn pakowanie(_pliki:u32, _suma: Option<u32>) -> Option<Self> where Self: Sized { None }
    fn kompresja(_pliki:u32, _suma: Option<u32>) -> Option<Self> where Self: Sized { None }
    fn szyfrowanie(_pliki:u32, _suma: Option<u32>) -> Option<Self> where Self: Sized { None }


    fn nazwa() -> &'static str;
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxBinPak {
    Start,
    StatusZnaleziono {
        pliki: u32
    },
    Sprawdzanie((&'static str,String)),
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
    Finito (String),
}
impl Logi for LogTxBinPak {
    fn start() -> Self { Self::Start }
    fn status(msg: (&'static str,String)) -> Self { Self::Sprawdzanie(msg) }
    fn blad(msg: String) -> Self { Self::Błąd(msg) }
    fn finito(msg: Option<String>) -> Self { Self::Finito(msg.unwrap_or_default()) }
    fn znalezione_pliki(pliki: u32) -> u32{
        pliki
    }
    fn pakowanie(pliki:u32, suma: Option<u32>) -> Option<Self> { Some(LogTxBinPak::Pakowanie { aktualny: pliki, suma }) }
    fn kompresja(pliki:u32, suma: Option<u32>) -> Option<Self> { Some(LogTxBinPak::Kompresja { aktualny: pliki, suma }) }
    fn szyfrowanie(pliki:u32, suma: Option<u32>) -> Option<Self> { Some(LogTxBinPak::Szyfrowanie { aktualny: pliki, suma }) }
    fn nazwa() -> &'static str {
        "[Log Binary packing]"
    }
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
    Sprawdzanie((&'static str,String)),
    Rozpoczęto(u32, Option<u32>),
    FiltrowaniePlików(Option<u32>),
    Pominięte { sciezka: String, powod: String },
    Finito(String),
    Błąd(String),
}
impl Logi for LogTxKonw {
    fn start() -> Self { Self::Start }
    fn status(msg: (&'static str,String)) -> Self { Self::Sprawdzanie(msg) }
    fn blad(msg: String) -> Self { Self::Błąd(msg) }
    fn finito(msg: Option<String>) -> Self { Self::Finito(msg.unwrap_or_default()) }

    // Konwersja używa u32, więc mapujemy to tutaj
    fn postep_liczbowy(aktualny: u32, max: Option<u32>) -> Option<Self> {
        Some(Self::Rozpoczęto(aktualny, max))
    }
    fn nazwa() -> &'static str {
        "[Log Conversion]"
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsPak {
    Start,
    PostępPreOperacji(u32,Option<u32>),
    PostępSkładania(u32,Option<u32>),
    Finito(String),
    Błąd(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogTxDdsUnpak {
    Start,
    Sprawdzanie((&'static str,String)),
    Pending(u8),
    Finito(String),
    Błąd(String),
}
impl Logi for LogTxDdsUnpak {
    fn start() -> Self { Self::Start }
    fn status(msg: (&'static str,String)) -> Self { Self::Sprawdzanie(msg) }
    fn blad(msg: String) -> Self { Self::Błąd(msg) }
    fn finito(msg: Option<String>) -> Self { Self::Finito(msg.unwrap_or_default()) }
    fn nazwa() -> &'static str {
        "[Log Dds unpack]"
    }
}
#[derive(Clone, Debug)]
pub enum LogTxMerge {
    Start,
    Sprawdzanie((&'static str,String)),
    Finito(String),
    Błąd(String),
}
impl Logi for LogTxMerge {
    fn start() -> Self { Self::Start }
    fn status(msg: (&'static str,String)) -> Self { Self::Sprawdzanie(msg) }
    fn blad(msg: String) -> Self { Self::Błąd(msg) }
    fn finito(msg: Option<String>) -> Self { Self::Finito(msg.unwrap_or_default()) }
    fn nazwa() -> &'static str {
        "[Log Merging]"
    }
}