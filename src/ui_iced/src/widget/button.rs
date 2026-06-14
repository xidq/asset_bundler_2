use crate::ui::wiadomosci::message_enum::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::MergeMsg;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use crate::widget::colors_n_stuff::WYSOKOSC_CZCIONEK_PRZYCISKI;
use crate::widget::styles::{styl_hint, styl_przycisków};
use enumy::inne_ui::{ActProces, BtnState, ButtonType, PrzyciskiGlowneMenu, UiPods, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, text, tooltip};
use iced::Element;
use iced_core::{Color, Length};
use std::borrow::Cow;
use std::sync::Arc;
use strum::EnumMessage;

pub trait ElementarnyTekst<'a>: Into<Element<'a, Message>> {
    fn element(self, jezyk: &WybórJęzyka) -> Element<'a,Message>;
}

impl<'a> ElementarnyTekst<'a> for &'a str{
    fn element(self, jezyk: &WybórJęzyka) -> Element<'a, Message> {
        text(
            jezyk.t(self)
        )
            .font(jezyk.get_font())
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center()
            .into()

    }
}
impl<'a> ElementarnyTekst<'a> for Element<'a, Message> {
    fn element(self, _jezyk: &WybórJęzyka) -> Element<'a, Message> {
        self
    }
}
#[allow(clippy::too_many_arguments)]
pub fn przycisk<'a, T>(
    element: T,
    typ: ButtonType,
    szerokość: Length,
    wysokość: Length,
    kolor: &'a Color, 
    stan: &'a BtnState,
    jezyk: &'a WybórJęzyka, 
    temat:&'a UstawieniaThemeWsio
) -> Element<'a,Message>
where T:ElementarnyTekst<'a>+'a
{


    let xxx =
        button(
            element.element(jezyk),
        )
            .padding(5)
            .height(wysokość)
            .width(szerokość)
            .on_press(Message::Przyciski(typ.clone()))
            .style(styl_przycisków(stan, kolor, temat));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
            jezyk.t(typ.get_message().unwrap_or("err_msg")),
            tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }
}
pub fn przycisk_rozszerzenia<'a>(
    rozszerzenie: ImgExtTag,
    typ: ButtonType,
    kolor: &'a Color,
    stan: &'a BtnState,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a,Message>{

    let msg = match typ{
        ButtonType::KonwRozszerzenia => {Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::Rozszerzenia(rozszerzenie.clone()))}
        ButtonType::MergeRozszerzenia => {Message::ŁączenieZdjęć(MergeMsg::Rozszerzenia(rozszerzenie.clone()))}
        ButtonType::DdsRozszerzenia => {Message::Dds(DdsMsg::Rozszerzenia(rozszerzenie.clone()))}
        _ => {Message::Nic}
    };

    
    let xxx =
        button(
            text(
                rozszerzenie.małe()
            )
                .font(jezyk.get_font())
                .height(Length::Fill)
                .width(Length::Fill)
                .center()

        )
            .height(50.)
            .width(Length::FillPortion(1))
            .on_press(msg)
            .style(styl_przycisków(stan, kolor, temat));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
                rozszerzenie.duże(),
                tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }
}
pub(crate) trait LabelText{
    fn element(self, jezyk: &WybórJęzyka) -> Cow<'static, str>;
}

impl LabelText for String{
    fn element(self, _jezyk: &WybórJęzyka) -> Cow<'static, str>{
        Cow::Owned(self)
    }
}
impl LabelText for &'static str {
    fn element(self, jezyk: &WybórJęzyka) -> Cow<'static, str> {
        Cow::Borrowed(jezyk.t(self))
    }
}

pub fn pole_tekstowe_przycisku<'a, T>(label: T, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message>
where T:LabelText
{
    text(
        label.element(jezyk)
    )
        .color(Color{ a:temat.obecny_theme.mid, ..Color::WHITE})
        .font(jezyk.get_font())
        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
        .width(Length::Fill)
        .center()
        .into()

    
}

pub fn przycisk_startu<'a>(
    proces: &'a ActProces,
    kolor: &'a Color,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message>{
    let status = match proces{
        ActProces::BinPak => {&temat.temp.start_btn_status.bin_pak}
        ActProces::BinUnpak => {&temat.temp.start_btn_status.bin_unpak}
        ActProces::DdsPak => {&temat.temp.start_btn_status.dds_pak}
        ActProces::DdsUnpak => {&temat.temp.start_btn_status.dds_unpak}
        ActProces::Merge => {&temat.temp.start_btn_status.laczenie}
        ActProces::Konw => {&temat.temp.start_btn_status.konwersja}
    };
    let msg = if *status == BtnState::Active { Some(Message::Startujemy(proces.clone())) } else { None };
    let label = match status{
        BtnState::Active => {"mgt_btn_ready"}
        BtnState::Disabled => {"mgt_btn_busy_processing_other"}
        BtnState::Processing => {"mgt_btn_busy_processing"}
        BtnState::LackData => {"mgt_btn_gib_data"}
        // _ => {"mgt_proces_error"}
    };
    let xxx =
        button(
            pole_tekstowe_przycisku( label, jezyk, temat)
        )
            .height(50.)
            .style(styl_przycisków(status, kolor, temat))
            .on_press_maybe(msg)
            .width(Length::FillPortion(1));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
                jezyk.t(&("hint_".to_owned() + label)),
                tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }
}

