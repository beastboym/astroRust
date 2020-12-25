use ggez::audio;
use ggez::Context;
pub(crate) struct Sound {
    pub shot_sound: audio::Source,
    pub collision: audio::Source,
    pub game_over: audio::Source,
    pub bg_loop: audio::Source,
}

impl Sound {
    pub(crate) fn default(ctx: &mut Context) -> Self {
        Sound {
            shot_sound: audio::Source::new(ctx, "/ressources/pew.wav").unwrap(),
            collision: audio::Source::new(ctx, "/ressources/collision.wav").unwrap(),
            game_over: audio::Source::new(ctx, "/ressources/game_over.wav").unwrap(),
            bg_loop: audio::Source::new(ctx, "/ressources/bg_loop.mp3").unwrap(),
        }
    }
}

pub(crate) struct Images {
    pub background: String,
    pub meteor: String,
    pub shot: String,
    pub ship: String,
}

impl Images {
    pub fn default() -> Self {
        Images {
            background: "/ressources/bryan-goff1.jpg".to_string(),
            meteor: "/ressources/meteor.png".to_string(),
            shot: "/ressources/laser_shot1.png".to_string(),
            ship: "/ressources/ship.png".to_string(),
        }
    }
}