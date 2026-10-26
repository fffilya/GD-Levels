#[cfg(test)]
mod tests {
    use gdlevels::{
        component::Component, object::Object, specialObjs::{
            Trigger::*,
            TriggerComponent::*
        }, types::{Error, HSV}
    };
    #[test]
    fn object_new() -> Result<(), Error<'static>> {
        let obj = Object::new(vec![]);
        assert_eq!(obj.contains(), "155,1;");
        Ok(())
    }
    #[test]
    fn object_ID() -> Result<(), Error<'static>> {
        let obj = Object::new(vec![Component::ID(211)]);
        assert_eq!(obj.contains(), "1,211,155,1;");
        Ok(())
    }
    #[test]
    fn object_components() -> Result<(), Error<'static>> {
        let obj = Object::new(vec![Component::ID(211), Component::X(30.0), Component::Y(30.0), Component::Groups(vec![1, 3, 5]), Component::MainColorHSV(HSV {hue: 100, sat: 1.0, val: 1.0, sat_checked: false, val_checked: false})]);
        assert_eq!(obj.contains(), "1,211,2,30,3,30,57,1.3.5,41,1,43,100a1.00a1.00a0a0,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_alpha() -> Result<(), Error<'static>> {
        let obj = Object::trigger(Alpha{ duration: 1.0, opacity: 1.0 }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,1007,10,1,35,1,51,1,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_move() -> Result<(), Error<'static>> {
        let obj = Object::trigger(Move{ x: 10, y: 15, duration: 1.0 }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,901,28,30,29,45,10,1,51,1,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_color() -> Result<(), Error<'static>> {
        let obj = Object::trigger(Color{ duration: 1.0, colorID: 100, red: 255, blue: 0, green: 0, opacity: 1.0, blending: false, player1_color: false, player2_color: false }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,899,10,1,23,100,7,255,8,0,9,0,35,1,17,0,15,0,16,0,51,1,155,1;");
        Ok(())
    }
    fn object_trigger_opacity_overflow() -> Result<(), Error<'static>> {
        let obj = Object::trigger(Alpha{ duration: 1.0, opacity: 1.0 }, vec![], None);
        // assert_eq!();
        Ok(())
    }
}
