use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::{ForDds, ForDdsKompresja, ForFfKompresja};
use enumy::rozszerzenia::ext::{ImgExt, ImgExtTag};
use enumy::statusy::{LogTxDdsPak, LogTxDdsUnpak};
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
    PakowaniePostęp(LogTxDdsPak),
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
    RozpakPostęp(LogTxDdsUnpak),
    JpgProg,
    WebpLoss,
    AvifLoss,
    Dx9,
}