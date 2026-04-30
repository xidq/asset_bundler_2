use enumy::inne_ui::{StronyDds, WybranyFormatZdjecia};
use enumy::opcje::{OptFormatDds, OptFormatyKoloruObrazOgólny, OptKompresjaDds, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::{LogTxDoPakowanieDds, LogTxDoRozpakowanieDds};

#[derive(Debug, Clone)]
pub enum DdsMessage {
    ZmienMenuDds(StronyDds),
    PakowaniePathIn(String),
    PakowaniePathInFiles,
    PakowaniePathInFolders,
    PakowaniePathOut(String),
    PakowaniePathOutBtn,
    PakowanieFormat(OptFormatDds),
    PakowanieKompresja(OptKompresjaDds),
    PakowanieNazwa(String),
    PakowanieStart,
    PakowaniePostęp(LogTxDoPakowanieDds),
    RozpakInPath(String),
    RozpakInPathBtn,
    RozpakOutPath(String),
    RozpakOutPathBtn,
    Nic,
    RozkapExt(WybranyFormatZdjecia),
    RozpakExtDane(OptRozszerzeniaPlikówZdjęciowych),
    RozpakExtBit(OptRozszerzeniaPlikówZdjęciowych),
    RozpakBitDepth(OptRozszerzeniaPlikówZdjęciowychZnacznik, OptFormatyKoloruObrazOgólny),
    RozpakStart,
    RozpakPostęp(LogTxDoRozpakowanieDds),
}