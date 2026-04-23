use enumy::inne_ui::StronyDds;
use enumy::opcje::{OptFormatDds, OptKompresjaDds};
use enumy::statusy::LogTxDoPakowanieDds;

#[derive(Debug, Clone)]
pub enum DdsMessage {
    ZmienMenuDds(StronyDds),
    DDS_Pakowanie_ZmianaŚcieżkiWejściowej(String),
    DDS_Pakowanie_ZmianaŚcieżkiWejściowejWybór,
    DDS_Pakowanie_ZmianaŚcieżkiWyjściowej(String),
    DDS_Pakowanie_ZmianaŚcieżkiWyjściowejWybór,
    DDS_Pakowanie_ZmianaWybranegoFormatu(OptFormatDds),
    DDS_Pakowanie_ZmianaWybranejKompresji(OptKompresjaDds),
    DdsPakowanieZmianaNazwy(String),
    DdsPakowanieWysylanieDanych,
    DdsPakowaniePostep(LogTxDoPakowanieDds),
    DdsrozpakowanieZmianaŚcieżkiWejściowej(String),
    DdsrozpakowanieZmianaŚcieżkiWejściowejWybór,
    DdsRozpakowanieZmianaŚcieżkiWyjściowej(String),
    DdsRozpakowanieZmianaŚcieżkiWyjściowejWybór,
    Nic,
}