use iced::widget::{button, container, pick_list, slider, text_input};
use iced::widget::button::{Catalog, Status, Style};
use iced_core::{Background, Border, Color, Radians, Shadow, Vector};
use iced_core::gradient::{ColorStop, Linear};
use enumy::inne_ui::{BtnState, RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::widget::colors_n_stuff::{KOLOR_CZCIONKI_JASNY, KOLOR_ERROR, KOLOR_OBRAMOWANIA_NIE_AKTYWNY};


pub fn styl_przycisków<'a>(
    stan: &'a BtnState,
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style + 'a {
    move |_theme, _status| {

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
        offset: 0.5,
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
        text_color: Color { a:temat.tekst.mid, ..temat.tekst.kolor },
        border: Border {
            radius: 5.0.into(),
            width: 1.0,
            color: KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
            // ..Border::default()
        },
        ..button::Style::default()
    }
}

pub fn styl_hint(
    temat: &UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {

        container::Style {
            text_color: Some(Color { a: temat.tekst.hi, ..Color::BLACK }),


            background: Some(iced::Color { a: temat.obecny_theme.hi, ..temat.kolory.hint }.into()),

            border: iced::Border {
                color: iced::Color { a: temat.obecny_theme.hi, ..Color::BLACK },
                width: 1.0,
                radius: 6.0.into(), // Zaokrąglone rogi
            },

            shadow: iced::Shadow::default(),
            snap: false,
        }

    }

}
pub fn styl_pick_list<'a>(
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::pick_list::Status| {



        use iced::widget::pick_list;

        let tlo_bazowe = temat.obecny_theme.bground;
        let kolor_akcentuu =
            Color{a:temat.obecny_theme.mid, ..*kolor_akcentu };
        let kolor_border = Color{a:temat.obecny_theme.low, ..*kolor_akcentu };

        pick_list::Style {
            text_color: Color {
                a: temat.tekst.hi,
                ..temat.tekst.kolor
            },
            placeholder_color: Color {
                a: temat.obecny_theme.mid,
                ..temat.tekst.kolor
            },
            handle_color: temat.obecny_theme.kolor,
            background: tlo_bazowe.into(),
            border: Border {
                radius: 5.0.into(),
                width: if matches!(_status, pick_list::Status::Hovered) {
                    1.1
                } else {
                    1.0
                },
                color: if matches!(_status, pick_list::Status::Hovered) {
                    kolor_akcentuu
                } else {
                    kolor_border
                },
            },
        }
    }
}

pub fn styl_menu_pick<'a>(
    kolor_zaznaczenia: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> iced::overlay::menu::Style + 'a {
    move |_theme| {
        let tlo_menu = temat.obecny_theme.bground;
        let kolor_zaznaczenia = Color{a: temat.obecny_theme.mid, ..*kolor_zaznaczenia};

        iced::overlay::menu::Style {
            text_color: Color {
                a: temat.tekst.mid,
                ..temat.tekst.kolor
            },
            background: tlo_menu.into(),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: Color {
                    a: temat.obecny_theme.low,
                    ..temat.obecny_theme.kolor
                },
            },

            selected_text_color: temat.obecny_theme.kolor,
            selected_background: kolor_zaznaczenia.into(),
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
        }
    }
}

pub fn styl_text_input<'a>(
    checker:bool,
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
        use iced::widget::text_input;
        use iced::{Border, Color};

        let kolor_error = Color{a: 0.2, ..KOLOR_ERROR};
        let tlo_bazowe = temat.obecny_theme.bground;
        // let kolor_akcentu = Color::from_rgb(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2);

        // Sprawdzamy stany z uwzględnieniem struktury wariantu Focused
        let (czy_hover, czy_focused, czy_disabled) = match _status {
            text_input::Status::Hovered => (true, false, false),
            text_input::Status::Focused { is_hovered } => (is_hovered, true, false),
            text_input::Status::Disabled => (false, false, true),
            _ => (false, false, false),
        };

        text_input::Style {
            // Jeśli wyłączony, lekko przyciemniamy tło
            background: if czy_disabled {
                Color {
                    a: temat.obecny_theme.mid,
                    ..tlo_bazowe
                }
                    .into()
            } else {
                tlo_bazowe.into()
            },

            value: if czy_disabled {
                Color {
                    a: temat.obecny_theme.low,
                    ..temat.obecny_theme.kolor
                }
            } else {
                Color {
                    a: temat.obecny_theme.hi,
                    ..temat.obecny_theme.kolor
                }
            },

            placeholder: Color {
                a: temat.obecny_theme.low,
                ..temat.obecny_theme.kolor
            },

            border: Border {
                radius: 5.0.into(),
                // Świecimy ramką jeśli hover LUB focused
                width: if (czy_hover || czy_focused) && !czy_disabled {
                    1.0
                } else {
                    0.5
                },
                color: if !checker{
                    kolor_error
                }else if (czy_hover || czy_focused) && !czy_disabled {
                    *kolor_akcentu
                } else {
                    Color {
                        a: temat.obecny_theme.low,
                        ..temat.obecny_theme.kolor
                    }
                },
            },

            icon: Color{a:temat.tekst.hi,..temat.tekst.kolor},
            selection: Color {
                a: temat.obecny_theme.low,
                ..*kolor_akcentu
            },
        }
    }
}