pub fn przycisk_glowne_menu<'a>(przycisk: PrzyciskiGlowneMenu, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message>{
    let (status, msg, kolor, hint, label) = match przycisk{
        PrzyciskiGlowneMenu::Binarka => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::BinPak) | Some(ActProces::BinUnpak)) {
                    &BtnState::Processing
                } else if matches!(temat.temp.act_window, UiPods::BinPak | UiPods::BinUnpak) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::BinPak,
            &temat.kolory.binarka,
            "hint_ui_main_btn_binary",
            "ui_main_btn_binary"
            ),
        PrzyciskiGlowneMenu::Konwersja => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::Konw)) {
                    &BtnState::Processing
                } else if matches!(temat.temp.act_window, UiPods::KonwPath | UiPods::KonwRes | UiPods::KonwExt | UiPods::KonwEtc) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::KonwPath,
            &temat.kolory.konwersja,
            "hint_ui_main_btn_conversion",
            "ui_main_btn_conversion"
            ),
        PrzyciskiGlowneMenu::Łączenie => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::Merge)) {
                    &BtnState::Processing
                } else if matches!(temat.temp.act_window, UiPods::Merge | UiPods::MergeExt) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::Merge,
            &temat.kolory.laczenie,
            "hint_ui_main_btn_merge",
            "ui_main_btn_merge"
            ),
        PrzyciskiGlowneMenu::Dds => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::DdsPak) | Some(ActProces::DdsUnpak)) {
                    &BtnState::Processing
                } else if matches!(temat.temp.act_window, UiPods::DdsUnpak | UiPods::DdsPak | UiPods::DdsExt) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::DdsPak,
            &temat.kolory.dds,
            "hint_ui_main_btn_dds",
            "ui_main_btn_dds"
            ),
        PrzyciskiGlowneMenu::Ustawienia => (
            {
                if matches!(temat.temp.act_window, UiPods::Ustawienia) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::Ustawienia,
            &temat.kolory.ustawienia,
            "hint_ui_main_btn_settings",
            "ui_main_btn_settings"
            ),
    };
    let xx =
        button(pole_tekstowe_przycisku( label, jezyk, temat))
            .height(50.)
            .style(styl_przycisków(status, kolor, temat))
            .on_press(Message::ZmienWariant(msg))
            .width(Length::FillPortion(1));

    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xx,
                jezyk.t(hint),
                tooltip::Position::Bottom
            ).style(styl_hint(temat)).into()
        }
        false => {xx.into()}
    }
}
pub fn przycisk_podmenu<'a>(przycisk: UiPods, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message>{
    let (status, msg, kolor, hint, label) = match przycisk{
        UiPods::BinPak => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::BinPak)) {
                        &BtnState::Processing
                    } else if matches!(temat.temp.act_window, UiPods::BinPak) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::BinPak,
            &temat.kolory.binarka,
            "hint_ui_menu_bin_pack",
            "ui_menu_bin_pack"
            ),
        UiPods::BinUnpak => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::BinUnpak)) {
                        &BtnState::Processing
                    } else if matches!(temat.temp.act_window, UiPods::BinUnpak) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::BinUnpak,
            &temat.kolory.binarka,
            "hint_ui_menu_bin_unpack",
            "ui_menu_bin_unpack"
            ),
        UiPods::KonwPath => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::Konw)) {
                        &BtnState::Processing
                    } else if matches!(temat.temp.act_window, UiPods::KonwPath) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::KonwPath,
            &temat.kolory.konwersja,
            "hint_ui_conversion_paths",
            "ui_conversion_paths"
            ),
        UiPods::KonwExt => (
            {
                if matches!(temat.temp.act_window, UiPods::KonwExt) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::KonwExt,
            &temat.kolory.konwersja,
            "hint_ui_conversion_extensions",
            "ui_conversion_extensions"
            ),
        UiPods::KonwRes => (
            {
                if matches!(temat.temp.act_window, UiPods::KonwRes) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::KonwRes,
            &temat.kolory.konwersja,
            "hint_ui_conversion_resolutions",
            "ui_conversion_resolutions"
            ),
        UiPods::KonwEtc => (
            {
                if matches!(temat.temp.act_window, UiPods::KonwEtc) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::KonwEtc,
            &temat.kolory.konwersja,
            "hint_ui_conversion_rest",
            "ui_conversion_rest"
            ),
        UiPods::DdsPak => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::DdsPak)) {
                        &BtnState::Processing
                    } else if matches!(temat.temp.act_window, UiPods::DdsPak) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::DdsPak,
            &temat.kolory.dds,
            "hint_ui_dds_pack",
            "ui_dds_pack"
            ),
        UiPods::DdsUnpak => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::DdsUnpak)) {
                        &BtnState::Processing
                    } else if matches!(temat.temp.act_window, UiPods::DdsUnpak) {
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    }
            },
            UiPods::DdsUnpak,
            &temat.kolory.dds,
            "hint_ui_dds_unpack",
            "ui_dds_unpack"
            ),
        UiPods::DdsExt => (
            {
                if matches!(temat.temp.act_window, UiPods::DdsExt) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::DdsExt,
            &temat.kolory.dds,
            "hint_ui_gen_extensions",
            "ui_gen_extensions"
        ),
        UiPods::Merge => (
            {
                if matches!(temat.temp.act_proc, Some(ActProces::Merge)) {
                    &BtnState::Processing
                } else if matches!(temat.temp.act_window, UiPods::Merge) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::Merge,
            &temat.kolory.laczenie,
            "hint_ui_merge",
            "ui_merge"
        ),
        UiPods::MergeExt => (
            {
                if matches!(temat.temp.act_window, UiPods::MergeExt) {
                    &BtnState::Active
                } else {
                    &BtnState::Disabled
                }
            },
            UiPods::MergeExt,
            &temat.kolory.laczenie,
            "hint_ui_gen_extensions",
            "ui_gen_extensions"
        ),
        _ => unreachable!()
    };
    let xx =
        button(pole_tekstowe_przycisku( label, jezyk, temat))
            .height(50.)
            .style(styl_przycisków(status, kolor, temat))
            .on_press(Message::ZmienWariant(msg))
            .width(Length::FillPortion(1));

    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xx,
                jezyk.t(hint),
                tooltip::Position::Bottom
            ).style(styl_hint(temat)).padding(10).into()
        }
        false => {xx.into()}
    }
}
pub fn btn_bdepth_konwersja<'a, T>(
    bdepth: T,
    stan: &'a BtnState,
    jezyk: &'a WybórJęzyka,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>
