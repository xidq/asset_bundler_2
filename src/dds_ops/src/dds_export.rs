use crate::dds_halper::oblicz_ilosc_mipmap;
use crate::strukt::DaneDoZapisu;
use dds::header::{Dx10Header, Dx9Header, Dx9PixelFormat, DxgiFormat, FourCC, Header};
use dds::*;
use enumy::rozszerzenia::kompresje::{DdxDxVersion, ForDds};
use enumy::statusy::LogTxDdsPak;
use futures::channel::mpsc::Sender;


/// # Secondary fn for image -> dds
/// 
pub async fn save_image_to_dds(
    dane: DaneDoZapisu<'_>,
    mut tx: Sender<LogTxDdsPak>,
) -> Result<(), EncodingError> {

    
    let mut przerób: u32 = 0;
    // dbg!("jestem w save_rgba_image_with_mipmaps");

    let mut wywoływacz = ||{
        przerób += 1;
    //     // dbg!(&przerób, (dane.image_data.len() as u32 * 2) + 4);
    // 
            let _ = tx.try_send(LogTxDdsPak::PostępSkładania(przerób,Some((dane.image_data.len() as u32 * 2) + 4)));
    // 
    };
    // let mut wywoływacz = async ||{
    //     
    //     przerób += 1;
    //     wyslij_status(&mut tx,Some(LogTxDdsPak::PostępSkładania(przerób,Some((dane.image_data.len() as u32 * 2) + 4)))).await;
    // 
    // };
    
    wywoływacz();

    let (formatowanko_dxgi, formatowanko_dx9, formatowanko_format) = match dane.format {
        ForDds::DxgiFormatBc1Unorm => (DxgiFormat::BC1_UNORM, Some(Dx9PixelFormat::FourCC(FourCC::DXT1)), Format::BC1_UNORM),
        ForDds::DxgiFormatBc1UnormSrgb => (DxgiFormat::BC1_UNORM_SRGB, Some(Dx9PixelFormat::FourCC(FourCC::DXT1)), Format::BC1_UNORM),
        ForDds::DxgiFormatBc1Typeless => (DxgiFormat::BC1_TYPELESS, Some(Dx9PixelFormat::FourCC(FourCC::DXT1)), Format::BC1_UNORM),

        ForDds::DxgiFormatBc2Unorm => (DxgiFormat::BC2_UNORM, Some(Dx9PixelFormat::FourCC(FourCC::DXT3)), Format::BC2_UNORM),
        ForDds::DxgiFormatBc2UnormSrgb => (DxgiFormat::BC2_UNORM_SRGB, Some(Dx9PixelFormat::FourCC(FourCC::DXT3)), Format::BC2_UNORM),
        ForDds::DxgiFormatBc2Typeless => (DxgiFormat::BC2_TYPELESS, Some(Dx9PixelFormat::FourCC(FourCC::DXT3)), Format::BC2_UNORM),

        ForDds::DxgiFormatBc3Unorm => (DxgiFormat::BC3_UNORM, Some(Dx9PixelFormat::FourCC(FourCC::DXT5)), Format::BC3_UNORM),
        ForDds::DxgiFormatBc3UnormSrgb => (DxgiFormat::BC3_UNORM_SRGB, Some(Dx9PixelFormat::FourCC(FourCC::DXT5)), Format::BC3_UNORM),
        ForDds::DxgiFormatBc3Typeless => (DxgiFormat::BC3_TYPELESS, Some(Dx9PixelFormat::FourCC(FourCC::DXT5)), Format::BC3_UNORM),

        ForDds::DxgiFormatBc4Unorm => (DxgiFormat::BC4_UNORM, None, Format::BC4_UNORM),
        ForDds::DxgiFormatBc4Snorm => (DxgiFormat::BC4_SNORM, None, Format::BC4_SNORM),
        ForDds::DxgiFormatBc4Typeless => (DxgiFormat::BC4_TYPELESS, None, Format::BC4_UNORM),

        ForDds::DxgiFormatBc5Unorm => (DxgiFormat::BC5_UNORM, None, Format::BC5_UNORM),
        ForDds::DxgiFormatBc5Snorm => (DxgiFormat::BC5_SNORM, None, Format::BC5_SNORM),
        ForDds::DxgiFormatBc5Typeless => (DxgiFormat::BC5_TYPELESS, None, Format::BC5_UNORM),

        ForDds::DxgiFormatBc6HUF16 => (DxgiFormat::BC6H_UF16, None, Format::BC6H_UF16),
        ForDds::DxgiFormatBc6HSF16 => (DxgiFormat::BC6H_SF16, None, Format::BC6H_SF16),
        ForDds::DxgiFormatBc6HTypeless => (DxgiFormat::BC6H_TYPELESS, None, Format::BC6H_UF16),

        ForDds::DxgiFormatBc7Unorm => (DxgiFormat::BC7_UNORM, None, Format::BC7_UNORM),
        ForDds::DxgiFormatBc7UnormSrgb => (DxgiFormat::BC7_UNORM_SRGB, None, Format::BC7_UNORM),
        ForDds::DxgiFormatBc7Typeless => (DxgiFormat::BC7_TYPELESS, None, Format::BC7_UNORM),
    };

    wywoływacz();

    // dbg!(&formatowanko_dxgi, &formatowanko_format);
    // let x: dds::Format = dds::Format::;
    let format = formatowanko_dxgi; // BC5, BC7
    let ilosc_tekstur = dane.image_data.len() as u32;
    // let header = Header::new_image(width, height, format);
    // let mut dx10_header = Dx10Header::new_image(dane.width, dane.height, format)
    //     .with_mipmap_count(oblicz_ilosc_mipmap(dane.width, dane.height));
    // dx10_header.array_size = ilosc_tekstur;
    let dx9check = formatowanko_dx9.is_some();

    // let mut dx9_header = Dx9Header::new_image(dane.width, dane.height, formatowanko_dx9.unwrap())
    //     .with_mipmap_count(oblicz_ilosc_mipmap(dane.width, dane.height));

    let mut dx10_header = Dx10Header::new_image(dane.width, dane.height, format)
        .with_mipmap_count(oblicz_ilosc_mipmap(dane.width, dane.height));
    dx10_header.array_size = ilosc_tekstur;
    // dx9_header.array_size = ilosc_tekstur;

    let header: Header = match dane.dx_ver{
        DdxDxVersion::Dx10 => {
            let mut dx10_header = Dx10Header::new_image(dane.width, dane.height, format)
                .with_mipmap_count(oblicz_ilosc_mipmap(dane.width, dane.height));
            dx10_header.array_size = ilosc_tekstur;
            dx10_header.into()
        }
        DdxDxVersion::Dx9 => {
            if dx9check{
                if let Some(dx9_format) = formatowanko_dx9 {
                    // Jeśli jest format DX9, tworzymy go na miejscu z unwrapowanego formatu piksela
                    let dx9_header = Dx9Header::new_image(dane.width, dane.height, dx9_format)
                        .with_mipmap_count(oblicz_ilosc_mipmap(dane.width, dane.height));
                    dx9_header.into()
                } else {
                    // Jeśli nie ma formatu DX9, po prostu zwracamy przygotowany wcześniej dx10_header
                    dx10_header.into()
                }
            }
            else
            {dx10_header.into()}}
    };


    let mut encoder = Encoder::new(dane.file, formatowanko_format, &header)?;
    encoder.encoding.quality = dane.kompresja; // CompressionQuality::Fast
    encoder.mipmaps.generate = dane.minimaps;

    wywoływacz();

    for data in dane.image_data.iter() {
        let view = ImageView::new(data, Size::new(dane.width, dane.height), ColorFormat::RGBA_U8)
            .expect("Błąd danych obrazka");

        wywoływacz();

        encoder.write_surface(view)?;

        wywoływacz();
        
    }
    wywoływacz();
    encoder.finish()?;
    Ok(())
}
