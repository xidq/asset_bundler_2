use iced::futures::channel::mpsc::Sender;
use iced::futures::SinkExt;
use crate::statusy::Logi;
use crate::log_file_gen::generuj_plik_logow;

// pub async fn wyslij_status<T>(mut tx: Sender<T>)where T:Logi{
//     tx
//         .send(
//             LogTxBinPak::StatusZnaleziono {
//                 pliki: licznik,
//             },
//         )
//         .await;
//
// }
pub async fn loguj<T: Logi>(tx: &mut Sender<T>, log: T) {
    if tx.send(log).await.is_err() {
        eprintln!("Odbiorca logów rozłączony.");
        generuj_plik_logow(format!("{} Odbiorca logów rozłączony.", T::nazwa()));
    }
}

/// # Send status
/// fn on 'futures' mpsc channel that can use trait "Logi".
/// 
/// If there's None, then nothing will happen,
/// if Some() is used then such thing will be sent.
/// 
/// Ofc there's log generation if error and console eprintln! inside
pub async fn wyslij_status<T: Logi>(tx: &mut Sender<T>, log: Option<T>) {
    if let Some(l) = log {
        loguj(tx, l).await;
    }
}