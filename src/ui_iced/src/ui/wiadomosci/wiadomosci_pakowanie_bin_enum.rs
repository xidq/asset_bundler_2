use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::statusy::LogTxBinPak;

#[derive(Debug, Clone)]
pub enum BinPakMsg {
    InputPath,
    OutputPath,
    Kompresja(OptKompresjaPlikówPoziomKompresjiZstd),
    Filtr(OptKompresjaPlikówFiltracjaPlików),
    Uruchom,
    Nic,
    LogProcesu(LogTxBinPak),
}