use std::sync::{Arc, mpsc};
use std::time::Instant;
// use crate::zmiana_fot::{DaneDoBathKonwersjaZdjec, LogTxDoBathKonwersjaZdjęć, rozszerzenia_plików_zdjęciowych};

mod edycja_ff;
mod edycja_jpg;
mod edycja_png;
mod edycja_qoi;
mod edycja_tga;
mod edycja_webp;
pub mod pomocnicze;
mod wczytanie_zdjec;
pub mod zmiana_fot;
