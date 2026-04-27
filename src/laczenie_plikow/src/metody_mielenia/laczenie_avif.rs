use encodery::avif::{avif_match, avif_zapis};
use encodery::halper::zaszumianie;
use enumy::opcje::{AvifChroma, AvifMetodaKompresji, OptFormatyKoloruObrazuAvif, OptInterpolacja};
use image::imageops::FilterType;
use image::DynamicImage;
use std::path::{Path, PathBuf};

pub async fn laczenie_avif(
    bufor: DynamicImage,
    ścieżka_wyjściowa: &PathBuf,
    lossy: Option<u8>,
    bit_depth: OptFormatyKoloruObrazuAvif,
    zaszumianie_zmienna: Option<u8>,
    metoda_kompresji: AvifMetodaKompresji,
    szybkość:i32,
    chrummaaa:AvifChroma,
) -> Result<(), tokio::io::Error> {

    let final_final_final_v3_xd = match zaszumianie_zmienna {
        Some(x) => zaszumianie(x, bufor),
        None => bufor,
    };

    let (avf_img,nazwa_organu) = avif_match(final_final_final_v3_xd, &bit_depth).await?;
    avif_zapis(avf_img, nazwa_organu, ścieżka_wyjściowa, &"".to_string(), "", lossy, szybkość, &chrummaaa, &metoda_kompresji).await?;
    
    Ok(())
}


