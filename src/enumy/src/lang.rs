use crate::wybranie_jezykowe::WybórJęzyka;
use std::path::Path;

pub fn zmieniacz_ilosci_bajtow<T>(liczba: T) -> String
where
    T: TryInto<u64> + std::fmt::Display + Copy,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
{
    let n: u64 = liczba
        .try_into()
        .expect("Błąd: Liczba jest ujemna lub za duża dla u64");
    let n_f = n as f32;
    let bity = 8_f32;
    let kibi = 1024_u32;

    match n {
        0..=8 => format!("{}b", n),
        9..=8192 => format!("{:.2}B", n_f / bity),
        8193..=8388608 => format!("{:.2}KB", n_f / (kibi.pow(1) as f32 * bity)),
        8388609..=8589934592 => format!("{:.2}MB", n_f / (kibi.pow(2) as f32 * bity)),
        8589934593..=8796093022208 => format!("{:.2}GB", n_f / (kibi.pow(3) as f32 * bity)),
        _ => format!("{:.2}TB", n_f / (kibi.pow(4) as f32 * bity)),
    }
}
pub fn odmiana_liczbowa<T>(liczba: T) -> String
where
    T: TryInto<u64> + std::fmt::Display + Copy,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
{
    let n: u64 = liczba
        .try_into()
        .expect("Błąd: Liczba jest ujemna lub za duża dla u64");
    // Konwersja na u64 (bezpieczna dla wszystkich typów dodatnich)

    let ostatnia_cyfra = n % 10;
    let dwie_ostatnie = n % 100;

    let zakonczenie = match (n, ostatnia_cyfra, dwie_ostatnie) {
        // Dokładnie 1 plik
        (1, _, _) => "numbers_1",

        // Wyjątek nastolatek: 12, 13, 14 (zawsze "plików")
        (_, _, 12..=14) => "numbers_3",

        // Końcówki 2, 3, 4 (np. 22, 23, 24, 104) -> "pliki"
        (_, 2..=4, _) => "numbers_2",

        // Wszystko inne (0, 5-11, 15-21, 25-31, 101, 1001 itd.) -> "plików"
        _ => "numbers_3",
    };

    zakonczenie.to_string()
}

pub trait ToU32 {
    fn to_u32(&self) -> u32;
}

impl ToU32 for i32 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for i16 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for i8 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for f64 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for f32 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for u16 { fn to_u32(&self) -> u32 { *self as u32 } }
impl ToU32 for u8 { fn to_u32(&self) -> u32 { *self as u32 } }
impl WybórJęzyka {

    pub fn format_sciezek(&self, xx: &Path) -> String {
        let starrrrtuuuuuu = match xx.components().nth(1){
            None => {String::new()}
            Some(xxx) => {xxx.as_os_str().to_string_lossy().to_string()}
        };

        let endo = match xx.file_name(){
            None => {String::new()}
            Some(xxx) => {xxx.to_string_lossy().to_string()}
        };

        let poczatek = if xx.to_string_lossy().len() < 30 {
            xx.to_string_lossy().to_string()
        } else if starrrrtuuuuuu.len() < 30 {
            let secnd = match xx.components().nth(2){
                None => {String::new()}
                Some(xxx) => {xxx.as_os_str().to_string_lossy().to_string()}
            };
            starrrrtuuuuuu + "/" + &secnd + "/"
        } else {
            starrrrtuuuuuu + "/"
        };

        if xx.to_string_lossy().len() < 30 {
            poczatek
        } else {
            poczatek + " ... " + "/" + endo.as_str()
        }
    }
    pub fn normal_u32_option(&self,var: (impl Into<u32> + std::clone::Clone, Option<impl Into<u32> + std::clone::Clone>)) -> (u32, Option<u32>){
        let vwar0: u32 = var.0.clone().into();
        let f1:u32 = vwar0;
        let xxx;

        if let Some(wartosc2) = var.1 {
            xxx = Some(wartosc2.clone().into() )
        } else {xxx = None}
        // let f2 = var.1.map(|v| v.to_f32()).unwrap_or(0.0);

        (f1, xxx)
    }
    pub fn normal_u64_option(&self,var: (impl Into<u64> + std::clone::Clone, Option<impl Into<u64> + std::clone::Clone>)) -> (u64, Option<u64>){
        let vwar0: u64 = var.0.clone().into();
        let f1:u64 = vwar0;
        let xxx;

        if let Some(wartosc2) = var.1 {
            xxx = Some(wartosc2.clone().into() )
        } else {xxx = None}
        // let f2 = var.1.map(|v| v.to_f32()).unwrap_or(0.0);

        (f1, xxx)
    }

