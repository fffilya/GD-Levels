use gdlevels::types::*;
use gdlevels::level::Level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = GDFile::decode(".result/CCLocalLevels.dat");
    let mut level = Level::empty();
    file.find_level("testing", &mut level)?;

    level.print();
    
    Ok(())
}
