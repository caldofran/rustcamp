use core::{
    option::Option::{self, None, Some},
    result::Result::{Err, Ok},
    todo,
};
use std::{borrow::Cow, default, env};

fn main() {
    let args = conf_from_args();
    let path: Cow<str> = match args {
        Ok(Some(path)) => path.into(),
        Ok(None) => match conf_from_env_var() {
            Ok(Some(path)) => path.into(),
            _ => "/etc/app/app.conf".into(),
        },
        Err(error) => panic!("Found error {}", error),
    };
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

fn conf_from_env_var() -> Result<Option<String>, String> {
    env::var("APP_CONF")
        .map(|var| {
            if var.trim().is_empty() {
                None
            } else {
                Some(var)
            }
        })
        .map_err(|error| format!("Found error {}", error))
}