    pub fn bajt(&self, val:impl Into<u64> + std::clone::Clone )->String{
        let xx: u64 = val.clone().into();
        let kb: u64 = 1024_u64 * 8;
        let mb: u64 = 1024_u64.pow(2) * 8;
        let gb: u64 = 1024_u64.pow(3) * 8;
        let tb: u64 = 1024_u64.pow(4) * 8;
        match xx{
            0..=8 => { format!("{}b",xx) }
            n if n < kb => { format!("{:.2}B",xx as f64/8.) }
            n if n < mb => { format!("{:.2}kB",xx as f64/kb as f64) }
            n if n < gb => { format!("{:.2}MB",xx as f64/mb as f64) }
            n if n < tb => { format!("{:.2}GB",xx as f64/gb as f64) }
            _ => { format!("{:.2}TB", xx as f64/tb as f64 ) }
        }

    }
}

impl WybórJęzyka {
    pub fn t(&self, klucz: &str) -> &'static str {
        match self {
            WybórJęzyka::PL => match klucz {
                "roz_16" => "16"
                ,"info_roz_16" => "",
                "roz_32" => "32"
                ,"info_roz_32" => "",
                "roz_64" => "64"
                ,"info_roz_64" => "",
                "roz_128" => "128"
                ,"info_roz_128" => "",
                "roz_256" => "256"
                ,"info_roz_256" => "",
                "roz_512" => "512"
                ,"info_roz_512" => "",
                "roz_1024" => "1k"
                ,"info_roz_1024" => "",
                "roz_2048" => "2k"
                ,"info_roz_2048" => "",
                "roz_4096" => "4k"
                ,"info_roz_4096" => "",
                "roz_6144" => "6k"
                ,"info_roz_6144" => "",
                "roz_8192" => "8k"
                ,"info_roz_8192" => "",
                "roz_16384" => "16k"
                ,"info_roz_16384" => "",
                "roz_org" => "Rozdzielczość oryginalna",
                "info_roz_org" => "",

                "numbers_1" => "plik",
                "numbers_2" => "pliki",
                "numbers_3" => "plików",

                "mgt_input_file" => "Plik wejściowy",
                "mgt_input_file_multiple" => "Pliki wejściowe",
                "mgt_input_file_r" => "Plik wejściowy czerwony",
                "mgt_input_file_g" => "Plik wejściowy zielony",
                "mgt_input_file_b" => "Plik wejściowy niebieski",
                "mgt_input_file_a" => "Plik wejściowy alpha",
                "mgt_input_folder" => "Folder wejściowy",
                "mgt_input_folder_multiple" => "Foldery wejściowe",
                "mgt_input_folder_or_file" => "Plik lub folder wejściowy",
                "mgt_output_folder" => "Folder wyjściowy",
                "mgt_compression_level_label" => "Poziom kompresji",
                "mgt_compression_none" => "Brak kompresji",
                "mgt_filter_label" => "Filtr",
                "mgt_file_name" => "Nazwa pliku",
                "mgt_btn_ready" => "Rozpocznij",
                "mgt_btn_busy_processing" => "Działam",
                "mgt_btn_busy_processing_other" => "Coś innego już trwa",
                "mgt_btn_gib_data" => "Uzupełnij informacje",
                "mgt_proces_error" => "Błąd",
                "mgt_interpolation" => "Interpolacja",
                "mgt_gen_off" => "Wyłączone",
                "mgt_more" => "więcej",
                "mgt_and" => "i",
                "msg_no_folder_nor_file" => "Nie wybrano plików bądź folderów",
                "mgt_exif_data" => "Dane EXIF",
                "mgt_file_treatment" => "Istniejące pliki:",

                "ui_main_btn_binary" => "Binarka",
                "ui_main_btn_conversion" => "Konwersja",
                "ui_main_btn_merge" => "Łączenie",
                "ui_main_btn_dds" => "DDS",
                "ui_main_btn_settings" => "Ustawienia",
                "ui_menu_bin_pack" => "Pakowanie",
                "ui_menu_bin_unpack" => "Rozpakowywanie",
                "ui_conversion_paths" => "Ścieżki",
                "ui_conversion_extensions" => "Rozszerzenia",
                "ui_conversion_resolutions" => "Rozdzielczości",
                "ui_conversion_rest" => "Inne",
                "ui_dds_pack" => "Pakowanie",
                "ui_dds_unpack" => "Rozpakowywanie",
                "ui_gen_extensions" => "Rozszerzenia",
                "ui_merge" => "Łączenie",


                "proces_binary_pack_collecting_pending" => "Zebrano",
                "proces_binary_pack_packing_pending" => "Pakuję",
                "proces_binary_pack_packing" => "Spakowano",
                "proces_binary_pack_compression_pending" => "Postęp kompresji",
                "proces_binary_pack_compression" => "Zakończona kompresja",
                "proces_binary_pack_encoding_pending" => "Postęp kodowania",
                "proces_binary_pack_encoding" => "Zakończone kodowanie",
                "proces_binary_pack_end" => "Zakończono proces pakowania w czasie",

                "proces_binary_unpack_files_pending" => "Pliki",
                "proces_binary_unpack_files" => "Znaleziono",
                "proces_binary_unpack_decoding_pending" => "Postęp dekodowania",
                "proces_binary_unpack_decoding" => "Zdekodowano",
                "proces_binary_unpack_decompression" => "Zdekompresowano",
                "proces_binary_unpack_unpacking_pending" => "Postęp rozpakowywania",
                "proces_binary_unpack_unpacking" => "Rozpakowano",
                "proces_binary_unpack_end" => "Zakończono proces rozpakowywania w czasie",
                
                "proces_conversion_reset_paths" => "Resetuj ścieżki",
                "proces_conversion_checkbox" => "Scieżka wyjściowa ta sama co wejściowa",
                "proces_conversion_proces_pending" => "Postęp procesu: ",
                

                "conversion_jpg_prog" => "Zapis progresywny",
                "conversion_jpg_sampling" => "Próbkowanie:",
                "conversion_jpg_qua" => "Kwant?:",
                "conversion_avif_lossless" => "Bezstratne",
                "mgt_compression" => "Kompresja:",
                "conversion_avif_chroma" => "Chroma:",
                "conversion_webp_losless" => "Bezstratny.",
                "conversion_noising" => "Zaszumienie:",
                "conversion_alpha_color" => "Kolor alpha:",
                "dds_format" => "Format:",


                "hint_mgt_btn_ready" => "Rozpocznij proces",
                "hint_mgt_btn_busy_processing" => "Proces już jest w fazie działania",
                "hint_mgt_btn_busy_processing_other" => "Zaiste inny proces obecnie jest wykonywany",
                "hint_mgt_btn_gib_data" => "Czegoś tu jeszcze brakuje",
                "hint_mgt_proces_error" => "Błąd",
                "hint_ui_main_btn_binary" => "Pakowanie/rozpakowywanie plików",
                "hint_ui_main_btn_conversion" => "Zbiorcze przerabianie zdjęć",
                "hint_ui_main_btn_merge" => "Łączenie kilku obrazów w jeden",
                "hint_ui_main_btn_dds" => "Zakładka z obsługą plików z formatem .dds",
                "hint_ui_main_btn_settings" => "Zakładka z ustawieniami",
                "hint_ui_menu_bin_pack" => "Pakowanie plików",
                "hint_ui_menu_bin_unpack" => "Rozpakowywanie plików",
                "hint_ui_conversion_paths" => "Tutaj wybierzesz ścieżki",
                "hint_ui_conversion_extensions" => "Tutaj wybierzesz rozszerzenia",
                "hint_ui_conversion_resolutions" => "Tutaj wybierzesz rozdzielczości",
                "hint_ui_conversion_rest" => "Tutaj znajdziesz inne operacje",
                "hint_ui_gen_extensions" => "Menu z rozszerzeniami",
                "hint_binary_pack_process_btn" => "Przycisk do uruchomienia pakowania plików",
                "hint_binary_pack_choose_out_path_btn" => "Przycisk do wyboru ścieżki wyjściowej, gdzie ma być umieszczony plik",
                "hint_binary_pack_choose_in_path_btn" => "Przycisk do wyboru ścieżki wejściowej, z której to pliki będą zbierane do pakowania",
                "hint_binary_pack_choose_filter" => "Menu wyboru filtracji plików",
                "hint_binary_pack_choose_compression" => "Menu wyboru jak mocna kompresja ma być zastosowana",
                "hint_binary_unpack_process_btn" => "Przycisk do uruchomienia rozpakowywania plików",
                "hint_binary_unpack_choose_out_path_btn" => "Przycisk do wyboru ścieżki wyjściowej, gdzie mają być umieszczone pliki",
                "hint_binary_unpack_choose_in_path_btn" => "Przycisk do wyboru ścieżki wejściowej dla pliku, który ma zostać rozpakowany ;)",
                "hint_conversion_choose_file_btn" => "Wybierz plik",
                "hint_conversion_choose_folder_btn" => "Wybierz folder",
                "hint_conversion_choose_output_folder" => "Wybierz folder wyjściowy",
                "hint_conversion_choose_output_folder_paths" => "Folder wyjściowy",
                "hint_conversion_choose_output_folder_paths_same" => "Folder wyjściowy ten sam co wejściowy",
                "hint_conversion_reset_paths" => "Zresetuj ścieżki wejściowe",
                "hint_conversion_jpg_prog" => "Przełącznik zapisu progresywnego",
                "hint_conversion_webp_losless" => "Jeżeli aktywny to zapis bezstratny",
                "hint_conversion_process_btn" => "Przycisk do uruchomienia rozpakowywania plików",
                "hint_conversion_process_btn_pending" => "Kieruj się wskazówkami na przycisku, acz ogólnie to przycisk do uruchomienia rozpakowywania plików",
                "hint_conversion_avif_lossless" => "Przełącznik zapisu bezstratnego",
                "hint_ui_dds_pack" => "Pakowanie do pliku .dds",
                "hint_ui_dds_unpack" => "Rozpakowanie pliku dds",
                "hint_ui_merge" => "Łączenie plików w jeden",


                "log_status_welcome_msg_welcome_morning" => "Dobrej nocy", //powitanie w nocy 00 do 6 rano
                "log_status_welcome_msg_welcome_evening" => "Dobry wieczór",
                "log_status_welcome_msg_welcome_day" => "Dzień dobry",
                "log_status_welcome_msg_today" => "Dziś",
                "log_status_welcome_msg_time" => "Jest godzina",
                "log_status_welcome_msg_sys_rdy" => "System w gotowości",
                "log_status_welcome_msg_lang_detected" => "Wykryto język",
                "log_help_menu" => "'Ctrl + H' aby widzieć podpowiedzi",

                "console_menu_status" => "Status:",
                "console_menu_reset" => "Resetuj",

                "data_chck_start" => "Odebrano dane, rozpoczynam analizę",
                "data_chck_input_path" => "Śceżika wejściowa:",
                "data_chck_output_path" => "Śceżika wyjściowa:",
                "data_chck_jpg_bdepth_err" => "Jpg: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_jpg_bdepth" => "Sprawdzanie danych Jpg:",
                "data_chck_png_bdepth_err" => "Png: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_png_bdepth" => "Sprawdzanie danych Png:",
                "data_chck_webp_bdepth_err" => "Webp: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_webp_bdepth" => "Sprawdzanie danych Webp:",
                "data_chck_tga_bdepth_err" => "Tga: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_tga_bdepth" => "Sprawdzanie danych Tga:",
                "data_chck_ff_bdepth_err" => "FF: Wykryto nieobsługiwaną kompresje lub brak wyboru!",
                "data_chck_ff_bdepth" => "Sprawdzanie danych FF:",
                "data_chck_qoi_bdepth_err" => "Qoi: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_qoi_bdepth" => "Sprawdzanie danych Qoi:",
                "data_chck_avif_bdepth_err" => "Avif: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!",
                "data_chck_avif_bdepth" => "Sprawdzanie danych Avif:",
                "data_chck_res_err" => "Błędna lub brak rozdzielczości",
                "data_chck_res" => "Sprawdzanie rozdzielczości:",
                "data_chck_inter_err" => "Błędna lub brak interpolacji",
                "data_chck_inter" => "Sprawdzanie interpolacji:",
                "data_chck_fin" => "Wsio ok!",
                

                _ => "Brak tłumaczenia",
            },
            WybórJęzyka::EN => match klucz {
                "roz_16" => "16"
                ,"info_roz_16" => "",
                "roz_32" => "32"
                ,"info_roz_32" => "",
                "roz_64" => "64"
                ,"info_roz_64" => "",
                "roz_128" => "128"
                ,"info_roz_128" => "",
                "roz_256" => "256"
                ,"info_roz_256" => "",
                "roz_512" => "512"
                ,"info_roz_512" => "",
                "roz_1024" => "1k"
                ,"info_roz_1024" => "",
                "roz_2048" => "2k"
                ,"info_roz_2048" => "",
                "roz_4096" => "4k"
                ,"info_roz_4096" => "",
                "roz_6144" => "6k"
                ,"info_roz_6144" => "",
                "roz_8192" => "8k"
                ,"info_roz_8192" => "",
                "roz_16384" => "16k"
                ,"info_roz_16384" => "",
                "roz_org" => "Original resolution",
                "info_roz_org" => "",

                "numbers_1" => "file",
                "numbers_2" | "numbers_3"=> "files",

                "mgt_input_file" => "Input file",
                "mgt_input_file_r" => "Input file red",
                "mgt_input_file_g" => "Input file green",
                "mgt_input_file_b" => "Input file blue",
                "mgt_input_file_a" => "Input file alpha",
                "mgt_input_folder" => "Output folder",
                "mgt_input_folder_or_file" => "Input file or folder",
                "mgt_output_folder" => "Input folder",
                "mgt_compression_level_label" => "Compression level",
                "mgt_compression_none" => "Without Compression",
                "mgt_filter_label" => "Filter",
                "mgt_file_name" => "File name",
                "mgt_btn_ready" => "Start",
                "mgt_btn_busy_processing" => "Workin'",
                "mgt_btn_busy_processing_other" => "Smth else is goin' on",
                "mgt_btn_gib_data" => "Fill data",
                "mgt_proces_error" => "Error",
                "mgt_interpolation" => "Interpolation",
                "mgt_gen_off" => "Off",

                "ui_main_btn_binary" => "Bin file",
                "ui_main_btn_conversion" => "Conversion",
                "ui_main_btn_merge" => "Merging",
                "ui_main_btn_dds" => "DDS",
                "ui_main_btn_settings" => "Settings",
                "ui_menu_bin_pack" => "Packing",
                "ui_menu_bin_unpack" => "Unpacking",
                "ui_conversion_paths" => "Paths",
                "ui_conversion_extensions" => "Extensions",
                "ui_conversion_resolutions" => "Resolutions",
                "ui_conversion_rest" => "Inne",
                "ui_dds_pack" => "Packing",
                "ui_dds_unpack" => "Unpacking",
                "ui_gen_extensions" => "Extensions",

                "proces_binary_pack_collecting_pending" => "Collected",
                "proces_binary_pack_packing_pending" => "Packing",
                "proces_binary_pack_packing" => "Packed",
                "proces_binary_pack_compression_pending" => "Compression progress",
                "proces_binary_pack_compression" => "Compression finished",
                "proces_binary_pack_encoding_pending" => "Encoding progress",
                "proces_binary_pack_encoding" => "Encoding finished",
                "proces_binary_pack_end" => "Packing process finished in",

                "proces_binary_unpack_files_pending" => "Files",
                "proces_binary_unpack_files" => "Found",
                "proces_binary_unpack_decoding_pending" => "Decoding progress",
                "proces_binary_unpack_decoding" => "Decoded",
                "proces_binary_unpack_decompression" => "Decompressed",
                "proces_binary_unpack_unpacking_pending" => "Unpacking progress",
                "proces_binary_unpack_unpacking" => "Unpacked",
                "proces_binary_unpack_end" => "Unpacking process finished in",

                "proces_conversion_reset_paths" => "Reset paths",
                "proces_conversion_checkbox" => "Output path same as input",
                "proces_conversion_proces_pending" => "Process progress: ",

                "conversion_jpg_prog" => "Progressive",
                "conversion_jpg_sampling" => "Sampling:",
                "conversion_jpg_qua" => "Quant:",
                "conversion_avif_lossless" => "BLossless",
                "mgt_compression" => "Compression:",
                "conversion_avif_chroma" => "Chroma:",
                "conversion_webp_losless" => "Lossless",
                "conversion_noising" => "Noising:",
                "conversion_alpha_color" => "Alpha color:",

                "hint_mgt_btn_ready" => "Start the process",
                "hint_mgt_btn_busy_processing" => "Process is already running",
                "hint_mgt_btn_busy_processing_other" => "Indeed, another process is currently active",
                "hint_mgt_btn_gib_data" => "Something is still missing here",
                "hint_mgt_proces_error" => "Error",
                "hint_ui_main_btn_binary" => "Pack/unpack files",
                "hint_ui_main_btn_conversion" => "Bulk image processing",
                "hint_ui_main_btn_merge" => "Merge multiple images into one",
                "hint_ui_main_btn_dds" => "Tab for .dds file support",
                "hint_ui_main_btn_settings" => "Settings tab",
                "hint_ui_menu_bin_pack" => "Packing files",
                "hint_ui_menu_bin_unpack" => "Unpacking files",
                "hint_ui_conversion_paths" => "Select paths here",
                "hint_ui_conversion_extensions" => "Select extensions here",
                "hint_ui_conversion_resolutions" => "Select resolutions here",
                "hint_ui_conversion_rest" => "Other operations can be found here",
                "hint_binary_pack_process_btn" => "Button to start file packing",
                "hint_binary_pack_choose_out_path_btn" => "Select the output path for the file",
                "hint_binary_pack_choose_in_path_btn" => "Select the input path from which files will be collected for packing",
                "hint_binary_pack_choose_filter" => "File filtration selection menu",
                "hint_binary_pack_choose_compression" => "Select the compression strength",
                "hint_binary_unpack_process_btn" => "Button to start file unpacking",
                "hint_binary_unpack_choose_out_path_btn" => "Select the output path for the unpacked files",
                "hint_binary_unpack_choose_in_path_btn" => "Select the input file to be unpacked ;)",
                "hint_conversion_choose_file_btn" => "Select file",
                "hint_conversion_choose_folder_btn" => "Select folder",
                "hint_conversion_choose_output_folder" => "Select output folder",
                "hint_conversion_choose_output_folder_paths" => "Output folder",
                "hint_conversion_choose_output_folder_paths_same" => "Output folder same as input",
                "hint_conversion_reset_paths" => "Reset input paths",
                "hint_conversion_jpg_prog" => "Toggle progressive encoding",
                "hint_conversion_webp_losless" => "Enable for lossless encoding",
                "hint_conversion_process_btn" => "Button to start image conversion",
                "hint_conversion_process_btn_pending" => "Follow the button cues, but generally it's the start button",
                "hint_conversion_avif_lossless" => "Toggle lossless encoding",


                "log_status_welcome_msg_welcome_morning" => "Good night", // 00:00 to 06:00
                "log_status_welcome_msg_welcome_evening" => "Good evening",
                "log_status_welcome_msg_welcome_day" => "Good morning/Good day",
                "log_status_welcome_msg_today" => "Today is",
                "log_status_welcome_msg_time" => "The time is",
                "log_status_welcome_msg_sys_rdy" => "System ready",
                "log_status_welcome_msg_lang_detected" => "Language detected",

                _ => "Missing translation",
            },
            WybórJęzyka::DE => match klucz {
                "packing_config_menu" => "Verpackungskonfiguration",
                "input_folder" => "Eingabeordner...",
                "output_folder" => "Ausgabeordner...",
                "output_file_name" => "Ausgabedateiname...",
                "packing_start" => "EXPORT STARTEN",
                "skanuj" => "Scannen",
                _ => "Keine Übersetzung",
            },
            WybórJęzyka::ES => match klucz {
                "packing_config_menu" => "Configuración de empaque",
                "input_folder" => "Carpeta de entrada...",
                "output_folder" => "Carpeta de salida...",
                "output_file_name" => "Nombre del archivo...",
                "packing_start" => "INICIAR EXPORTACIÓN",
                "skanuj" => "Escanear",
                _ => "Sin traducción",
            },
            WybórJęzyka::HU => match klucz {
                "packing_config_menu" => "Csomagolási konfiguráció",
                "input_folder" => "Bemeneti mappa...",
                "output_folder" => "Kimeneti mappa...",
                "output_file_name" => "Kimeneti fájlnév...",
                "packing_start" => "EXPORTÁLÁS INDÍTÁSA",
                "skanuj" => "Szkennelés",
                _ => "Nincs fordítás",
            },
            WybórJęzyka::JP => match klucz {
                "main_toggle_export" => "パッキング",
                "main_toggle_import" => "アンパッキング",
                "main_toggle_photo_compil" => "画像編集",
                "main_toggle_photo_merge" => "画像結合",

                "packing_config_menu" => "パッキング設定",
                "input_folder" => "入力フォルダ...",
                "output_folder" => "出力フォルダ...",
                "output_file_name" => "出力ファイル名...",
                "packing_start" => "エクスポート開始",
                "compression_level_label" => "圧縮",
                "file_filter_label" => "フィルタリング",
                "skanuj" => "スキャン",
                "files_counting_numbers_1" => "個のファイル",
                "files_counting_numbers_2" => "個のファイル",
                "files_counting_numbers_3" => "個のファイル",
                "status_note_started" => "開始",
                "status_note_pending" => "処理中",
                "status_note_io" => "I/O処理",
                "status_note_finished" => "完了",
                "comp_none" => "圧縮なし",
                "comp_std" => "標準",
                "comp_max" => "最大",
                "filter_all" => "すべて",
                "filter_graphic" => "画像",
                "filter_audio" => "音声",
                "filter_text" => "テキスト",
                "filter_pdf" => "PDF",

                "foto_edit_menu_resolution" => "解像度",
                "foto_edit_menu_rest" => "その他",
                "foto_edit_menu_extensions" => "拡張子",
                "foto_edit_menu_paths" => "パス",

                "general_off" => "無効",

                "foto_edit_noising" => "ノイズ付加",

                "foto_edit_interpolation" => "補完法",
                "foto_edit_color" => "カラー",
                "foto_edit_quality" => "画質",

                "" => "",

                "console_menu_reset" => "リセット",
                "console_menu_status" => "ステータス:",

                "log_status_welcome_msg_welcome" => "こんにちは",
                "log_status_welcome_msg_today" => "本日は",
                "log_status_welcome_msg_time" => "時刻",
                "log_status_welcome_msg_sys_rdy" => "システム準備完了",
                "log_status_welcome_msg_lang_detected" => "システム言語を検出しました",
                "log_status_" => "",

                "log_status_critical_error" => "致命的なエラー",
                "log_status_operation_unpacking" => "アンパッキング",
                "log_status_operation_decompression" => "解凍中",
                "log_status_finished" => "完了",
                "log_status_started" => "開始",
                "log_status_in_time" => "経過時間",

                "foto_edit_tooltip_jpg_color" => "カラー JPG",
                "foto_edit_tooltip_jpg_bw" => "モノクロ JPG",

                "foto_edit_resolution_oryginal" => "オリジナル",

                "btn_proces_avaliable" => "実行",
                "btn_proces_in_progress" => "処理中...",
                "btn_proces_in_progress_other" => "他のプロセスが実行中です",
                "btn_proces_lack_of_data" => "データ不足",

                "OptInterpolacja_nearest" => "最近傍補間",
                "OptInterpolacja_triangle" => "トライアングル",
                "OptInterpolacja_catmull" => "Catmull-Rom法",
                "OptInterpolacja_gaussian" => "ガウスぼかし",
                "OptInterpolacja_lanczos" => "ランチョス法",
                _ => "翻訳なし",
            },
            WybórJęzyka::KR => match klucz {
                "main_toggle_export" => "패킹",
                "main_toggle_import" => "언패킹",
                "main_toggle_photo_compil" => "사진 편집",
                "main_toggle_photo_merge" => "사진 합치기",

                "packing_config_menu" => "패킹 설정",
                "input_folder" => "입력 폴더...",
                "output_folder" => "출력 폴더...",
                "output_file_name" => "출력 파일명...",
                "packing_start" => "내보내기 시작",
                "compression_level_label" => "압축",
                "file_filter_label" => "필터링",
                "skanuj" => "스캔",
                "files_counting_numbers_1" => "개의 파일",
                "files_counting_numbers_2" => "개의 파일",
                "files_counting_numbers_3" => "개의 파일",
                "status_note_started" => "시작됨",
                "status_note_pending" => "대기 중",
                "status_note_io" => "I/O 작업",
                "status_note_finished" => "완료됨",
                "comp_none" => "압축 없음",
                "comp_std" => "표준",
                "comp_max" => "최대",
                "filter_all" => "전체",
                "filter_graphic" => "그래픽",
                "filter_audio" => "오디오",
                "filter_text" => "텍스트",
                "filter_pdf" => "PDF",

                "foto_edit_menu_resolution" => "해상도",
                "foto_edit_menu_rest" => "기타",
                "foto_edit_menu_extensions" => "확장자",
                "foto_edit_menu_paths" => "경로",

                "general_off" => "비활성화",

                "foto_edit_noising" => "노이즈 추가",

                "foto_edit_interpolation" => "보간법",
                "foto_edit_color" => "컬러",
                "foto_edit_quality" => "품질",

                "" => "",

                "console_menu_reset" => "초기화",
                "console_menu_status" => "상태:",

                "log_status_welcome_msg_welcome" => "안녕하세요",
                "log_status_welcome_msg_today" => "오늘은",
                "log_status_welcome_msg_time" => "시간",
                "log_status_welcome_msg_sys_rdy" => "시스템 준비 완료",
                "log_status_welcome_msg_lang_detected" => "시스템 언어가 감지되었습니다",
                "log_status_" => "",

                "log_status_critical_error" => "치명적 오류",
                "log_status_operation_unpacking" => "언패킹",
                "log_status_operation_decompression" => "압축 해제 중",
                "log_status_finished" => "종료",
                "log_status_started" => "시작",
                "log_status_in_time" => "소요 시간",

                "foto_edit_tooltip_jpg_color" => "컬러 JPG",
                "foto_edit_tooltip_jpg_bw" => "흑백 JPG",

                "foto_edit_resolution_oryginal" => "원본",

                "btn_proces_avaliable" => "시작",
                "btn_proces_in_progress" => "처리 중...",
                "btn_proces_in_progress_other" => "다른 작업이 진행 중입니다",
                "btn_proces_lack_of_data" => "데이터 부족",

                "OptInterpolacja_nearest" => "최근접 이웃",
                "OptInterpolacja_triangle" => "트라이앵글",
                "OptInterpolacja_catmull" => "Catmull-Rom",
                "OptInterpolacja_gaussian" => "가우시안",
                "OptInterpolacja_lanczos" => "란초스",
                _ => "번역 없음",
            },
            WybórJęzyka::TH => match klucz {
                "main_toggle_export" => "การบรรจุไฟล์",
                "main_toggle_import" => "การแตกไฟล์",
                "main_toggle_photo_compil" => "แก้ไขรูปภาพ",
                "main_toggle_photo_merge" => "รวมรูปภาพ",

                "packing_config_menu" => "ตั้งค่าการบรรจุไฟล์",
                "input_folder" => "โฟลเดอร์ขาเข้า...",
                "output_folder" => "โฟลเดอร์ขาออก...",
                "output_file_name" => "ชื่อไฟล์ผลลัพธ์...",
                "packing_start" => "เริ่มส่งออก",
                "compression_level_label" => "การบีบอัด",
                "file_filter_label" => "การกรองไฟล์",
                "skanuj" => "สแกน",
                "files_counting_numbers_1" => "ไฟล์",
                "files_counting_numbers_2" => "ไฟล์",
                "files_counting_numbers_3" => "ไฟล์",
                "status_note_started" => "เริ่มแล้ว",
                "status_note_pending" => "รอดำเนินการ",
                "status_note_io" => "การดำเนินการ I/O",
                "status_note_finished" => "เสร็จสิ้น",
                "comp_none" => "ไม่มีการบีบอัด",
                "comp_std" => "มาตรฐาน",
                "comp_max" => "สูงสุด",
                "filter_all" => "ทั้งหมด",
                "filter_graphic" => "กราฟิก",
                "filter_audio" => "เสียง",
                "filter_text" => "ข้อความ",
                "filter_pdf" => "PDF",

                "foto_edit_menu_resolution" => "ความละเอียด",
                "foto_edit_menu_rest" => "อื่นๆ",
                "foto_edit_menu_extensions" => "นามสกุลไฟล์",
                "foto_edit_menu_paths" => "เส้นทางไฟล์",

                "general_off" => "ปิดใช้งาน",

                "foto_edit_noising" => "เพิ่มสัญญาณรบกวน",

                "foto_edit_interpolation" => "การประมาณค่าในช่วง",
                "foto_edit_color" => "สี",
                "foto_edit_quality" => "คุณภาพ",

                "" => "",

                "console_menu_reset" => "รีเซ็ต",
                "console_menu_status" => "สถานะ:",

                "log_status_welcome_msg_welcome" => "สวัสดี",
                "log_status_welcome_msg_today" => "วันนี้คือ",
                "log_status_welcome_msg_time" => "เวลา",
                "log_status_welcome_msg_sys_rdy" => "ระบบพร้อมใช้งาน",
                "log_status_welcome_msg_lang_detected" => "ตรวจพบภาษาของระบบ",
                "log_status_" => "",

                "log_status_critical_error" => "ข้อผิดพลาดร้ายแรง",
                "log_status_operation_unpacking" => "การแตกไฟล์",
                "log_status_operation_decompression" => "การคลายบีบอัด",
                "log_status_finished" => "เสร็จสิ้น",
                "log_status_started" => "เริ่มแล้ว",
                "log_status_in_time" => "ในเวลา",

                "foto_edit_tooltip_jpg_color" => "JPG สี",
                "foto_edit_tooltip_jpg_bw" => "JPG ขาวดำ",

                "foto_edit_resolution_oryginal" => "ต้นฉบับ",

                "btn_proces_avaliable" => "เริ่ม",
                "btn_proces_in_progress" => "กำลังดำเนินการ...",
                "btn_proces_in_progress_other" => "มีกระบวนการอื่นทำงานอยู่",
                "btn_proces_lack_of_data" => "ข้อมูลไม่ครบถ้วน",

                "OptInterpolacja_nearest" => "จุดใกล้ที่สุด",
                "OptInterpolacja_triangle" => "สามเหลี่ยม",
                "OptInterpolacja_catmull" => "Catmull-Rom",
                "OptInterpolacja_gaussian" => "เกาส์เซียน",
                "OptInterpolacja_lanczos" => "Lanczos",
                _ => "ไม่มีคำแปล",
            },
        }
    }
}
