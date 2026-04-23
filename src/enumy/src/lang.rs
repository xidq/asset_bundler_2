use std::fmt;

// use crate::enums_structs_io::ProcesStatus;
use crate::wybranie_jezykowe::WybórJęzyka;

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

impl WybórJęzyka {
    pub fn t(&self, klucz: &str) -> &'static str {
        match self {
            WybórJęzyka::PL => match klucz {
                "main_toggle_export" => "Pakowanie",
                "main_toggle_import" => "Rozpakowywanie",
                "main_toggle_photo_compil" => "Edycja foto",
                "main_toggle_photo_merge" => "Łączenie zdjęć",

                "packing_config_menu" => "Konfiguracja Pakowania",
                "input_folder" => "Folder wejściowy...",
                "input_file" => "Plik wejściowy...",
                "input_folder_or_file" => "Folder lub plik wejściowy...",
                "output_folder" => "Folder wyjściowy...",
                "output_file_name" => "Nazwa pliku wynikowego...",
                "packing_start" => "ROZPOCZNIJ EKSPORT",
                "compression_level_label" => "Kompresja",
                "file_filter_label" => "Filtrowanie",
                "skanuj" => "Skanuj",
                "files_counting_numbers_1" => "plik",
                "files_counting_numbers_2" => "pliki",
                "files_counting_numbers_3" => "plików",
                "status_note_started" => "Rozpoczęto",
                "status_note_pending" => "W trakcie",
                "status_note_io" => "operacje I/O",
                "status_note_finished" => "Zakończono",
                "comp_none" => "Brak Kompresji",
                "comp_std" => "Standardowa",
                "comp_max" => "Maksymalna",
                "filter_all" => "Wszystko",
                "filter_graphic" => "Graficzne",
                "filter_audio" => "Audio",
                "filter_text" => "Tekstowe",
                "filter_pdf" => "PDF",

                "foto_edit_menu_resolution" => "Rozdzielczość",
                "foto_edit_menu_rest" => "Reszta",
                "foto_edit_menu_extensions" => "rozszerzenia_plików_zdjęciowych",
                "foto_edit_menu_paths" => "Ścieżki",

                "general_off" => "wyłączone",

                "foto_edit_noising" => "OptEfektZaszumiania",

                "foto_edit_interpolation" => "OptInterpolacja",
                "foto_edit_color" => "Kolor",
                "foto_edit_quality" => "Jakość",

                "" => "",

                "console_menu_reset" => "Resetuj",
                "console_menu_status" => "Status:",

                "btn_bussy_processing_other" => "Inny proces trwa",
                "btn_bussy_processing" => "W trakcie",
                "btn_gib_data" => "Uzupełnij dane",

                "un_packing_btn_start" => "Rozpocznij rozpakowywanie",
                "packing_btn_start" => "Rozpocznij pakowanie",
                "process_btn_start" => "Rozpocznij przetwarzanie",

                "log_status_welcome_msg_welcome_evening" => "Dobry wieczór",
                "log_status_welcome_msg_welcome_morning" => "Dzień dobry",
                "log_status_welcome_msg_welcome_day" => "Dzień dobry",
                "log_status_welcome_msg_today" => "Dziś jest",
                "log_status_welcome_msg_time" => "Godzina",
                "log_status_welcome_msg_sys_rdy" => "System gotowy",
                "log_status_welcome_msg_lang_detected" => "Wykryto język systemu",
                "log_status_" => "",

                "log_status_critical_error" => "BŁĄD KRYTYCZNY",
                "log_status_operation_unpacking" => "Rozpakowywanie",
                "log_status_operation_decompression" => "StatusDekompresjaPlikówDekompresja",
                "log_status_finished" => "Zakończono",
                "log_status_started" => "Rozpoczęto",
                "log_status_in_time" => "W czasie",

                "foto_edit_tooltip_jpg_color" => "Kolorowy jpg",
                "foto_edit_tooltip_jpg_bw" => "Szary jpg",

                "foto_edit_resolution_oryginal" => "Bazowa",

                "btn_proces_avaliable" => "Rozpocznij",
                "btn_proces_in_progress" => "Proces w toku...",
                "btn_proces_in_progress_other" => "Inny proces trwa",
                "btn_proces_lack_of_data" => "Uzupełnij dane",

                "OptInterpolacja_nearest" => "nearest",
                "OptInterpolacja_triangle" => "triangle",
                "OptInterpolacja_catmull" => "catmull",
                "OptInterpolacja_gaussian" => "gaussian",
                "OptInterpolacja_lanczos" => "lanczos",

                _ => "Brak tłumaczenia",
            },
            WybórJęzyka::EN => match klucz {
                "main_toggle_export" => "Packing",
                "main_toggle_import" => "Unpacking",
                "main_toggle_photo_compil" => "Photo Edit",
                "main_toggle_photo_merge" => "Photo Merge",

                "packing_config_menu" => "Packing Configuration",
                "input_folder" => "Input folder...",
                "output_folder" => "Output folder...",
                "output_file_name" => "Output file name...",
                "packing_start" => "START EXPORT",
                "compression_level_label" => "Compression",
                "file_filter_label" => "Filtering",
                "skanuj" => "Scan",
                "files_counting_numbers_1" => "file",
                "files_counting_numbers_2" => "files",
                "files_counting_numbers_3" => "files",
                "status_note_started" => "Started",
                "status_note_pending" => "In progress",
                "status_note_io" => "I/O operations",
                "status_note_finished" => "Finished",
                "comp_none" => "No Compression",
                "comp_std" => "Standard",
                "comp_max" => "Maximum",
                "filter_all" => "All",
                "filter_graphic" => "Graphics",
                "filter_audio" => "Audio",
                "filter_text" => "Text",
                "filter_pdf" => "PDF",

                "foto_edit_menu_resolution" => "Resolution",
                "foto_edit_menu_rest" => "Others",
                "foto_edit_menu_extensions" => "Extensions",
                "foto_edit_menu_paths" => "Paths",

                "general_off" => "disabled",

                "foto_edit_noising" => "Dithering / Noise",

                "foto_edit_interpolation" => "Interpolation",
                "foto_edit_color" => "Color",
                "foto_edit_quality" => "Quality",

                "" => "",

                "console_menu_reset" => "Reset",
                "console_menu_status" => "Status:",

                "log_status_welcome_msg_welcome" => "Welcome",
                "log_status_welcome_msg_today" => "Today is",
                "log_status_welcome_msg_time" => "Time",
                "log_status_welcome_msg_sys_rdy" => "System ready",
                "log_status_welcome_msg_lang_detected" => "System language detected",
                "log_status_" => "",

                "log_status_critical_error" => "CRITICAL ERROR",
                "log_status_operation_unpacking" => "Unpacking",
                "log_status_operation_decompression" => "Decompression",
                "log_status_finished" => "Finished",
                "log_status_started" => "Started",
                "log_status_in_time" => "In time",

                "foto_edit_tooltip_jpg_color" => "Color JPG",
                "foto_edit_tooltip_jpg_bw" => "Grayscale JPG",

                "foto_edit_resolution_oryginal" => "Original",

                "btn_proces_avaliable" => "Start",
                "btn_proces_in_progress" => "Processing...",
                "btn_proces_in_progress_other" => "Another process running",
                "btn_proces_lack_of_data" => "Missing czcionki",

                "OptInterpolacja_nearest" => "Nearest",
                "OptInterpolacja_triangle" => "Triangle",
                "OptInterpolacja_catmull" => "Catmull-Rom",
                "OptInterpolacja_gaussian" => "Gaussian",
                "OptInterpolacja_lanczos" => "Lanczos",
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
