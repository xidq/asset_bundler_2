use crate::ui::podstrony::merging::lewy::strona_wyboru;
use crate::ui::podstrony::merging::rozszerzenia::rozszerzenia;
use crate::ui::podstrony::merging::sciezki::sciezki;
use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::oddzielacze::oddzielacz_pionowy;
use enumy::dane_do_przetwarzania::DaneMerge;
use enumy::inne_ui::{UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row};
use iced::Element;
use iced_core::Length;

pub fn merge_view<'a>(
    dane: &'a DaneMerge,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {






    let prawa = match temat.temp.act_window {
        UiPods::Merge => {sciezki(dane,&temat.kolory.laczenie,jezyk,temat).spacing(15).padding(15)}
        UiPods::MergeExt => {rozszerzenia(dane,&temat.kolory.laczenie,jezyk,temat).padding(15)}
        _ => {Column::new()}
    }




        .width(Length::FillPortion(2));

    Row::new().push(strona_wyboru(jezyk,temat).spacing(15).padding(15).width(Length::FillPortion(1))).push(oddzielacz_pionowy()).push(prawa).into()

}