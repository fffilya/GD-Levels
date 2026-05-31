use gdlevels::types::*;
use gdlevels::level::Level;
use gdlevels::object::Object;
use gdlevels::specialObjs::*;
use gdlevels::types::Component::*;

// now lets add our first trigger!
// triggers are being add using Object.trigger(Trigger, TriggerComponents, Option<BasicComponents>)
// we also need to use gdlevels::specialObjs for this
// lets add move trigger thats moving block with group of 1 by 1 block in 0.5 seconds

fn main() -> Result<(), Error> {
    let mut level = Level::empty();

    // adding basic object
    level.add(Object::new(vec![ID(211), X(30.0), Y(30.0), Groups(vec![1])]));

    // adding our move trigger where first argument is X, second is Y, and third is duration
    // note: 10 move trigger units = 30 game units = 1 block
    level.add(Object::trigger(Trigger::Move{ x: 10, y: 0, duration: 0.5}, vec![TriggerComponent::TargetGroupID(1)], None)?);
    
    let _ = level.copy_to_clipboard();

    Ok(())
}
