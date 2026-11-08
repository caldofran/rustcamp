use core::{
    option::Option::{self, None, Some},
    result::Result::{Err, Ok},
    todo,
};
use std::env;

fn main() {
    println!("Implement me!");
}

fn conf_from_args() -> Result<Option<String>, &'static str> {
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--conf" {
            let path = args.next().ok_or("--conf required a path")?;
            if path.is_empty() {
                return Err("--conf must not be empty");
            }
            return Ok(Some(path));
        }
    }

    Ok(None)
}

fn conf_from_env_var() -> Result<Option<String>, &'static str> {
    env::var("APP_CONF").ok_or("Not valid Unicode")?
}
