use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;

#[derive(Debug,Clone)]
pub enum ZbiorowePrzetwarzanieZdjęćMessage{
    ZdjecieEdycjaZmianaWybraneToggleRozszerzenie(OptRozszerzeniaPlikówZdjęciowychZnacznik),
    ZdjeciaEdycjaZmianaJakosciJpg(u8),
    ZdjeciaEdycjaZmianaProgresJpg,
    ZdjeciaEdycjaZmianaKompresjiPng(u8),
    ZdjeciaEdycjaZmianaKolorAlpha(u8, u16),
    DopasujRozdzielczosci(OptRozdzielczościObrazów),
    ZdjeciaEdycjaZmianaBitDepthTga(OptFormatyKoloruObrazuTga),
    ZdjeciaEdycjaZmianaLosslessWebp,
    ZdjeciaEdycjaZmianaJakosciWebp(u8),
    ZdjeciaEdycjaZmianaKompresjaFF(OptMetodaKompresjiZdjecia),
    ZdjeciaEdycjaZmianaKompresjaWartoscFF(u8),
    ZdjeciaEdycjaZmianaBitDepthQoi(OptFormatyKoloruObrazuQoi),
    ZdjeciaZmienFolderInPathChanged(String),
    ZdjeciaZmienFolderOutPathChanged(String),
    ZdjeciaZmienFolderOutPathTenSam(bool),
    ZdjeciaEdycjaZmianaZaszumiania(u8),
    WysylkaDanychDoObrobkiZdjec,
    PostepEdycjaFot(LogTxDoBathKonwersjaZdjęć),
    WybierzPlikInFotoEdycjaPakowanie,
    ZdjecieEdycjaZmianaWybraneToggleKolor(OptRozszerzeniaPlikówZdjęciowychZnacznik,OptFormatyKoloruObrazOgólny),
    Nic,
    WybierzFolderInFotoEdycjaPakowanie,
    ResetujStanWejsciowychSciezekEdycjaFoto,
    WybierzFolderOutFotoEdycjaPakowanie,
}