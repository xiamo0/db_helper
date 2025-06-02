use crate::{args_util, sqlx_util};

pub fn run() -> Result<(), String> {
    let result = args_util::run();

    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }
    sqlx_util::run(&result?)
}
