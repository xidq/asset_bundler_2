use std::path::PathBuf;
use iced::Event;
use enumy::inne_ui::{StronyDds, WybraneOknoEdycjiZdjęć};
use enumy::opcje::{OptFormatDds, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptKompresjaDds, OptKompresjaPlikówFiltracjaPlików, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów, OptUIWariantPodstrony};
use enumy::statusy::{LogTxDoBathKonwersjaZdjęć, LogTxDoDekompresjiPliku, LogTxDoKompresjiPliku, LogTxDoŁączeniaZdjęć};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    ZmienWariant(OptUIWariantPodstrony),
    // Aktualizacja pól Eksportu
    PakowaniePathChanged(String),
    PakowanieOutPathChanged(String),
    StatusDekompresjaPlikówDekompresjaPlikPathChanged(String),
    StatusDekompresjaPlikówDekompresjaOutPathChanged(String),
    NazwaPaczkiChanged(String),
    ResetLogPakowanie,
    // Akcje
    UruchomProcesPakowania,
    PostepPakowania(LogTxDoKompresjiPliku),
    PakowanieZakonczone(Result<(), String>),
    ProcesZakonczony(Result<(), String>),
    PostepRozpakowania(LogTxDoDekompresjiPliku),
    LogDodaj(String),
    EventOccurred(Event),

    DevZmienJezyk(WybórJęzyka),
    WybierzFolderInPakowanie,
    WybierzFolderOutPakowanie,
    OptKompresjaPlikówPoziomKompresjiZstdChanged(String),
    PoziomInterpolacjiChanged(String),
    OptKompresjaPlikówPoziomKompresjiZstdSearch(String),
    FiltracjaChanged(OptKompresjaPlikówFiltracjaPlików),
    Nic,
    None,
    FilterChanged(String),
    FilterSearch(String),
    WybierzFolderOutExPakowanie,
    WybierzPlikExPakowanie,
    UruchomProcesRozpakowania,
    ResetLogRozpakowania,
    ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć),
    ZdjeciaZmienPlikInPathChanged(String),
    ZdjeciaZmienFolderInPathChanged(String),
    ZdjeciaZmienFolderOutPathChanged(String),
    ZdjeciaZmienFolderOutPathTenSam(bool),
    DoNothingxD(bool),
    DoNothingU8xD(u8),
    DoNothingStringxD(String),
    WybierzPlikInFotoEdycjaPakowanie,
    WybierzFolderInFotoEdycjaPakowanie,
    WybierzFolderOutFotoEdycjaPakowanie,
    ResetujStanWejsciowychSciezekEdycjaFoto,
    ZdjeciaEdycjaZmianaJakosciJpg(u8),
    ZdjeciaEdycjaZmianaProgresJpg,
    ZdjeciaEdycjaZmianaKolorJpg(OptFormatyKoloruObrazOgólny),
    ZdjeciaEdycjaZmianaWybranyJpg,
    ZdjeciaEdycjaZmianaKompresjiPng(u8),
    ZdjeciaEdycjaZmianaWybranyPng,
    ZdjeciaEdycjaZmianaFiltraPng,
    ZdjeciaEdycjaZmianaKolorPng(u8, u16),
    ZdjeciaEdycjaZmianaBitDepthPng(OptFormatyKoloruObrazOgólny),
    ZdjeciaEdycjaZmianaAlphaPng,
    DopasujRozdzielczosci(OptRozdzielczościObrazów),
    UsuńLogi,
    ZdjeciaEdycjaZmianaZaszumiania(u8),
    WysylkaDanychDoObrobkiZdjec,
    PostepEdycjaFot(LogTxDoBathKonwersjaZdjęć),
    ZdjeciaEdycjaZmianaWybranyTga,
    ZdjeciaEdycjaZmianaBitDepthTga(OptFormatyKoloruObrazuTga),
    ZdjeciaEdycjaZmianaLosslessWebp,
    ZdjeciaEdycjaZmianaWybranyWebp,
    ZdjeciaEdycjaZmianaJakosciWebp(u8),
    ZdjeciaEdycjaZmianaKolorWebp(OptFormatyKoloruObrazOgólny),
    InitLogStartowy,
    ZdjeciaEdycjaZmianaWybranyFF,
    ZdjeciaEdycjaZmianaKompresjaFF(OptMetodaKompresjiZdjecia),
    ZdjeciaEdycjaZmianaKompresjaWartoscFF(u8),
    ZdjeciaEdycjaZmianaBitDepthQoi(OptFormatyKoloruObrazuQoi),
    ZdjeciaEdycjaZmianaWybranyQoi,
    ŁączenieZdjęć(ŁączenieZdjęćMessage),
    Dds(DdsMessage),
    ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage),
}