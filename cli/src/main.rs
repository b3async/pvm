use lenaris::OperatingSystem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operating_system = OperatingSystem::discover()?;

    println!("{operating_system:?}");

    Ok(())
}
