use crate::ui::program::czcionki::{KOLORCRIMSONGLORY, KOLORTŁA};
use crate::ui::program::{DevToolsMenu, Message, WybórJęzyka};
use crate::{styl_menu_pick, styl_pick_list};
use iced::widget::{pick_list, text};
use iced::{Border, Color, Element, };
use iced_core::{Shadow, Vector};

pub fn dev_tools(opcje:&DevToolsMenu) -> Element<'_, Message> {
    let aktualnie_wybrany_jezyk = match opcje {
        DevToolsMenu::UstawieniaJęzyka { jezyk } => Some(*jezyk),
        _ => None, // Na wypadek gdyby dev_opcje były innym wariantem
    };

    iced::widget::column![
        text("Wybierz język").size(22),
        pick_list(
                    &WybórJęzyka::ALL[..], // Przekazujemy CAŁĄ listę (slice)
                    aktualnie_wybrany_jezyk,
                    Message::DevZmienJezyk // Akcja po kliknięciu
                )
                .placeholder("Wybierz...")
                .width(200)
                .style(styl_pick_list!(KOLORCRIMSONGLORY, (0.1,0.1,0.1)))
                .menu_style(styl_menu_pick!(KOLORCRIMSONGLORY, KOLORTŁA)),
    ]        .spacing(15)
        .into()
}