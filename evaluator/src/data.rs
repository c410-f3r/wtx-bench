use std::sync::OnceLock;
use wtx::misc::{NoStdRng, Rng};

pub(crate) fn string_bytes_8kib() -> &'static Vec<u8> {
    static POOL: OnceLock<Vec<u8>> = OnceLock::new();
    POOL.get_or_init(|| {
        let mut rslt = vec![0; 1024 * 8];
        string_bytes(&mut rslt);
        rslt
    })
}

pub(crate) fn string_bytes_2mib() -> &'static Vec<u8> {
    static POOL: OnceLock<Vec<u8>> = OnceLock::new();
    POOL.get_or_init(|| {
        let mut rslt = vec![0; 1024 * 1024 * 2];
        string_bytes(&mut rslt);
        rslt
    })
}

fn string_bytes(slice: &mut [u8]) {
    let mut rng = NoStdRng::default();
    for elem in slice {
        loop {
            let byte = rng.u8();
            if byte.is_ascii_alphanumeric() {
                *elem = byte;
                break;
            }
        }
    }
}
