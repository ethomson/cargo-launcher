mod alfred;
mod args;
mod cargo;
mod core;
mod error;
mod file;
mod hain;
mod io;
mod launcher;
mod tpl;

use log::debug;
use pretty_env_logger;
use std::process::exit;

use crate::args::args;
use crate::cargo::config;
use crate::error::Result;
use crate::launcher::launch;

const SUCCESS_CODE: i32 = 0;
const FAILED_CODE: i32 = 1;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = args();
    debug!("args: {:?}", args);
    let config = config()?;

    match launch(&args, &config) {
        Ok(_) => exit(SUCCESS_CODE),
        Err(err) => {
            eprintln!("Error occurred: {}", err);
            exit(FAILED_CODE);
        }
    }
}
