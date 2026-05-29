use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::text::info_male;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::ext::{ImgExt, ImgExtTag};
use iced::widget::{Column, Row};
use iced::Element;
use strum::{EnumMessage, IntoEnumIterator};

pub fn wybory<'a>(dane: &'a DaneKonw, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {

//jpg ----------------------------------------------------------------------------------------------

    let jpg_bool = dane.tag.contains(&ImgExtTag::Jpg);

    let (jpg_jakosc, jpg_prog, jpg_bdepth, jpg_qa) =
        if let Some(ImgExt::Jpg {
             jakosc, progresywny, bit_depth, quant, ..
         }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Jpg { .. })) {
        (
            jakosc.to_string(),
            progresywny,
            bit_depth,
            quant.get_message().unwrap_or("err").to_string()
        )
    } else {
        (
            "-".to_string(),
            &false,
            &Vec::new(),
            "-".to_string()
        )
    };

    let mut jpg_row = Row::new().spacing(3.);

    jpg_row = jpg_row
        .push(info_male("|".to_string(), jpg_bool, temat));

    for wariant in BdepthJpg::iter() {
        jpg_row = jpg_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                jpg_bdepth.contains(&wariant),
                temat
            )
        );
    }

    jpg_row = jpg_row
        .push(info_male("|".to_string(), jpg_bool,temat))
        .push(info_male(jpg_jakosc, jpg_bool, temat))
        .push(info_male("|".to_string(), jpg_bool,temat))
        .push(info_male(jpg_qa, jpg_bool, temat))
        .push(info_male("|".to_string(), jpg_bool,temat))
        .push(info_male("Prog".to_string(), *jpg_prog, temat));


//avif ----------------------------------------------------------------------------------------------

    let avif_bool = dane.tag.contains(&ImgExtTag::Avif);

    let (avif_speed, avif_lossy, avif_bdepth, avif_kompresja, avif_chroma) =
        if let Some(ImgExt::Avif {
                        chroma, speed, metoda_kompresji, lossy, bit_depth
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Avif { .. })) {
            (
                speed.to_string(),
                match lossy{ Some (xx) => xx.to_string(), None => "Lossless".to_string()},
                bit_depth,
                metoda_kompresji.krotkie(),
                chroma.to_string()
            )
        } else {
            (
                "-".to_string(),
                "-".to_string(),
                &Vec::new(),
                "-",
                "-".to_string(),
            )
        };

    let mut avif_row = Row::new().spacing(3.);
    let mut avif_row2 = Row::new().spacing(3.);


    avif_row = avif_row
        .push(info_male("|".to_string(), avif_bool, temat));

    for wariant in BdepthAvif::iter() {
        avif_row = avif_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                avif_bdepth.contains(&wariant),
                temat
            )
        );
    }

    avif_row = avif_row
        .push(info_male("|".to_string(), avif_bool,temat))
        .push(info_male(avif_speed, avif_bool, temat))
        .push(info_male("|".to_string(), avif_bool,temat))
        .push(info_male(avif_kompresja.to_string(), avif_bool, temat))
        .push(info_male("|".to_string(), avif_bool,temat));

    avif_row2= avif_row2
        .push(info_male("|".to_string(), avif_bool,temat))
        .push(info_male(avif_lossy, avif_bool, temat))
        .push(info_male("|".to_string(), avif_bool,temat))
        .push(info_male(avif_chroma, avif_bool, temat));

//png ----------------------------------------------------------------------------------------------

    let png_bool = dane.tag.contains(&ImgExtTag::Png);

    let (png_kompresja, png_bdepth, ) =
        if let Some(ImgExt::Png {
                        kompresja, bit_depth
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Png { .. })) {
            (
                kompresja.to_string(),
                bit_depth,
            )
        } else {
            (
                "-".to_string(),
                &Vec::new(),
            )
        };

    let mut png_row = Row::new().spacing(3.);

    png_row = png_row
        .push(info_male("|".to_string(), png_bool, temat));

    for wariant in BdepthPng::iter() {
        png_row = png_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                png_bdepth.contains(&wariant),
                temat
            )
        );
    }

    png_row = png_row
        .push(info_male("|".to_string(), png_bool,temat))
        .push(info_male(png_kompresja, png_bool, temat))
        .push(info_male("|".to_string(), png_bool,temat));

//webp ----------------------------------------------------------------------------------------------

    let webp_bool = dane.tag.contains(&ImgExtTag::Webp);

    let (webp_jakosc, webp_lossy, webp_bdepth) =
        if let Some(ImgExt::Webp {
                        jakosc, lossless, bit_depth
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Webp { .. })) {
            (
                jakosc.to_string(),
                lossless,
                bit_depth,
            )
        } else {
            (
                "-".to_string(),
                &false,
                &Vec::new(),
            )
        };

    let mut webp_row = Row::new().spacing(3.);

    webp_row = webp_row
        .push(info_male("|".to_string(), webp_bool, temat));

    for wariant in BdepthWebp::iter() {
        webp_row = webp_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                webp_bdepth.contains(&wariant),
                temat
            )
        );
    }

    webp_row = webp_row
        .push(info_male("|".to_string(), webp_bool,temat))
        .push(info_male(webp_jakosc, webp_bool && !*webp_lossy, temat))
        .push(info_male("|".to_string(), webp_bool,temat))
        .push(info_male("Lossless".to_string(), *webp_lossy, temat))
        .push(info_male("|".to_string(), webp_bool,temat));


