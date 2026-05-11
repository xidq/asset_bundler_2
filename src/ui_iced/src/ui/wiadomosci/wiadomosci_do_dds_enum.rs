use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::{ForDds, ForDdsKompresja, ForFfKompresja};
use enumy::rozszerzenia::rozszerzenia::{ImgExt, ImgExtTag};
use enumy::statusy::{LogTxDoPakowanieDds, LogTxDoRozpakowanieDds};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum DdsMsg {
    PakowaniePathIn(String),
    PakowaniePathInFiles,
    PakowaniePathInFolders,
    PakowaniePathOut(String),
    PakowaniePathOutBtn,
    PakowanieFormat(ForDds),
    PakowanieKompresja(ForDdsKompresja),
    PakowanieNazwa(String),
    PakowanieStart,
    PakowaniePostęp(LogTxDoPakowanieDds),
    RozpakInPath(String),
    RozpakInPathBtn,
    RozpakOutPath(String),
    RozpakOutPathBtn,
    Nic,

    ZdjeciaLaczenieZmianaRozszerzenieFf(ForFfKompresja),
    RozpakExtDane(ImgExt),
    RozpakExtBit(ImgExt),
    Bdepth(ImgExtTag, Arc<dyn BitDepth>),

    Rozszerzenia(ImgExtTag),
    RozpakStart,
    RozpakPostęp(LogTxDoRozpakowanieDds),
    JpgProg,
    WebpLoss,
    AvifLoss,
}