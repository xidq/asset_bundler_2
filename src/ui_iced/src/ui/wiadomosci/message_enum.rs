use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::MergeMsg;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::BinPakMsg;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::BinUnpakMsg;
use enumy::implementacje::ElementyDropdown;
use enumy::inne_ui::{ActProces, ButtonType, DropdownType, SliderType, TextInputType, UiPods};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::Event;
use std::fmt::Debug;
use std::sync::Arc;
// use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {

    ZmienWariant(UiPods),

    // Akcje
    LogDodaj(String),
    EventOccurred(Event),

    DevZmienJezyk(WybórJęzyka),

    Dropdown(Arc<dyn ElementyDropdown + Send + Sync>, DropdownType),
    TextInputHandling(String, TextInputType),
    Przyciski(ButtonType),
    Startujemy(ActProces),
    Slidery(SliderType, i32),
    ChckStatus,
    Nic,
    
    DoNothingxD(bool),
    DoNothingU8xD(u8),
    DoNothingStringxD(String),
    
    UsuńLogi,

    WysylkaDanychDoObrobkiZdjec,
    InitLogStartowy,

    BtnToggleActive(&'static str),

    ŁączenieZdjęć(MergeMsg),
    Dds(DdsMsg),
    ZbiorowePrzetwarzanieZdjęć(KonwMsg),
    PakowanieBinarki(BinPakMsg),
    RozpakowanieBinarki(BinUnpakMsg),
    UruchomProces(ActProces),
    UpdateProcesUiBtn,
    UpdateProcesUiBtnPost,
    InitUstawienia,
    DevZmienKolory(UiPods,String, u8),
    DevResetUstawien,
}