#[macro_export]
macro_rules! styl_przycisków {
    ($warunek:expr, $warunek2:expr, $kolor:expr) => {
        move |_theme, _status| {
            // let warunek_konieczny = match format!("{}",$warunek).as_str() {
            //     "true" => true,
            //     "false" => false,
            //     _=>matches!(_status, button::Status::Active),
            // };   // szkoda że stanu nie zapisuje... ehhh

            if let button::Status::Hovered = _status {
                return button::Style {
                    background: Some(
                        Color {
                            r: $kolor.0,
                            g: $kolor.1,
                            b: $kolor.2,
                            a: 0.2, // Twoja przezroczystość dla hover
                        }
                        .into(),
                    ),
                    text_color: Color::WHITE,
                    border: Border {
                        radius: 5.0.into(),
                        ..Border::default()
                    },
                    ..button::Style::default()
                };
            }
            // if warunek_konieczny{
            if $warunek {
                let mut stopsy = [None; 8];
                stopsy[0] = Some(ColorStop {
                    offset: 0.5, // Koniec koloru w połowie wysokości
                    color: Color {
                        a: 0.0,
                        ..Color::from_rgb($kolor.0, $kolor.1, $kolor.2).into()
                    },
                });
                stopsy[1] = Some(ColorStop {
                    offset: 1.0, // Start na górze
                    color: Color {
                        a: 0.5,
                        ..Color::from_rgb($kolor.0, $kolor.1, $kolor.2).into()
                    },
                });

                button::Style {
                    background: Some(Background::Gradient(
                        iced_core::gradient::Gradient::Linear {
                            0: Linear {
                                angle: Radians(0.0),
                                stops: stopsy,
                            },
                        },
                    )),
                    text_color: Color {
                        a: 0.8,
                        ..Color::WHITE
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                    ..button::Style::default()
                }
            } else if $warunek2 {
                let mut stopsy = [None; 8];
                stopsy[0] = Some(ColorStop {
                    offset: 0.0, // Start na dole
                    color: Color {
                        a: 0.5,
                        ..Color::from_rgb($kolor.0, $kolor.1, $kolor.2).into()
                    },
                });
                stopsy[1] = Some(ColorStop {
                    offset: 0.5, // Koniec koloru w połowie wysokości
                    color: Color {
                        a: 0.0,
                        ..Color::from_rgb($kolor.0, $kolor.1, $kolor.2).into()
                    },
                });
                button::Style {
                    background: Some(Background::Gradient(
                        iced_core::gradient::Gradient::Linear {
                            0: Linear {
                                angle: Radians(0.0),
                                stops: stopsy,
                            },
                        },
                    )),
                    text_color: Color {
                        a: 0.8,
                        ..Color::WHITE
                    },
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                    ..button::Style::default()
                }
            } else {
                // --- STAN WYŁĄCZONY (Biały z Alpha 0.2) ---
                button::Style {
                    background: Some(
                        Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.0, // Twoje 0.2 alpha
                        }
                        .into(),
                    ),
                    text_color: Color {
                        a: 0.5,
                        ..Color::WHITE
                    }, // Lekko przygaszony tekst
                    border: Border {
                        radius: 5.0.into(),
                        width: 1.0,
                        color: Color {
                            a: 0.1,
                            ..Color::WHITE
                        },
                        // ..Border::default()
                    },
                    ..button::Style::default()
                }
            }
        }
    };
}
#[macro_export]
macro_rules! styl_pick_list {
    ($kolor_akcentu:expr,$kolor_tla:expr) => {
        move |_theme: &iced::Theme, _status: iced::widget::pick_list::Status| {
            use iced::widget::pick_list;

            // Kolor bazowy tła (0.1, 0.1, 0.1)
            let tlo_bazowe = Color::from_rgb($kolor_tla.0, $kolor_tla.1, $kolor_tla.2);
            let kolor_akcentu =
                Color::from_rgba($kolor_akcentu.0, $kolor_akcentu.1, $kolor_akcentu.2, 0.5);
            let kolor_border =
                Color::from_rgba($kolor_akcentu.0, $kolor_akcentu.1, $kolor_akcentu.2, 0.2);

            // 1. Styl samego głównego pola PickList
            pick_list::Style {
                text_color: Color {
                    a: 0.7,
                    ..Color::WHITE
                },
                placeholder_color: Color {
                    a: 0.5,
                    ..Color::WHITE
                },
                handle_color: Color::WHITE,
                background: tlo_bazowe.into(),
                border: Border {
                    radius: 5.0.into(),
                    width: if matches!(_status, pick_list::Status::Hovered) {
                        1.0
                    } else {
                        1.0
                    },
                    color: if matches!(_status, pick_list::Status::Hovered) {
                        kolor_akcentu
                    } else {
                        kolor_border
                    },
                },
            }
        }
    };
}

