use gdlevels::{
    level::Level,
    component::Component
};

// now lets iterate through the level and do something with every object
// i want to find every object with id of 211 and add them group 2

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut level = Level::empty();
    let _ = level.decode_from_gdlvl(".result/testing.gdlvl");
    level.print();
    
    _ = level.iter_mut()
        .try_for_each(|object| -> Result<(), Box<dyn std::error::Error>> {
            println!("{:?}", object);
            if object.have_component(Component::ID(211))? {
                object.add_component(Component::Groups(vec![2]));
        }
        Ok(())
        });

    _ = level.encode_to_gdlvl(".result/testing_new.gdlvl", ".result/testing.gdlvl");
    Ok(())
}
