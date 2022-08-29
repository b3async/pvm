mod services;
pub use services::SysInfo;

#[derive(Debug, PartialEq, Eq)]
pub enum DistroID {
    Arch,
    Debian,
    Rhel,
}

impl TryFrom<String> for DistroID {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "debian" => Ok(Self::Debian),
            "arch" => Ok(Self::Arch),
            "rhel" | "rhel fedora" => Ok(Self::Rhel),
            distro_id => Err(crate::error::Error::UnsupportedDistro(distro_id.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VendorID {
    Linux,
    MacOS,
}

impl TryFrom<String> for VendorID {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "linux" => Ok(Self::Linux),
            "darwin" => Ok(Self::MacOS),
            vendor_id => Err(crate::error::Error::UnsupportedSystem(vendor_id.to_owned())),
        }
    }
}

pub trait DiscoveryService {
    type Error;

    fn get_distro_id() -> Result<String, Self::Error>;
    fn get_vendor_id() -> Result<String, Self::Error>;
}

pub struct Vendor(pub VendorID, pub Option<DistroID>);

impl Vendor {
    pub fn discover<DS>() -> Result<Self, crate::error::Error>
    where
        DS: DiscoveryService<Error = crate::error::Error>,
    {
        let vendor_id = <DS>::get_vendor_id()?;
        let vendor_id = VendorID::try_from(vendor_id)?;

        match vendor_id {
            VendorID::Linux => {
                let distro = <DS>::get_distro_id()?;
                let distro = DistroID::try_from(distro)?;

                Ok(Self {
                    0: vendor_id,
                    1: Some(distro),
                })
            }
            VendorID::MacOS => Ok(Self {
                0: vendor_id,
                1: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;

    #[test]
    fn can_convert_string_to_vendor_id() {
        let darwin = "Darwin".to_owned();
        let linux = "Linux".to_owned();

        assert_eq!(VendorID::try_from(darwin).unwrap(), VendorID::MacOS);
        assert_eq!(VendorID::try_from(linux).unwrap(), VendorID::Linux);

        let windows = VendorID::try_from("Windows".to_owned());

        assert!(windows.is_err());
        assert_eq!(
            windows.err().unwrap(),
            Error::UnsupportedSystem("windows".to_owned())
        );
    }

    #[test]
    fn can_convert_string_to_distro_id() {
        let debian = DistroID::try_from("Debian".to_owned());
        let arch = DistroID::try_from("Arch".to_owned());
        let fedora = DistroID::try_from("Rhel fedora".to_owned());
        let rhel = DistroID::try_from("Rhel".to_owned());

        assert_eq!(debian.unwrap(), DistroID::Debian);
        assert_eq!(arch.unwrap(), DistroID::Arch);
        assert_eq!(fedora.unwrap(), DistroID::Rhel);
        assert_eq!(rhel.unwrap(), DistroID::Rhel);

        let solaris = DistroID::try_from("Solaris".to_owned());

        assert!(solaris.is_err());
        assert_eq!(
            solaris.err().unwrap(),
            Error::UnsupportedDistro("solaris".to_owned())
        );
    }
}
