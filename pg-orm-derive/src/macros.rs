macro_rules! continue_token_stream {
    ($v:expr, $errors:ident) => {
        match $v {
            Ok(v) => v,
            Err(e) => {
                $errors.push(e);
                continue;
            }
        }
    };
}

macro_rules! try_token_stream {
    ($v:expr) => {
        match $v {
            Ok(v) => v,
            Err(e) => return e,
        }
    };
}
