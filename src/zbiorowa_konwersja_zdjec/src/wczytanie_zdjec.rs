use futures::SinkExt;
use futures::channel::mpsc::Sender;
use image::ImageDecoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use enumy::statusy::LogTxDoBathKonwersjaZdjęć;



pub async fn aktualizuj_postep(
    obecna_op: &Arc<Mutex<u32>>,
    metryka: u32,
    nadawca: &mut Sender<LogTxDoBathKonwersjaZdjęć>,
) {
    async move {
        let mut oopr = obecna_op.lock().await;
        *oopr += 1;
        let obecnie = *oopr;
        drop(oopr);
        
        let _ = nadawca
            .send(
                LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćRozpoczęto(
                    obecnie,
                    Some(metryka),
                ),
            )
            .await;
        
    }
    .await;
}
