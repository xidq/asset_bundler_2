use enumy::statusy::LogTxDoDekompresjiPliku;

#[derive(Debug, Clone)]
pub enum BinUnpakMsg {
    InputFile,
    OutputPath,
    Uruchom,
    Nic,
    LogProcesu(LogTxDoDekompresjiPliku),
    InputPath,
}