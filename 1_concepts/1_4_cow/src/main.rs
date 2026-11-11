use core::result::Result::{Err, Ok};
use std::env;

fn main() {
    let args = env::args();
    let env_var = env::var("APP_CONF");
    match config_path::parse_path(args, env_var) {
        Ok(path) => println!("Found path {}", path),
        Err(_) => panic!(),
    }
}

mod config_path {
    use core::cfg;
    use core::{option::Option::None, result::Result, todo};
    use std::borrow::Cow;
    use std::env::{Args, VarError};

    pub enum Error {}

    pub fn parse_path(args: Args, env_var: Result<String, VarError>) -> Result<String, Error> {
        let cli_path = from_args(args);
        let path: Cow<str> = match cli_path {
            Ok(Some(path)) => path.into(),
            Ok(None) => match from_env_var(env_var) {
                Ok(Some(path)) => path.into(),
                _ => "/etc/app/app.conf".into(),
            },
            Err(error) => panic!("Found error {}", error),
        };
        todo!()
    }

    fn from_args(args: Args) -> Result<Option<String>, &'static str> {
        let mut mut_args = args.skip(1);
        while let Some(arg) = mut_args.next() {
            if arg == "--conf" {
                let path = mut_args.next().ok_or("--conf required a path")?;
                if path.is_empty() {
                    return Err("--conf must not be empty");
                }
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    fn from_env_var(env_var: Result<String, VarError>) -> Result<Option<String>, String> {
        env_var
            .map(|var| {
                if var.trim().is_empty() {
                    None
                } else {
                    Some(var)
                }
            })
            .map_err(|error| format!("Found error {}", error))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_name() {}
    }
}
