use crate::dds_halper::oblicz_ilosc_mipmap;
use dds::header::{Dx10Header, DxgiFormat, Header};
use dds::*;
use enumy::rozszerzenia::kompresje::ForDds;
use enumy::statusy::LogTxDdsPak;
use futures::channel::mpsc::Sender;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

pub fn save_rgba_image_with_mipmaps(
    file: &mut File,
    image_data: Vec<&[u8]>,
    width: u32,
    height: u32,
    format: &ForDds,
    kompresja: CompressionQuality,
    mut przerób: &mut f32,
    max_plikow:usize,
    mut percent:&mut u8,
    mut tx: Sender<LogTxDdsPak>,
) -> Result<(), EncodingError> {
    dbg!("jestem w save_rgba_image_with_mipmaps");

    let mut wywoływacz = ||{
        *przerób += 1.;
        dbg!(&przerób);
        if *percent < (*przerób / max_plikow as f32 * 100.).round() as u8 {
            *percent = (*przerób / max_plikow as f32 * 100.).round() as u8;
            let _ = tx.try_send(LogTxDdsPak::Pending(*percent));
        }
    };

    let mut licznik_w_pętli:u32 = 0;

    wywoływacz();



    let (formatowanko_dxgi, formatowanko_format) = match format {
        ForDds::DxgiFormatBc1Unorm => (DxgiFormat::BC1_UNORM, Format::BC1_UNORM),
        ForDds::DxgiFormatBc1UnormSrgb => (DxgiFormat::BC1_UNORM_SRGB, Format::BC1_UNORM),
        ForDds::DxgiFormatBc1Typeless => (DxgiFormat::BC1_TYPELESS, Format::BC1_UNORM),

        ForDds::DxgiFormatBc2Unorm => (DxgiFormat::BC2_UNORM, Format::BC2_UNORM),
        ForDds::DxgiFormatBc2UnormSrgb => (DxgiFormat::BC2_UNORM_SRGB, Format::BC2_UNORM),
        ForDds::DxgiFormatBc2Typeless => (DxgiFormat::BC2_TYPELESS, Format::BC2_UNORM),

        ForDds::DxgiFormatBc3Unorm => (DxgiFormat::BC3_UNORM, Format::BC3_UNORM),
        ForDds::DxgiFormatBc3UnormSrgb => (DxgiFormat::BC3_UNORM_SRGB, Format::BC3_UNORM),
        ForDds::DxgiFormatBc3Typeless => (DxgiFormat::BC3_TYPELESS, Format::BC3_UNORM),

        ForDds::DxgiFormatBc4Unorm => (DxgiFormat::BC4_UNORM, Format::BC4_UNORM),
        ForDds::DxgiFormatBc4Snorm => (DxgiFormat::BC4_SNORM, Format::BC4_SNORM),
        ForDds::DxgiFormatBc4Typeless => (DxgiFormat::BC4_TYPELESS, Format::BC4_UNORM),

        ForDds::DxgiFormatBc5Unorm => (DxgiFormat::BC5_UNORM, Format::BC5_UNORM),
        ForDds::DxgiFormatBc5Snorm => (DxgiFormat::BC5_SNORM, Format::BC5_SNORM),
        ForDds::DxgiFormatBc5Typeless => (DxgiFormat::BC5_TYPELESS, Format::BC5_UNORM),

        ForDds::DxgiFormatBc6HUF16 => (DxgiFormat::BC6H_UF16, Format::BC6H_UF16),
        ForDds::DxgiFormatBc6HSF16 => (DxgiFormat::BC6H_SF16, Format::BC6H_SF16),
        ForDds::DxgiFormatBc6HTypeless => (DxgiFormat::BC6H_TYPELESS, Format::BC6H_UF16),

        ForDds::DxgiFormatBc7Unorm => (DxgiFormat::BC7_UNORM, Format::BC7_UNORM),
        ForDds::DxgiFormatBc7UnormSrgb => (DxgiFormat::BC7_UNORM_SRGB, Format::BC7_UNORM),
        ForDds::DxgiFormatBc7Typeless => (DxgiFormat::BC7_TYPELESS, Format::BC7_UNORM),
    };
    // dbg!(&formatowanko_dxgi, &formatowanko_format);
    // let x: dds::Format = dds::Format::;
    let format = formatowanko_dxgi; // BC5, BC7
    let ilosc_tekstur = image_data.len() as u32;
    // let header = Header::new_image(width, height, format);
    let mut dx10_header = Dx10Header::new_image(width, height, format)
        .with_mipmap_count(oblicz_ilosc_mipmap(width, height));
    dx10_header.array_size = ilosc_tekstur;

    let header: Header = dx10_header.into();

    let mut encoder = Encoder::new(file, formatowanko_format, &header)?;
    encoder.encoding.quality = kompresja; // CompressionQuality::Fast
    encoder.mipmaps.generate = true;

    wywoływacz();

    dbg!(&image_data.len());




    for (i,data) in image_data.iter().enumerate() {
        let view = ImageView::new(data, Size::new(width, height), ColorFormat::RGBA_U8)
            .expect("Błąd danych obrazka");

        encoder.write_surface(view)?;


            wywoływacz();
        
    }
    wywoływacz();
    encoder.finish()?;
    Ok(())
}
