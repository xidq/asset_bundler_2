use iced::Color;

#[derive(Clone, Debug)]
pub enum WybraneOknoEdycjiZdjęć {
    Ścieżki,
    OptRozszerzeniaPlikówZdjęciowych,
    MenuOptRozdzielczościObrazów,
    MenuReszta,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPodstrony {
    BinPakowanie,
    BinRozpakowanie,
    KonwersjaFoto,
    KonwersjaFotoŚcieżki,
    KonwersjaFotoRozszerzenia,
    KonwersjaFotoRozdzielczości,
    KonwersjaFotoMenuReszta,
    DaneDoŁączeniaZdjęćo,
    ObslugaDds,
    // Ustawienia,
    // Logi,
    Dev,
}
pub struct UstawieniaThemeWsio{
    pub kolory:ObecnyColorTheme,
    pub obecny_theme:ObecnyColorThemePrzezroczystosci,
    pub tekst:ObecnyColorCzcionkiPrzezroczystosci,
    pub ustawienia: Ustawienia,
    pub temp: Temp,
}
pub struct Temp{
    pub aktywny_proces: ActProces,
    pub aktywne_okno: UiPodstrony,
}
pub struct Ustawienia{
    pub halp_menu:bool,

    
}
pub struct  ObecnyColorTheme{
    pub binarka: Color,
    pub konwersja:Color,
    pub laczenie:Color,
    pub dds:Color,
    pub ustawienia:Color,
    pub hint:Color,
}
pub struct ObecnyColorThemePrzezroczystosci{
    pub max:f32,
    pub hi:f32,
    pub mid:f32,
    pub low:f32,
    pub min:f32,
    pub kolor:Color,
    pub bground: Color,
    pub bground_lewy: Color,
}
pub struct ObecnyColorCzcionkiPrzezroczystosci{
    pub max:f32,
    pub hi:f32,
    pub mid:f32,
    pub low:f32,
    pub min:f32,
    pub kolor:Color,
}
#[derive(Clone, Debug)]
pub enum StronyDds {
    ZplikuDoDds,
    ZddsDoPliku,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WybranyFormatZdjecia{
    Jpg,
    Png,
    Webp,
    Tga,
    Ff,
    Qoi,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RodzajeContainer{
    Góra,
    Dół,
    Oba,
}

#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq)]
pub enum ActProces {
    PakowaniePliku,
    RozpakowaniePliku,
    DdsPakowanie,
    DdsRozpakowanie,
    ŁączenieZdjęć,
    KonwersjaZdjęć,
    Żodyn,
}
