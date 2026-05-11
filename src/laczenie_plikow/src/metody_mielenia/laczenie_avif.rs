use encodery::avif::{avif_match, avif_zapis};
use encodery::halper::zaszumianie;
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::kolor::ForAvifChroma;
use enumy::rozszerzenia::kompresje::ForAvifKompresja;
use image::DynamicImage;
use std::path::Path;
#[allow(clippy::too_many_arguments)]
pub async fn laczenie_avif(
    bufor: DynamicImage,
    ścieżka_wyjściowa: &Path,
    lossy: Option<u8>,
    bit_depth: BdepthAvif,
    zaszumianie_zmienna: Option<u8>,
    metoda_kompresji: ForAvifKompresja,
    szybkość:i32,
    chrummaaa: ForAvifChroma,
) -> Result<(), tokio::io::Error> {

    let final_final_final_v3_xd = match zaszumianie_zmienna {
        Some(x) => zaszumianie(x, bufor),
        None => bufor,
    };

    let (avf_img,nazwa_organu) = avif_match(final_final_final_v3_xd, &bit_depth).await?;
    avif_zapis(avf_img, nazwa_organu, ścieżka_wyjściowa, &"".to_string(), "", lossy, szybkość, &chrummaaa, &metoda_kompresji).await?;
    
    Ok(())
}


