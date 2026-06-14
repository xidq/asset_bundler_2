use crate::wczytywanie::avif::avif;
use crate::wczytywanie::check_extension::rozpoznaj_format;
use crate::wczytywanie::ff::ff;
use crate::wczytywanie::jpg::jpeg;
use crate::wczytywanie::png::png;
use crate::wczytywanie::qoi::qoi;
use crate::wczytywanie::strukty::DaneDoWczytywania;
use crate::wczytywanie::tga::tga;
use crate::wczytywanie::unknown::unknown;
use crate::wczytywanie::webp::webp;
use enumy::rozszerzenia::ext::ImgExtTag;
use std::io::Read;
use std::path::PathBuf;
use crate::wczytywanie::exr::exr_loading;

/// # Main fn for image decoding
/// That's how it is.
/// 
/// I'm using Image crate as just wrapper for moving data,
/// and using their resize fn elsewhere ;)
pub fn wczytaj_pliki(
    ścieżka: PathBuf
) -> Result<(DaneDoWczytywania, String), std::io::Error>{
    let bajty = std::fs::read(&ścieżka)?;
    let nazwa = ścieżka
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("nieznany")
        .to_string();
    let rozszerzenie = ścieżka
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // let sprawdzanie_kompresji = sprawdzanie_kompresji_zdjecia?;
    let sprawdzanie_kompresji = match rozszerzenie.as_str(){
        "zst" => {
            let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "bz2" => {
            let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "xz" => {
            let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        _ => {bajty}
    };
    
    let fotu= match rozpoznaj_format(&sprawdzanie_kompresji){
        ImgExtTag::Avif => {avif(&sprawdzanie_kompresji)}
        ImgExtTag::Jpg => {jpeg(&sprawdzanie_kompresji)}
        ImgExtTag::Png => {png(&sprawdzanie_kompresji)}
        ImgExtTag::Webp => {webp(&sprawdzanie_kompresji)}
        ImgExtTag::Tga => {tga(&sprawdzanie_kompresji)}
        ImgExtTag::Ff => {ff(&sprawdzanie_kompresji)}
        ImgExtTag::Qoi => {qoi(&sprawdzanie_kompresji)}
        ImgExtTag::Unknown => {unknown(&sprawdzanie_kompresji)}
        ImgExtTag::Exr => {exr_loading(&sprawdzanie_kompresji)}
    }?;
    
    Ok((fotu, nazwa))
}


// pub fn sprawdzanie_kompresji_zdjecia(rozszerzenie: String, bajty: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
//     let blablabla = match rozszerzenie.as_str(){
//         "zst" => {
//             let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
//             let mut rozpakowane = Vec::new();
//             decoder.read_to_end(&mut rozpakowane)?;
//             rozpakowane
//         }
//         "bz2" => {
//             let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
//             let mut rozpakowane = Vec::new();
//             decoder.read_to_end(&mut rozpakowane)?;
//             rozpakowane
//         }
//         "xz" => {
//             let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
//             let mut rozpakowane = Vec::new();
//             decoder.read_to_end(&mut rozpakowane)?;
//             rozpakowane
//         }
//         _ => {bajty}
//     };
//     Ok(blablabla)
// }