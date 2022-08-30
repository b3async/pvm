#[cfg(feature = "arch")]
#[test]
fn test_vendor_discovery() {
    use lenaris::{DistroID, SysInfo, Vendor, VendorID};

    let vendor = Vendor::discover::<SysInfo>();

    assert!(vendor.is_ok());

    let Vendor(vendor_id, distro_id) = vendor.unwrap();

    assert!(distro_id.is_some());
    assert_eq!(distro_id.unwrap(), DistroID::Arch);
    assert_eq!(vendor_id, VendorID::Linux);
}
