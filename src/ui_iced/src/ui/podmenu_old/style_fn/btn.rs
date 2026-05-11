use std::sync::{Arc, RwLock, RwLockReadGuard};
use crate::ui::podmenu::kolory::{
    KOLOR_CZCIONKI_JASNY, KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
};
use enumy::inne_ui::{ActProces, BtnState, UstawieniaThemeWsio};
use iced::widget::button;
use iced::{Border, Color};
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians};

pub fn styl_przycisków<'a>(
    stan: &'a BtnState,
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style + 'a {
    move |_theme, _status| {

        // Sprawdzanie czy ten id ma jakiś status
        // Nadrzędnym jest hovered


        if let button::Status::Hovered = _status {

            return btn_hover(kolor, temat);

        } else {

            match stan {

                BtnState::Processing => btn_processing(kolor, temat),
                BtnState::Active => btn_aktywny(kolor, temat),
                _ => btn_active(kolor, temat),

            }
        }

    }
}
fn btn_hover(kolor: &Color, temat: &UstawieniaThemeWsio) -> button::Style {
    button::Style {
        background: Some(
            Color {
                a: temat.obecny_theme.low, ..*kolor // Twoja przezroczystość dla hover
            }
                .into(),
        ),
        text_color: KOLOR_CZCIONKI_JASNY,
        border: Border {
            radius: 5.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }

}
fn btn_processing(kolor: &Color, temat: &UstawieniaThemeWsio) -> button::Style {
    let mut stopsy = [None; 8];
    stopsy[0] = Some(ColorStop {
        offset: 0.5, // Koniec koloru w połowie wysokości
        color: Color {
            a: 0.0,
            ..*kolor
        },
    });
    stopsy[1] = Some(ColorStop {
        offset: 1.0, // Start na górze
        color: Color {
            a: temat.obecny_theme.mid,
            ..*kolor
        },
    });

    button::Style {
        background: Some(Background::Gradient(
            iced_core::gradient::Gradient::Linear(Linear {
                angle: Radians(0.0),
                stops: stopsy,
            }),
        )),
        text_color: KOLOR_CZCIONKI_JASNY,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 5.0.into(),
        },
        ..button::Style::default()
    }
}
fn btn_aktywny(kolor: &Color, temat: &UstawieniaThemeWsio) -> button::Style {
    let mut stopsy = [None; 8];
    stopsy[0] = Some(ColorStop {
        offset: 0.0, // Start na dole
        color: Color {
            a: temat.obecny_theme.mid,
            ..*kolor
        },
    });
    stopsy[1] = Some(ColorStop {
        offset: 0.5, // Koniec koloru w połowie wysokości
        color: Color {
            a: 0.0,
            ..*kolor
        },
    });
    button::Style {
        background: Some(Background::Gradient(
            iced_core::gradient::Gradient::Linear(Linear {
                angle: Radians(0.0),
                stops: stopsy,
            }),
        )),
        text_color: Color { a:temat.tekst.hi, ..temat.tekst.kolor },
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 5.0.into(),
        },
        ..button::Style::default()
    }
}
fn btn_active(kolor: &Color, temat: &UstawieniaThemeWsio) -> button::Style {
    button::Style {
        background: Some(
            Color {
                a: temat.obecny_theme.min,
                ..temat.obecny_theme.kolor
            }
                .into(),
        ),
        text_color: Color { a:temat.tekst.mid, ..temat.tekst.kolor }, // Lekko przygaszony tekst
        border: Border {
            radius: 5.0.into(),
            width: 1.0,
            color: KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
            // ..Border::default()
        },
        ..button::Style::default()
    }
}
