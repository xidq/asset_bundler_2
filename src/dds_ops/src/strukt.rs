use std::fs::File;
use dds::CompressionQuality;
use enumy::rozszerzenia::kompresje::ForDds;

pub struct DaneDoZapisu<'a>{
    pub file: &'a mut File,
    pub image_data: Vec<&'a [u8]>,
    pub width: u32,
    pub height: u32,
    pub format: &'a ForDds,
    pub kompresja: CompressionQuality,
    pub minimaps: bool,
}