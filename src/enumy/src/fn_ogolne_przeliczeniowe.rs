use std::time::Instant;

pub fn przelicz_czas(xx:Instant) ->String{
    let trwanie = xx.elapsed();
    match trwanie.as_secs(){
        0..= 60 => { format!("{:.2?}", trwanie) },
        61..= 3600 => { format!("{}min : {}sec",trwanie.as_secs() / 60, trwanie.as_secs() % 60 ) },
        36001..= 86400 => { format!("{}h : {}min",trwanie.as_secs() / 3600, (trwanie.as_secs() / 60) % 60 ) }
        _ => { format!("{}d : {}h",trwanie.as_secs() / 86400, (trwanie.as_secs() / 3600) % 24 ) },
    }
}
pub fn przelicz_bajty(val:impl Into<u64> + std::clone::Clone )->String{
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