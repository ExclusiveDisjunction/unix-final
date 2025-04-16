use std::path::PathBuf;

pub const PROG_NAME: &str = "unix-back";
pub const LOG_DIR: &str = "./logs";

pub fn make_log_path() -> PathBuf {
    let postfix = format!("{}-run-{}.log", PROG_NAME, chrono::Local::now());
    let base = PathBuf::from(LOG_DIR);

    base.join(postfix)
}