pub fn styl_progress_bar<'a>(
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> iced::widget::progress_bar::Style + 'a {
    move |_theme: &iced::Theme| {
        use iced::widget::progress_bar;
        use iced::{Border, Color};

        let wypelnienie = Color{ a: temat.obecny_theme.mid, ..*kolor_akcentu};
        let kolor_border = Color{ a: temat.obecny_theme.low, ..*kolor_akcentu};;

        progress_bar::Style {
            // Tło całego paska (to pod spodem)
            background: temat.obecny_theme.bground.into(),
            // To, co się przesuwa (postęp)
            bar: wypelnienie.into(),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: kolor_border,
            },
        }
    }
}

pub fn styl_sliderów<'a>(
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style + 'a {
    move |_theme, _status| slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Color{a:temat.obecny_theme.mid, ..*kolor}.into(), // Aktywne (lewo) - Czerwony
                Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
            ),
            width: 4.,
            border: Border {
                color: Color{a:temat.obecny_theme.low, ..*kolor},
                width: 1.0,
                radius: 5.0.into(),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Color{a: temat.obecny_theme.hi, ..*kolor}.into(), // Czerwona kropka
            border_width: 1.0,
            border_color: Color::BLACK,
        },
    }
}
pub fn styl_kontenera<'a>(
    warunek: bool,
    rodzaj:RodzajeContainer,
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> container::Style + 'a {
    move |_theme| {


        if warunek {
            let mut stopsy = [None; 8];

            match rodzaj{
                RodzajeContainer::Góra => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.5,
                        color: Color {
                            a: 0.0,
                            ..*kolor
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 1.0,
                        color: Color {
                            a: temat.obecny_theme.mid,
                            ..*kolor
                        },
                    });

                }
                RodzajeContainer::Dół => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.5,
                        color: Color {
                            a: 0.0,
                            ..*kolor
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            a: temat.obecny_theme.mid,
                            ..*kolor
                        },
                    });
                }
                RodzajeContainer::Oba => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            a: temat.obecny_theme.mid,
                            ..*kolor
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 0.4,
                        color: Color {
                            a: 0.0,
                            ..*kolor
                        },
                    });
                    stopsy[2] = Some(ColorStop {
                        offset: 0.6,
                        color: Color {
                            a: 0.0,
                            ..*kolor
                        },
                    });
                    stopsy[3] = Some(ColorStop {
                        offset: 1.0,
                        color: Color {
                            a: temat.obecny_theme.mid,
                            ..*kolor
                        },
                    });

                }
            };



            container::Style {
                background: Some(Background::Gradient(
                    iced_core::gradient::Gradient::Linear(Linear {
                        angle: Radians(0.0),
                        stops: stopsy,
                    }),
                )),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                ..container::Style::default()
            }
        } else {
            container::Style {
                background: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 }.into()),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color { a: temat.obecny_theme.low, ..temat.obecny_theme.kolor },
                },
                ..container::Style::default()
            }
        }
    }
}
pub fn styl_scrollable<'a>(
    kolor_akcenty: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, iced::widget::scrollable::Status) -> iced::widget::scrollable::Style + 'a {
    move |_theme: &iced::Theme, status: iced::widget::scrollable::Status| {
        use iced::widget::scrollable::{self, AutoScroll, Rail, Scroller};
        use iced::{Background, Border, Color};



        // Dynamiczny dobór przezroczystości uchwytu w zależności od interakcji
        let scroller_color = match status {
            scrollable::Status::Active { .. } => Color { a: temat.obecny_theme.low, ..*kolor_akcenty },
            scrollable::Status::Hovered { .. } => Color { a: temat.obecny_theme.mid, ..*kolor_akcenty },
            scrollable::Status::Dragged { .. } => Color { a: temat.obecny_theme.hi, ..*kolor_akcenty },
        };

        // Wspólny border dla uchwytów (zaokrąglenie)
        let scroller_border = Border {
            radius: 5.0.into(), // Zaokrąglamy tak jak progress bar
            width: 0.0,
            color: Color{a:temat.obecny_theme.min, ..*kolor_akcenty},
        };

        scrollable::Style {
            container: iced::widget::container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: Border::default(),
                ..Default::default()
            },

            vertical_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(scroller_color),
                    border: scroller_border,
                },
            },

            horizontal_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(scroller_color),
                    border: scroller_border,
                },
            },

            gap: None,

            auto_scroll: AutoScroll {
                background: Background::Color(scroller_color),
                border: scroller_border,
                shadow: Default::default(),
                icon: Default::default(),
            },
        }
    }
}