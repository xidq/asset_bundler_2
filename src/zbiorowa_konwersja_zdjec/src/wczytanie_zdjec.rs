use futures::channel::mpsc::Sender;
use futures::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use enumy::statusy::LogTxKonw;


pub async fn aktualizuj_postep(
    obecna_op: &Arc<Mutex<u32>>,
    metryka: u32,
    nadawca: &mut Sender<LogTxKonw>,
) {
    async move {
        let mut oopr = obecna_op.lock().await;
        *oopr += 1;
        let obecnie = *oopr;
        drop(oopr);
        
        let _ = nadawca
            .send(
                LogTxKonw::Rozpoczęto(
                    obecnie,
                    Some(metryka),
                ),
            )
            .await;
        
    }
    .await;
}
