use std::sync::Arc;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForFfKompresja};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::statusy::LogTxKonw;

#[derive(Debug,Clone)]
pub enum KonwMsg {
    Rozszerzenia(ImgExtTag),
    JpgJakość(u8),
    JpgProg,
    JpgSampling(ForJpgSamplingFac),
    JpgQua(ForJpgQuant),
    JpgScan(u8),
    PngKompresja(u8),
    WypełnienieAlpha(u8, u16),
    Rozdzielczość(Rozdzielczości),
    AvifSpeed(i32),
    WebpLossless,
    AvifLossyToggle,
    AvifLossy(u8),
    AvifKompresja(ForAvifKompresja),
    AvifChroma(ForAvifChroma),
    WebpJakość(u8),
    FfKompresja(ForFfKompresja),
    FfKompresjaVal(u8),
    PathInText(String),
    PathOutText(String),
    PathOutPathInBool(bool),
    Noising(u8),
    Uruchom,
    Log(LogTxKonw),
    PathInFile,
    Bdepth(Arc<dyn BitDepth>),
    Nic,
    PathInFolder,
    PathsReset,
    PathOutFolder,
    Interpolacja(String),
    StabilizacjaDanychRozszerzen,
    ExifToggle,
}