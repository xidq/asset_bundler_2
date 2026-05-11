use std::borrow::Cow;
use std::path::PathBuf;
use iced::Element;
use iced::widget::text_input;
use iced_core::{Alignment, Color, Length};
use strum::EnumMessage;
use enumy::dane_do_przetwarzania::DaneDoObrbki;
use enumy::inne_ui::{TextInputType, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::styles::styl_text_input;

pub trait WartośćPola {
    fn as_str(&self, jezyk: &WybórJęzyka) -> Cow<'_, str>;
    fn czy_poprawne(&self) -> bool;
}

// Implementacja dla PathBuf
impl WartośćPola for PathBuf {
    fn as_str(&self, jezyk: &WybórJęzyka) -> Cow<'_, str> {
        Cow::Owned(jezyk.format_sciezek(self))
    }
    fn czy_poprawne(&self) -> bool {
        self.exists()
    }
}

// Implementacja dla &str
impl WartośćPola for &'static str {
    fn as_str(&self, _jezyk: &WybórJęzyka) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
    fn czy_poprawne(&self) -> bool {
        !self.is_empty()
    }
}
impl WartośćPola for String {
    fn as_str(&self, _jezyk: &WybórJęzyka) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
    fn czy_poprawne(&self) -> bool {
        !self.is_empty()
    }
}
pub fn tekstowe_pole_wypelniane<'a, T>(
    path_or_str: &'a T,
    wariant: &'a TextInputType,
    kolor: &'a Color,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
    szerokość: Length,
) -> Element<'a, Message>
where T: WartośćPola
{
    
    text_input(jezyk.t(wariant.get_message().unwrap_or("błąd danych")), &*path_or_str.as_str(jezyk))
        .font(jezyk.get_font())
        .on_input(|xx|Message::TextInputHandling(xx, wariant.clone()))
        .padding(10)
        .width(szerokość)
        .style(styl_text_input(path_or_str.czy_poprawne(), kolor,temat)).into()
    
}