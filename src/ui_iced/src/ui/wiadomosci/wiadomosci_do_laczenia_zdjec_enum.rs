use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::statusy::LogTxDoŁączeniaZdjęć;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MergeMsg {
    WybierzPlikInFotoLaczenieR,
    WybierzPlikInFotoLaczenieG,
    WybierzPlikInFotoLaczenieB,
    WybierzPlikInFotoLaczenieA,
    WybierzPlikInFotoLaczenieRPathChanged(String),
    WybierzPlikInFotoLaczenieGPathChanged(String),
    WybierzPlikInFotoLaczenieBPathChanged(String),
    WybierzPlikInFotoLaczenieAPathChanged(String),
    WybierzPlikInFotoLaczenieOutPathChanged(String),
    WybierzFolderOutFotoLaczenie,
    Bdepth(ImgExtTag, Arc<dyn BitDepth>),
    Rozszerzenia(ImgExtTag),
    ZdjeciaLaczenieZmianaWybranyFf,
    ZdjeciaLaczenieZmianaWybranyQoi,
    ZdjeciaLaczenieZmianaWybranyTga,
    ZdjeciaLaczenieZmianaWybranyWebp,
    ZdjeciaLaczenieZmianaWybranyPng,
    ZdjeciaLaczenieZmianaWybraneRozszerzenie(ImgExtTag),
    ZdjeciaLaczenieZmianaJakosciJpg(u8),
    ZdjeciaEdycjaZmianaJpgSampling(ForJpgSamplingFac),
    ZdjeciaEdycjaZmianaJpgQua(ForJpgQuant),
    ZdjeciaEdycjaZmianaJpgScans(u8),
    ZdjeciaLaczenieZmianaKompresjiPng(u8),
    ZdjeciaLaczenieZmianaJakosciWebp(u8),
    ZdjeciaLaczenieZmianaRozszerzenieFf(ForFfKompresja),

    ZdjeciaLaczenieZmianaKompresjiFfZstd(u8),
    ZdjeciaLaczenieZmianaKompresjiFfBzip2(u8),
    ZdjeciaLaczenieZmianaKompresjiFfXz(u8),
    Uruchom,
    PostepLaczeniaFot(LogTxDoŁączeniaZdjęć),
    ZdjeciaLaczenieZmianalosslessWebp,
    WybierzPlikInFotoLaczenieNazwaChanged(String),
    Nic,
    JpgProg,
    AvifLossyToggle,
}