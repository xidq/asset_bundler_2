pub const FILTERFOTO: [&str; 18] = [
    "jpg", "jpeg", "webp", "png", "tga", "bmp", "ff", "exr", "ico", "hdr", "pnm", "qoi", "tiff",
    "ff.zst", "ff.bz2", "ff.xz", "exr", "avif"
];

// #[derive(Debug, Clone)]
// pub struct OptRozszerzeniaPlikówZdjęciowychIopcje{
//     pub(crate) rozszerzenie: rozszerzenia_plików_zdjęciowych,
//     pub(crate) bit_depth: Vec<Obraz>,
// }

// #[derive(Clone)]
// pub enum CoRobimyZExif{
//     Usuwamy,
//     Zostawiamy,
//     Zmieniamy
// }

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPakowanie {
    pub zbieranie_plików: u32,
    pub pakowanie:(u32,Option<u32>),
    pub kompresja: (u64 ,Option<u64>),
    pub szyfrowanie: (u32 ,Option<u32>),
    pub koniec: String,
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogRozpakowywanie {
    pub kontrola_pliku: (u64,Option<u64>),
    pub rozpakowanie: (u32,Option<u32>),
    pub dekompresja: u64,
    pub deszyfrowanie: (u32,Option<u32>),
    pub czas: String,
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPrzetwarzanieFot {
    pub msg_start: String,
    pub msg_walidacja: (&'static str, String),
    pub msg_proces: String,
    pub msg_end: String,
    pub plik_początek: Option<u32>,
    pub plik_procent: (u32,Option<u32>),
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPakowaniaDds{
    pub pre: (u32, Option<u32>),
    pub post: (u32, Option<u32>),
    pub koniec:String,
    pub err:String,
    
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogRozpakowywanieDds{
    
}

