use std::sync::OnceLock;
use wtx::rng::{Rng, StaticRng};

const _8KIB: usize = 1024 * 8;
const _4MIB: usize = 1024 * 1024 * 2;

pub(crate) fn string_bytes_8kib() -> &'static [u8] {
    static POOL: OnceLock<Vec<u8>> = OnceLock::new();
    POOL.get_or_init(|| {
        let mut rslt = vec![0; _8KIB];
        string_bytes(&mut rslt);
        rslt
    })
    .as_slice()
}

pub(crate) fn string_bytes_2mib() -> &'static [u8] {
    static POOL: OnceLock<Vec<u8>> = OnceLock::new();
    POOL.get_or_init(|| {
        let mut rslt = vec![0; _4MIB];
        string_bytes(&mut rslt);
        rslt
    })
    .as_slice()
}

fn string_bytes(slice: &mut [u8]) {
    let mut rng = StaticRng::default();
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
