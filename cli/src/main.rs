use lenaris::{Vendor, SysInfo};
use std::fs::create_dir_all;

#[macro_export]
macro_rules! pvm_path {
    ($( $path:expr ),+) => {{
        let home_dir = std::env::var("HOME").expect("unable to find HOME environment variable");
        let mut path: std::path::PathBuf = [home_dir.as_str()].iter().collect();

        path.push(".pvm");

        $(
            path.push($path);
        )+

        &path.clone()
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

fn initialize_pvm_folders() -> std::io::Result<()> {
    if !pvm_build_path!().exists() {
        create_dir_all(pvm_build_path!())?;
    }

    if !pvm_versions_path!().exists() {
        create_dir_all(pvm_versions_path!())?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _operating_system = Vendor::discover::<SysInfo>()?;

    initialize_pvm_folders()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::pvm_path;

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
}
