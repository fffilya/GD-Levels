pub enum SpecialObject {
    Coin(u8),
    CollisionBlock{
        id: u16,
        is_dynamic: bool
    }
}

pub enum Trigger {
    Move {
        x: i32, 
        y: i32,
        duration: f32,
        // easing: Easing
    },
    Alpha {
        duration: f32,
        opacity: f32
    },
    Color {
        duration: f32,
        colorID: u16,
        red: u8,
        green: u8,
        blue: u8,
        opacity: f32,
        blending: bool,
        player1_color: bool,
        player2_color: bool
    },
}

pub enum TriggerComponent {
    TargetGroupID(u16),
}

pub enum Easing {
    NoOne,
    EaseIn(f32),
    EaseOut(f32),
    EaseInOut(f32),
    ElasticIn(f32),
    ElasticOut(f32),
    ElasticInOut(f32),
    BounceIn,
    BounceOut,
    BounceInOut,
    ExpIn,
    ExpOut,
    ExpInOut,
    SineIn,
    SineOut,
    SineInOut,
    BackIn,
    BackOut,
    BackInOut
}
