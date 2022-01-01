macro_rules! try_parse {
    ($decode_fn: ident, $bytes: expr) => {
        match $decode_fn($bytes) {
            Ok((Some(v), b)) => return Ok((v, b)),
            Err(e) => return Err(e),
            _ => {}
        }
    };
}
