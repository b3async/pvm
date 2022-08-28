use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Distro {
    Arch,
    Debian,
    Rhel,
}

impl TryFrom<Option<String>> for Distro {
    type Error = Error;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            let distro = value.to_ascii_lowercase();
            let distro = distro.as_str();

            match distro {
                "arch" => return Ok(Distro::Arch),
                "debian" => return Ok(Distro::Debian),
                distro => return Err(Error::UnsupportedDistro(distro.to_owned())),
            }
        }

        Err(Error::Failed("unable to discover distro".to_owned()))
    }
}

#[derive(Debug)]
pub enum OperatingSystem {
    Linux(Distro),
    Darwin,
}

impl TryFrom<Result<String, sys_info::Error>> for OperatingSystem {
    type Error = Error;

    fn try_from(value: Result<String, sys_info::Error>) -> Result<Self, Self::Error> {
        let os_type = match value {
            Ok(os_type) => os_type,
            Err(err) => {
                return Err(Error::Failed(format!(
                    "unable to discover os type: {}",
                    err
                )))
            }
        };

        let os_type = os_type.to_lowercase();
        let os_type = os_type.as_str();

        match os_type {
            "linux" => {
                let os_release = match sys_info::linux_os_release() {
                    Ok(release) => release,
                    Err(err) => {
                        return Err(Error::Failed(format!(
                            "unable to fetch linux release: {}",
                            err
                        )))
                    }
                };

                let distro = Distro::try_from(os_release.id_like)?;

                Ok(Self::Linux(distro))
            }
            "darwin" => Ok(Self::Darwin),
            os_type => Err(Error::UnsupportedSystem(os_type.to_owned())),
        }
    }
}

impl OperatingSystem {
    pub fn discover() -> Result<Self, Error> {
        OperatingSystem::try_from(sys_info::os_type())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_discover_os() {
        fn mock_os(os: &str, err: Option<sys_info::Error>) -> Result<String, sys_info::Error> {
            if let Some(err) = err {
                return Err(err);
            }

            Ok(os.to_owned())
        }

        #[cfg(target_os = "linux")]
        {
            let linux_os = OperatingSystem::try_from(mock_os("Linux", None));
            assert!(linux_os.map_err(|err| eprintln!("{err}")).is_ok());
        }

        #[cfg(target_os = "macos")]
        {
            let mac_os = OperatingSystem::try_from(mock_os("Darwin", None));
            assert!(mac_os.map_err(|err| eprintln!("{err}")).is_ok());
        }

        let windows_os = OperatingSystem::try_from(mock_os("Windows", None));
        assert!(windows_os.is_err());

        let unknown_os =
            OperatingSystem::try_from(mock_os("", Some(sys_info::Error::UnsupportedSystem)));

        assert!(unknown_os.is_err());
    }

    #[test]
    fn can_discover_distro() {
        let arch = Distro::try_from(Some("arch".to_owned())).unwrap();
        let debian = Distro::try_from(Some("debian".to_owned())).unwrap();
        let rhel = Distro::try_from(Some("rhel fedora".to_owned()));

        assert_eq!(Distro::Arch, arch);
        assert_eq!(Distro::Debian, debian);

        assert!(rhel.is_err());
    }
}
