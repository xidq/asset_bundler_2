use enumy::opcje::OptFormatyKoloruObrazOgólny;
use enumy::statusy::LogTxDoŁączeniaZdjęć;

#[derive(Debug, Clone)]
pub enum ŁączenieZdjęćMessage {
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
    ZdjeciaLaczenieZmianaWybranyFf,
    ZdjeciaLaczenieZmianaWybranyQoi,
    ZdjeciaLaczenieZmianaWybranyTga,
    ZdjeciaLaczenieZmianaWybranyWebp,
    ZdjeciaLaczenieZmianaWybranyPng,
    ZdjeciaLaczenieZmianaWybraneRozszerzenie(String),
    ZdjeciaLaczenieZmianaJakosciJpg(u8),
    ZdjeciaLaczenieZmianaRozszerzeniePng(OptFormatyKoloruObrazOgólny),
    ZdjeciaLaczenieZmianaKompresjiPng(u8),
    ZdjeciaLaczenieZmianaRozszerzenieWebp(OptFormatyKoloruObrazOgólny),
    ZdjeciaLaczenieZmianaJakosciWebp(u8),
    WysylkaDanychDoLaczeniaZdjec,
    PostepLaczeniaFot(LogTxDoŁączeniaZdjęć),
    ZdjeciaLaczenieZmianalosslessWebp,
    WybierzPlikInFotoLaczenieNazwaChanged(String),
    Nic,
}