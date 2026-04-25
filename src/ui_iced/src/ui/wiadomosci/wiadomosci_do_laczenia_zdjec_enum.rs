use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
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
    ZdjeciaLaczenieZmianaWybraneRozszerzenie(OptRozszerzeniaPlikówZdjęciowychZnacznik),
    ZdjeciaLaczenieZmianaJakosciJpg(u8),
    ZdjeciaLaczenieZmianaRozszerzeniePng(OptFormatyKoloruObrazOgólny),
    ZdjeciaLaczenieZmianaKompresjiPng(u8),
    ZdjeciaLaczenieZmianaRozszerzenieWebp(OptFormatyKoloruObrazOgólny),
    ZdjeciaLaczenieZmianaJakosciWebp(u8),
    ZdjeciaLaczenieZmianaRozszerzenieTga(OptFormatyKoloruObrazuTga),
    ZdjeciaLaczenieZmianaRozszerzenieFf(OptMetodaKompresjiZdjecia),

    ZdjeciaLaczenieZmianaKompresjiFfZstd(u8),
    ZdjeciaLaczenieZmianaKompresjiFfBzip2(u8),
    ZdjeciaLaczenieZmianaKompresjiFfXz(u8),
    ZdjeciaLaczenieZmianaRozszerzenieQoi(OptFormatyKoloruObrazuQoi),
    WysylkaDanychDoLaczeniaZdjec,
    PostepLaczeniaFot(LogTxDoŁączeniaZdjęć),
    ZdjeciaLaczenieZmianalosslessWebp,
    WybierzPlikInFotoLaczenieNazwaChanged(String),
    Nic,
}