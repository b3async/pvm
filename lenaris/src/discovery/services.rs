use crate::DiscoveryService;

pub struct SysInfo;

impl DiscoveryService for SysInfo {
    type Error = crate::error::Error;

    fn get_distro_id() -> Result<String, Self::Error> {
        let release = match sys_info::linux_os_release() {
            Ok(release) => release,
            Err(err) => {
                return Err(Self::Error::Failed(format!(
                    "unable to fetch release datails: {}",
                    err
                )))
            }
        };

        release
            .id_like
            .ok_or(Self::Error::Failed("unable to discover distro id".to_string()))
    }

    fn get_vendor_id() -> Result<String, Self::Error> {
        match sys_info::os_type() {
            Ok(os_type) => Ok(os_type),
            Err(err) => Err(crate::error::Error::Failed(format!(
                "failed vendor id discovery: {}",
                err
            ))),
        }
    }
}
