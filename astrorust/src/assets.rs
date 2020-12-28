use ggez::audio;
use ggez::Context;
/// Used to contain every sound effect for the game
pub(crate) struct Sound {
    pub shot_sound: audio::Source,
    pub collision: audio::Source,
    pub game_over: audio::Source,
    pub bg_loop: audio::Source,
}
    
impl Sound {
    /// Give the default values for the sounds effects used in the game
    pub(crate) fn default(ctx: &mut Context) -> Self {
        Sound {
            shot_sound: audio::Source::new(ctx, "/ressources/pew.wav").unwrap(),
            collision: audio::Source::new(ctx, "/ressources/collision.wav").unwrap(),
            game_over: audio::Source::new(ctx, "/ressources/game_over.wav").unwrap(),
            bg_loop: audio::Source::new(ctx, "/ressources/bg_loop.mp3").unwrap(),
        }
    }
}

/// Used to contain every images for the game
pub(crate) struct Images {
    pub background: String,
    pub meteor: String,
    pub shot: String,
    pub ship: String,
}

impl Images {
    /// Give the default values for the images used in the game
    pub fn default() -> Self {
        Images {
            background: "/ressources/bryan-goff1.jpg".to_string(),
            meteor: "/ressources/meteor.png".to_string(),
            shot: "/ressources/laser_shot1.png".to_string(),
            ship: "/ressources/ship.png".to_string(),
        }
    }
}
