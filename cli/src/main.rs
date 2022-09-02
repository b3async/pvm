mod actions;
mod context;
mod error;

use crate::context::Context;
use std::error::Error;
use structopt::StructOpt;

#[macro_export]
macro_rules! pvm_path {
    ($( $path:expr ),+) => {{
        let home_dir = std::env::var("HOME").expect("unable to find HOME environment variable");
        let mut path: std::path::PathBuf = [home_dir.as_str()].iter().collect();

        path.push(".pvm");

        $(
            path.push($path);
        )+

        path.clone()
    }};

    ($path:expr) => {
        pvm_path!($path,)
    };
}

#[macro_export]
macro_rules! pvm_build_path {
    () => {
        pvm_path!("builds")
    };
}

#[macro_export]
macro_rules! pvm_versions_path {
    () => {
        pvm_path!("versions")
    };
}

#[derive(StructOpt, PartialEq, Eq, Debug)]
#[structopt(name = "PVM", about = "A PHP Version Manager")]
enum Command {
    #[structopt(about = "List all PHP versions currently installed")]
    LS,
    #[structopt(about = "Inspect a PHP installation")]
    Inspect { version: String },
    #[structopt(about = "Remove a PHP installation")]
    Remove { version: String },
    #[structopt(about = "Install a PHP version")]
    Add { version: String },
    #[structopt(about = "Activate a PHP version")]
    Use { version: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = Context::default();
    ctx.init()?;

    let command: Command = Command::from_args();

    match command {
        Command::LS => actions::list_installations(&ctx),
        Command::Inspect { version } => actions::inspect_installation(version, &ctx),
        Command::Remove { version } => actions::remove_installation(version, &ctx),
        Command::Add { version } => actions::add_installation(version, &ctx),
        Command::Use { version } => actions::activate_installation(version, &ctx),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_pvm_paths() {
        let home_dir = std::env::var("HOME").unwrap();

        assert_eq!(
            format!("{home_dir}/.pvm/whatever.tar.gz").as_str(),
            pvm_path!("whatever.tar.gz").as_path().to_str().unwrap()
        );

        assert_eq!(
            format!("{home_dir}/.pvm/builds/php-8.1.9/source.tar.gz").as_str(),
            pvm_path!("builds/php-8.1.9/source.tar.gz")
                .as_path()
                .to_str()
                .unwrap()
        );
    }

    #[test]
    fn pvm_folder_paths() {
        let home_dir = std::env::var("HOME").unwrap();

        let build_path = pvm_build_path!();
        let versions_path = pvm_versions_path!();

        assert_eq!(
            format!("{home_dir}/.pvm/builds").as_str(),
            build_path.as_path().to_str().unwrap()
        );

        assert_eq!(
            format!("{home_dir}/.pvm/versions").as_str(),
            versions_path.as_path().to_str().unwrap()
        );
    }

    #[test]
    fn can_parse_ls_cmd() {
        assert_eq!(Command::from_iter(&["pvm", "ls"]), Command::LS);
    }

    #[test]
    fn can_parse_inspect_cmd() {
        assert_eq!(
            Command::from_iter(&["pvm", "inspect", "8.1.9"]),
            Command::Inspect {
                version: "8.1.9".to_owned()
            }
        );
    }

    #[test]
    fn can_parse_remove_cmd() {
        assert_eq!(
            Command::from_iter(&["pvm", "remove", "8.1.9"]),
            Command::Remove {
                version: "8.1.9".to_owned()
            }
        );
    }

    #[test]
    fn can_parse_add_cmd() {
        assert_eq!(
            Command::from_iter(&["pvm", "add", "8.1.9"]),
            Command::Add {
                version: "8.1.9".to_owned()
            }
        );
    }

    #[test]
    fn can_parse_use_cmd() {
        assert_eq!(
            Command::from_iter(&["pvm", "use", "8.1.9"]),
            Command::Use {
                version: "8.1.9".to_owned()
            }
        );
    }
}
