#[cfg(test)]
mod tests {
    use gdlevels::{
        types::Component::*,
        types::Error,
        specialObjs::{
            Trigger::*,
            TriggerComponent::*
        },
        object::Object
    };
    #[test]
    fn object_new() -> Result<(), Error> {
        let obj = Object::new(vec![]);
        assert_eq!(obj.contains(), "155,1;");
        Ok(())
    }
    #[test]
    fn object_ID() -> Result<(), Error> {
        let obj = Object::new(vec![ID(211)]);
        assert_eq!(obj.contains(), "1,211,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_alpha() -> Result<(), Error> {
        let obj = Object::trigger(Alpha{ duration: 1.0, opacity: 1.0 }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,1007,10,1,35,1,51,1,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_move() -> Result<(), Error> {
        let obj = Object::trigger(Move{ x: 10, y: 15, duration: 1.0 }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,901,28,30,29,45,10,1,51,1,155,1;");
        Ok(())
    }
    #[test]
    fn object_trigger_color() -> Result<(), Error> {
        let obj = Object::trigger(Color{ duration: 1.0, colorID: 100, red: 255, blue: 0, green: 0, opacity: 1.0, blending: false, player1_color: false, player2_color: false }, vec![TargetGroupID(1)], None)?;
        assert_eq!(obj.contains(), "1,899,10,1,23,100,7,255,8,0,9,0,35,1,17,0,15,0,16,0,51,1,155,1;");
        Ok(())
    }
    fn object_trigger_opacity_overflow() -> Result<(), Error> {
        let obj = Object::trigger(Alpha{ duration: 1.0, opacity: 1.0 }, vec![], None);
        // assert_eq!();
        Ok(())
    }
}
