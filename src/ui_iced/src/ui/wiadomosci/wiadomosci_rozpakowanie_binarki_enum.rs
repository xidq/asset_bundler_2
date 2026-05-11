use enumy::statusy::LogTxBinUnpak;

#[derive(Debug, Clone)]
pub enum BinUnpakMsg {
    InputFile,
    OutputPath,
    Uruchom,
    Nic,
    LogProcesu(LogTxBinUnpak),
    InputPath,
}