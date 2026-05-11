use iced::widget::{button, text, Button};
use iced_core::{Color, Length};
use enumy::inne_ui::{ActProces, UstawieniaThemeWsio};
use crate::ui::podmenu::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::wiadomosci::message_ui::Message;

pub fn btn_uniwersalny_tak<'a>(
    label: String, 
    id: &'static str,
    font: iced::Font,
    msg: Message,
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio  
) -> Button<'a, Message>{

    button(
        
        text(label)
            
            .color(KOLOR_CZCIONKI_SREDNI)
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
        
    )
        
        .on_press(msg)
        
        .style(
            
            styl_przycisków(
                id,
                kolor,
                temat
            )
            
        )
    
}