//tga ----------------------------------------------------------------------------------------------

    let tga_bool = dane.tag.contains(&ImgExtTag::Tga);

    let tga_bdepth =
        if let Some(ImgExt::Tga {
                         bit_depth
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Tga { .. })) {
            bit_depth
        } else {
            &Vec::new()
        };

    let mut tga_row = Row::new().spacing(3.);

    tga_row = tga_row
        .push(info_male("|".to_string(), tga_bool, temat));

    for wariant in BdepthTga::iter() {
        tga_row = tga_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                tga_bdepth.contains(&wariant),
                temat
            )
        );
    }

    tga_row = tga_row
        .push(info_male("|".to_string(), tga_bool,temat));


//ff ----------------------------------------------------------------------------------------------

    let ff_bool = dane.tag.contains(&ImgExtTag::Ff);

    let (ff_kompresja, /* ff_kompresja_wartosc */) =
        if let Some(ImgExt::Ff {
                        metoda_kompresji
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Ff { .. })) {

            (
                metoda_kompresji.to_string(),
                // match metoda_kompresji {
                //     ForFfKompresja::Zstd(v) | ForFfKompresja::Bzip2(v) | ForFfKompresja::Xz(v) => v.to_string(),
                //     ForFfKompresja::Brak => "".to_string(),
                // }
            )

        } else {
            (
                "-".to_string(),
                // "-".to_string()
            )
        };

    let mut ff_row = Row::new().spacing(3.);

    ff_row = ff_row
        .push(info_male("|".to_string(), ff_bool, temat));

    ff_row = ff_row
        .push(info_male(ff_kompresja, ff_bool, temat))
        .push(info_male("|".to_string(), ff_bool,temat));

//qoi ----------------------------------------------------------------------------------------------

    let qoi_bool = dane.tag.contains(&ImgExtTag::Qoi);

    let qoi_bdepth =
        if let Some(ImgExt::Qoi {
                        bit_depth
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Qoi { .. })) {
                bit_depth
        } else {
            &Vec::new()
        };

    let mut qoi_row = Row::new().spacing(3.);

    qoi_row = qoi_row
        .push(info_male("|".to_string(), qoi_bool, temat));

    for wariant in BdepthQoi::iter() {
        qoi_row = qoi_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                qoi_bdepth.contains(&wariant),
                temat
            )
        );
    }

    qoi_row = qoi_row
        .push(info_male("|".to_string(), qoi_bool,temat));

//exr ----------------------------------------------------------------------------------------------

    let exr_bool = dane.tag.contains(&ImgExtTag::Exr);

    let (exr_kompresja, /* ff_kompresja_wartosc */) =
        if let Some(ImgExt::Exr {
                        kompresja, ..
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Ff { .. })) {

            (
                kompresja.to_string(),
                // match metoda_kompresji {
                //     ForFfKompresja::Zstd(v) | ForFfKompresja::Bzip2(v) | ForFfKompresja::Xz(v) => v.to_string(),
                //     ForFfKompresja::Brak => "".to_string(),
                // }
            )

        } else {
            (
                "-".to_string(),
                // "-".to_string()
            )
        };

    let exr_bdepth =
        if let Some(ImgExt::Exr {
                        bit_depth, kompresja: _
                    }) = dane.rozszerzenia.iter().find(|f| matches!(f, ImgExt::Exr { .. })) {
            bit_depth
        } else {
            &Vec::new()
        };

    let mut exr_row = Row::new().spacing(3.);

    exr_row = exr_row
        .push(info_male("|".to_string(), exr_bool, temat));

    for wariant in BdepthExr::iter() {
        exr_row = exr_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                exr_bdepth.contains(&wariant),
                temat
            )
        );
    }

    exr_row = exr_row
        .push(info_male("|".to_string(), exr_bool,temat))
        .push(info_male(exr_kompresja, exr_bool,temat))
        .push(info_male("|".to_string(), exr_bool,temat));



    //rozdzielczosci -----------------------------------------------------------------------------------

    let mut roz_row = Row::new().spacing(3.);

    for wariant in Rozdzielczości::iter() {
        roz_row = roz_row.push(
            info_male(
                wariant.maly_wariant().to_string(),
                dane.opcje_rozdzielczości.contains(&wariant),
                temat
            )
        );
    }

// inne --------------------------------------------------------------------------------------------

    let mut inne_row = Row::new().spacing(3.);
    inne_row = inne_row
        .push(info_male("|".to_string(), true, temat))
        .push(info_male("Exif".to_string(), dane.exif, temat))
        .push(info_male("|".to_string(), true, temat))
        .push(info_male(format!("Noise: {}", dane.noising.unwrap_or( 0)), dane.noising.is_some(), temat))
        .push(info_male("|".to_string(), true, temat));


// łączenie ----------------------------------------------------------------------------------------

    Column::new()
        .push(info_male("Jpg".to_string(), jpg_bool, temat))
        .push(jpg_row)
        .push(info_male("Avif".to_string(), avif_bool, temat))
        .push(avif_row)
        .push(avif_row2)
        .push(info_male("Png".to_string(), png_bool, temat))
        .push(png_row)
        .push(info_male("Webp".to_string(), webp_bool, temat))
        .push(webp_row)
        .push(info_male("Tga".to_string(), tga_bool, temat))
        .push(tga_row)
        .push(info_male("FF".to_string(), ff_bool, temat))
        .push(ff_row)
        .push(info_male("Qoi".to_string(), qoi_bool, temat))
        .push(qoi_row)
        .push(info_male("Exr".to_string(), exr_bool, temat))
        .push(exr_row)
        .push(info_male("Resolutions".to_string(), true, temat))
        .push(roz_row)
        .push(info_male("Etc".to_string(), true, temat))
        .push(inne_row)
        .into()
}