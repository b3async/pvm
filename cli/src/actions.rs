use crate::Context;

pub fn list_installations(_ctx: &Context) {
    let _vendor = _ctx.vendor();

    println!("List all PHP versions currently installed");
}

pub fn inspect_installation(version: String, _ctx: &Context) {
    println!("Inspect a PHP {version} installation");
}

pub fn remove_installation(version: String, _ctx: &Context) {
    println!("Remove a PHP {version} installation");
}

pub fn add_installation(version: String, _ctx: &Context) {
    println!("Install PHP {version}");
}

pub fn activate_installation(version: String, _ctx: &Context) {
    println!("Activate PHP {version}");
}
