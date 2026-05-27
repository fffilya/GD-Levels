use gdlevels::types::*;

fn main() -> Result<(), String>{
    // let gdfile = GDFile::decode(".result/CCLocalLevels.dat");
    // let level = gdfile.find_level("testing")?;

    let mut level = Level::empty();

    level.add(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Scale(0.5), Component::Rotation(45.0)]);

    level.copyToClipboard();
    
    Ok(())
} 
