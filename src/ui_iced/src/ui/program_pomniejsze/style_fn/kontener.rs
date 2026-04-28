
use iced::widget::container;
use iced::{Color, Background, Border};
use iced_core::gradient::{Linear, ColorStop};
use iced::Radians;
use enumy::inne_ui::RodzajeContainer;

pub fn styl_kontenera<'a>(
    warunek: bool,
    kolor: (f32, f32, f32),
    rodzaj:RodzajeContainer,
) -> impl Fn(&iced::Theme) -> container::Style + 'a {
    move |_theme| {

        // let easy_in = |max_stop:u8,obecny_stop:u8,max_alpha:f32|{
        //     let mnożnik = (obecny_stop + 1) as f32 / (max_stop +1 )as f32;
        //     mnożnik.powf(2.)  * max_alpha
        // };
        // let easy_out = |max_stop:u8,obecny_stop:u8,max_alpha:f32|{
        //     let t = obecny_stop as f32 / (max_stop - 1) as f32;
        //     (1.0 - (1.0 - t).powf(2.)) * max_alpha
        // };

        if warunek {
            let mut stopsy = [None; 8];
            // match rodzaj{
            //     RodzajeContainer::Góra => {
            //         stopsy[0] = Some(ColorStop {
            //             offset: 0.7,
            //             color: Color {
            //                 a: 0.0,
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[1] = Some(ColorStop {
            //             offset: 0.75,
            //             color: Color {
            //                 a: easy_in(5,0,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[2] = Some(ColorStop {
            //             offset: 0.8,
            //             color: Color {
            //                 a: easy_in(5,1,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[3] = Some(ColorStop {
            //             offset: 0.85,
            //             color: Color {
            //                 a: easy_in(5,2,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[4] = Some(ColorStop {
            //             offset: 0.9,
            //             color: Color {
            //                 a: easy_in(5,3,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[5] = Some(ColorStop {
            //             offset: 0.95,
            //             color: Color {
            //                 a: easy_in(5,4,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[6] = Some(ColorStop {
            //             offset: 1.0,
            //             color: Color {
            //                 a: easy_in(5,5,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //     }
            //     RodzajeContainer::Dół => {
            //         stopsy[0] = Some(ColorStop {
            //             offset: 0.5,
            //             color: Color {
            //                 a: 0.0,
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[1] = Some(ColorStop {
            //             offset: 0.0,
            //             color: Color {
            //                 a: 0.5,
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //     }
            //     RodzajeContainer::Oba => {
            //         stopsy[0] = Some(ColorStop {
            //             offset: 0.0,
            //             color: Color {
            //                 a: easy_in(3,3,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[1] = Some(ColorStop {
            //             offset: 0.1,
            //             color: Color {
            //                 a: easy_in(3,2,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[2] = Some(ColorStop {
            //             offset: 0.2,
            //             color: Color {
            //                 a: easy_in(3,1,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[3] = Some(ColorStop {
            //             offset: 0.3,
            //             color: Color {
            //                 a: easy_in(3,0,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[4] = Some(ColorStop {
            //             offset: 0.7,
            //             color: Color {
            //                 a: easy_in(3,0,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[5] = Some(ColorStop {
            //             offset: 0.8,
            //             color: Color {
            //                 a: easy_in(3,1,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[6] = Some(ColorStop {
            //             offset: 0.9,
            //             color: Color {
            //                 a: easy_in(3,2,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //         stopsy[7] = Some(ColorStop {
            //             offset: 1.0,
            //             color: Color {
            //                 a: easy_in(3,3,0.5),
            //                 ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
            //             },
            //         });
            //     }
            match rodzaj{
                RodzajeContainer::Góra => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.5,
                        color: Color {
                            a: 0.0,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 1.0,
                        color: Color {
                            a: 0.5,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });

                }
                RodzajeContainer::Dół => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.5,
                        color: Color {
                            a: 0.0,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            a: 0.5,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                }
                RodzajeContainer::Oba => {
                    stopsy[0] = Some(ColorStop {
                        offset: 0.0,
                        color: Color {
                            a: 0.5,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                    stopsy[1] = Some(ColorStop {
                        offset: 0.4,
                        color: Color {
                            a: 0.0,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                    stopsy[2] = Some(ColorStop {
                        offset: 0.6,
                        color: Color {
                            a: 0.0,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                        },
                    });
                    stopsy[3] = Some(ColorStop {
                        offset: 1.0,
                        color: Color {
                            a: 0.5,
                            ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
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
            // Stan gdy warunek jest fałszywy (np. przezroczysty lub biały)
            container::Style {
                background: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 }.into()),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color { a: 0.1, ..Color::WHITE },
                },
                ..container::Style::default()
            }
        }
    }
}