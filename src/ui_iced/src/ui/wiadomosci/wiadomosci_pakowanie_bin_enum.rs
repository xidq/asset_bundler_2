use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::statusy::LogTxDoKompresjiPliku;

#[derive(Debug, Clone)]
pub enum PakowanieBinarkiMessage{
    InputPath,
    InputPathText(String),
    OutputPath,
    OutputPathText(String),
    Nazwa(String),
    Kompresja(OptKompresjaPlikówPoziomKompresjiZstd),
    Filtr(OptKompresjaPlikówFiltracjaPlików),
    Uruchom,
    Nic,
    LogProcesu(LogTxDoKompresjiPliku),
}