use enumy::statusy::LogTxDoDekompresjiPliku;

#[derive(Debug, Clone)]
pub enum RozpakowanieBinarkiMessage{
    InputFile,
    InputFileText(String),
    OutputPath,
    OutputPathText(String),
    Uruchom,
    Nic,
    LogProcesu(LogTxDoDekompresjiPliku),
    InputPath,
}