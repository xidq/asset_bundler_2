use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;
use enumy::inne_ui::{UiPodstrony, WybraneOknoEdycjiZdjęć};
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptKompresjaPlikówFiltracjaPlików, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów};
use enumy::statusy::{LogTxDoBathKonwersjaZdjęć, LogTxDoDekompresjiPliku, LogTxDoKompresjiPliku};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::Event;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    ZmienWariant(UiPodstrony),

    // Akcje
    LogDodaj(String),
    EventOccurred(Event),

    DevZmienJezyk(WybórJęzyka),

    Nic,
    
    DoNothingxD(bool),
    DoNothingU8xD(u8),
    DoNothingStringxD(String),
    
    UsuńLogi,

    WysylkaDanychDoObrobkiZdjec,

    InitLogStartowy,

    ŁączenieZdjęć(ŁączenieZdjęćMessage),
    Dds(DdsMessage),
    ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage),
    PakowanieBinarki(PakowanieBinarkiMessage),
    RozpakowanieBinarki(RozpakowanieBinarkiMessage),
}