#[cfg(feature = "darwin")]
#[test]
fn test_vendor_discovery() {
    use lenaris::{SysInfo, Vendor, VendorID};

    let vendor = Vendor::discover::<SysInfo>();

    assert!(vendor.is_ok());

    let Vendor(vendor_id, distro_id) = vendor.unwrap();

    assert!(distro_id.is_none());
    assert_eq!(vendor_id, VendorID::MacOS);
}
