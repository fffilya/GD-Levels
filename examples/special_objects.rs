use gdlevels::types::*;
use gdlevels::object::Object;
use gdlevels::specialObjs::SpecialObject;
use gdlevels::level::Level;
use gdlevels::component::Component;

// Now lets try "Special Objects".
// Special Objects are basic objects but they have at least one component that is unique only for this object or small group of objects.
// For this example we are gonna use Secret Coin and its "Secret Coin ID" component

fn main() -> Result<(), Error<'static>> {
    let mut level = Level::empty();

    level.add(Object::special_object(SpecialObject::Coin(1), Some(vec![Component::X(30.0), Component::Y(30.0)]))?);

    let _ = level.copy_to_clipboard();
    Ok(())
}
