use std::path::Path;
/// # Check avaliable space
/// We're estimating if such georgeous binary fille have possibility of being created on designed location.
/// 
/// There's smth for Linux and Windows, nothin' for Mac users
pub fn chck_av_spc_n_stuff(ghhh: &Path) -> Result<(), std::io::Error> {
    let mut lista_plików = Vec::new();
    let mut foldery_do_przejrzenia = vec![ghhh.to_path_buf()];

    while let Some(aktualny_folder) = foldery_do_przejrzenia.pop() {
        let wejścia = read_dir(aktualny_folder)?;

        // Zamiast next_entry() używamy standardowego pętli for lub .next() na iteratorze
        for entry_result in wejścia {
            // Każdy element iteratora to Result<DirEntry, Error>, więc wyciągamy go przez `?`
            let entry = entry_result?;
            let path = entry.path();

            if path.is_dir() {
                foldery_do_przejrzenia.push(path);
            } else if path.is_file() {
                lista_plików.push(path);
            }
        }
    }


    let calkowita_waga = lista_plików.iter().try_fold(0u64, |acc, hhh| {
        std::fs::metadata(hhh).map(|meta| acc + meta.len())
    })?;



    #[cfg(target_os = "linux")]
    let dostepne_miejsce = pobierz_wolne_miejsce_linux(&ghhh.to_string_lossy());

    #[cfg(target_os = "windows")]
    let dostepne_miejsce = pobierz_wolne_miejsce_windows(&ghhh.to_string_lossy());

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    let dostepne_miejsce = Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Niewspierany OS"));

    let bufor_bezpieczenstwa = 10 * 1024 * 1024;

    match dostepne_miejsce {
        Ok(wolne_bajty) => {
            if wolne_bajty > (calkowita_waga + bufor_bezpieczenstwa){
                Ok(())
            } else { Err(std::io::Error::new(std::io::ErrorKind::OutOfMemory,"Brak miejsca")) }
        }
        Err(e) => {
            eprintln!("Nie udało się zweryfikować przestrzeni dyskowej: {}", e);
            Err(e)
        }
    }
}
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::mem;
#[cfg(target_os = "linux")]
use std::os::raw::{c_char, c_ulong};

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Debug)]
struct Statvfs {
    f_bsize: c_ulong,
    f_frsize: c_ulong,
    f_blocks: c_ulong,
    f_bfree: c_ulong,
    f_bavail: c_ulong, // Bloki dostępne non-root aka usr
    f_files: c_ulong,
    f_ffree: c_ulong,
    f_favail: c_ulong,
    f_fsid: c_ulong,
    f_flag: c_ulong,
    f_namemax: c_ulong,
    _padding: [c_int; 6], // Zapewnia odpowiedni rozmiar struktury w pamięci
}
#[cfg(target_os = "linux")]
#[allow(non_camel_case_types)]
type c_int = std::os::raw::c_int;
#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn statvfs(path: *const c_char, buf: *mut Statvfs) -> c_int;
}

#[cfg(target_os = "linux")]
pub fn pobierz_wolne_miejsce_linux(sciezka: &str) -> Result<u64, std::io::Error> {
    let c_sciezka = CString::new(sciezka)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Ścieżka zawiera znak null"))?;

    let mut buf: Statvfs = unsafe { mem::zeroed() };

    let wynik = unsafe { statvfs(c_sciezka.as_ptr(), &mut buf) };

    if wynik == 0 {
        // Używamy f_frsize (rozmiar fragmentu), który jest poprawnym mnożnikiem dla bloków
        let wolne_bajty = buf.f_bavail as u64 * buf.f_frsize as u64;
        Ok(wolne_bajty)
    } else {
        Err(std::io::Error::last_os_error())
    }
}




// ---
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::fs;
use std::fs::read_dir;
#[cfg(target_os = "windows")]
use std::fs::read_dir;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

// use crate::statusy::LogTxBinPak;

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const u16,
        lpFreeBytesAvailableToCaller: *mut u64, // To nas interesuje (uwzględnia limity/kwoty)
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> i32;
}

/// Zwraca wolne miejsce dostępne dla użytkownika na partycji,
/// na której znajduje się podana ścieżka (w bajtach).
#[cfg(target_os = "windows")]
pub fn pobierz_wolne_miejsce_windows(sciezka: &str) -> Result<u64, std::io::Error> {
    // Windows wymaga formatu UTF-16 zakończonego zerem
    let os_str = OsStr::new(sciezka);
    let mut sciezka_utf16: Vec<u16> = os_str.encode_wide().collect();
    sciezka_utf16.push(0);

    let mut wolne_dla_uzytkownika: u64 = 0;
    let mut calkowite: u64 = 0;
    let mut wolne_calkowite: u64 = 0;

    let wynik = unsafe {
        GetDiskFreeSpaceExW(
            sciezka_utf16.as_ptr(),
            &mut wolne_dla_uzytkownika,
            &mut calkowite,
            &mut wolne_calkowite,
        )
    };

    if wynik != 0 {
        Ok(wolne_dla_uzytkownika)
    } else {
        Err(std::io::Error::last_os_error())
    }
}