where T: BitDepth + Clone
{

    let xxx =
        button(
            text(
                bdepth.label_min()
            )
                .font(jezyk.get_font())
                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                .width(Length::Fill).center()
        )
            .padding(5)
            .width(Length::FillPortion(1))
            .height(50.)
            .on_press(Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::Bdepth(Arc::new(bdepth.clone()))))
            .style(styl_przycisków(stan, kolor, temat));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
                bdepth.label_max(),
                tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }

}
pub fn btn_bdepth_merge<'a, T>(
    bdepth: T,
    stan: &'a BtnState,
    jezyk: &'a WybórJęzyka,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>
where T: BitDepth + Clone
{

    let xxx =
        button(
            text(
                bdepth.label_min()
            )
                .font(jezyk.get_font())
                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                .width(Length::Fill).center()
        )
            .padding(5)
            .width(Length::FillPortion(1))
            .height(50.)
            .on_press(Message::ŁączenieZdjęć(MergeMsg::Bdepth(bdepth.format(), Arc::new(bdepth.clone()))))
            .style(styl_przycisków(stan, kolor, temat));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
                bdepth.label_max(),
                tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }

}
pub fn btn_bdepth_dds<'a, T>(
    bdepth: T,
    stan: &'a BtnState,
    jezyk: &'a WybórJęzyka,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>
where T: BitDepth + Clone
{

    let xxx =
        button(
            text(
                bdepth.label_min()
            )
                .font(jezyk.get_font())
                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                .width(Length::Fill).center()
        )
            .padding(5)
            .width(Length::FillPortion(1))
            .height(50.)
            .on_press(Message::Dds(DdsMsg::Bdepth(bdepth.format(), Arc::new(bdepth.clone()))))
            .style(styl_przycisków(stan, kolor, temat));


    match temat.ustawienia.halp_menu{
        true => {
            tooltip(
                xxx,
                bdepth.label_max(),
                tooltip::Position::Bottom
            )
                .style(styl_hint(temat))
                .padding(10)
                .into()
        }
        false => {
            xxx.into()
        }
    }

}

pub fn btn_rozdzielczosci<'a>(
    rozdzielczosc: Rozdzielczości,
    jezyk: &'a WybórJęzyka,
    stan: &'a BtnState,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio,
) -> Element<'a, Message> {


        let xxx = button(
            text(
                jezyk.t(rozdzielczosc.get_message().unwrap_or("błąd danych rozdzielczosc"))
            ).font(jezyk.get_font()).width(Length::Fill).height(Length::Fill).center()
        )
            .on_press(Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::Rozdzielczość(rozdzielczosc)))
            .height(100.).width(Length::FillPortion(1))
            .style(styl_przycisków(stan, kolor, temat)
        );
    if temat.ustawienia.halp_menu{
        tooltip(
            xxx,
            jezyk.t(rozdzielczosc.get_detailed_message().unwrap_or("błąd danych rozdzielczosc detailed")),
            tooltip::Position::Bottom
        )
            .style(styl_hint(temat))
            .padding(10)
            .into()
    } else {
        xxx.into()
    }

}