#[macro_export]
macro_rules! styl_menu_pick {
    ($kolor_zaznaczenia:expr, $kolor_tla:expr) => {
        move |_theme| {
            let tlo_menu = Color::from_rgb($kolor_tla.0, $kolor_tla.1, $kolor_tla.2);
            let kolor_zaznaczenia = Color::from_rgba(
                $kolor_zaznaczenia.0,
                $kolor_zaznaczenia.1,
                $kolor_zaznaczenia.2,
                0.5,
            ); // TWÓJ KOLOR

            iced::overlay::menu::Style {
                text_color: Color {
                    a: 0.7,
                    ..Color::WHITE
                },
                background: tlo_menu.into(),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color {
                        a: 0.1,
                        ..Color::WHITE
                    },
                },

                selected_text_color: Color::WHITE,
                selected_background: kolor_zaznaczenia.into(),
                shadow: Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 5.0,
                },
            }
        }
    };
}

#[macro_export]
macro_rules! styl_checkbox {
    ($kolor_akcentu:expr, $kolor_tla:expr) => {
        (move |_theme: &iced::Theme, _status: iced::widget::checkbox::Status| {
            use iced::widget::checkbox;
            use iced::{Border, Color};

            let tlo_bazowe = Color::from_rgb($kolor_tla.0, $kolor_tla.1, $kolor_tla.2);
            let kolor_akcentu =
                Color::from_rgba($kolor_akcentu.0, $kolor_akcentu.1, $kolor_akcentu.2, 0.5);

            // Sprawdzamy stan Hover
            let czy_hover = matches!(_status, checkbox::Status::Hovered { .. });
            // Sprawdzamy czy zaznaczony (Status ma w sobie info o zaznaczeniu)
            let czy_zaznaczony = match _status {
                checkbox::Status::Active { is_checked } => is_checked,
                checkbox::Status::Hovered { is_checked } => is_checked,
                checkbox::Status::Disabled { is_checked } => is_checked,
            };

            checkbox::Style {
                // Tło kwadracika: jeśli zaznaczony, dajemy akcent, jeśli nie - tło bazowe
                background: if czy_zaznaczony {
                    kolor_akcentu.into()
                } else {
                    tlo_bazowe.into()
                },

                // Kolor "ptaszka" (widoczny tylko gdy zaznaczony)
                icon_color: Color {
                    a: 0.7,
                    ..Color::WHITE
                },

                // Kolor tekstu obok checkboxa
                text_color: Some(Color {
                    a: 0.7,
                    ..Color::WHITE
                }),

                border: Border {
                    radius: 3.0.into(), // Lekko mniejszy promień dla małego kwadratu
                    width: if czy_hover { 1.0 } else { 0.5 },
                    color: if czy_hover {
                        kolor_akcentu
                    } else {
                        Color {
                            a: 0.2,
                            ..Color::WHITE
                        }
                    },
                },
            }
        })
    };
}

#[macro_export]
macro_rules! styl_text_input {
    ($kolor_akcentu:expr, $kolor_tla:expr) => {
        (move |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
            use iced::widget::text_input;
            use iced::{Border, Color};

            let tlo_bazowe = Color::from_rgb($kolor_tla.0, $kolor_tla.1, $kolor_tla.2);
            let kolor_akcentu =
                Color::from_rgb($kolor_akcentu.0, $kolor_akcentu.1, $kolor_akcentu.2);

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
                        a: 0.5,
                        ..tlo_bazowe
                    }
                    .into()
                } else {
                    tlo_bazowe.into()
                },

                value: if czy_disabled {
                    Color {
                        a: 0.3,
                        ..Color::WHITE
                    }
                } else {
                    Color {
                        a: 0.7,
                        ..Color::WHITE
                    }
                },

                placeholder: Color {
                    a: 0.3,
                    ..Color::WHITE
                },

                border: Border {
                    radius: 5.0.into(),
                    // Świecimy ramką jeśli hover LUB focused
                    width: if (czy_hover || czy_focused) && !czy_disabled {
                        1.0
                    } else {
                        0.5
                    },
                    color: if (czy_hover || czy_focused) && !czy_disabled {
                        kolor_akcentu
                    } else {
                        Color {
                            a: 0.1,
                            ..Color::WHITE
                        }
                    },
                },

                icon: Color::WHITE,
                selection: Color {
                    a: 0.3,
                    ..kolor_akcentu
                },
            }
        })
    };
}
