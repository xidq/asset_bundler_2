use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use futures::SinkExt;

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
    }
}

pub async fn wyslij_status<T: Logi>(tx: &mut Sender<T>, log: Option<T>) {
    if let Some(l) = log {
        loguj(tx, l).await;
    }
}