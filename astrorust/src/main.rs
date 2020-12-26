use ggez::audio::SoundSource;
use ggez::graphics::{clear, present, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler, KeyMods},
};
use ggez::{Context, ContextBuilder, GameResult};

/// Define the maximum FPS for the game
const DESIRED_FPS: u32 = 60;

mod assets;
mod function;
mod game;
mod game_over;
mod main_menu;
/// The game's principal structure, import every elements necessary for the game
struct MainState {
    game_scene: game::GameScene,
    main_menu: main_menu::MainMenu,
    game_over: game_over::GameOver,
    switch_scene: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> Self {
        MainState {
            game_scene: game::GameScene::default(ctx),
            main_menu: main_menu::MainMenu::default(),
            game_over: game_over::GameOver::default(),
            switch_scene: 0,
        }
    }
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_scene.sound.bg_loop.playing() == false {
            let _ = self.game_scene.sound.bg_loop.play();
        }
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.switch_scene == 1 {
                self.game_scene.create_meteor();
            }
            self.game_scene.ship_event(ctx);
            for elem in self.game_scene.fire.iter_mut() {
                if elem.ball.y > 0.0 {
                    elem.ball.y -= game::SHOTS;
                } else if elem.ball.y <= 0.0 {
                    elem.life = false;
                }
            }
            for rock in self.game_scene.meteor.iter_mut() {
                if rock.rock.y < game::SCREEN_HEIGHT {
                    rock.rock.y += self.game_scene.speed;
                } else if rock.rock.y >= game::SCREEN_HEIGHT {
                    rock.life = false;
                }
                if rock.rock.overlaps(&self.game_scene.ship) {
                    self.game_scene.alive = false;
                    self.switch_scene = 2;
                }
                // println!("FPS: {:?}", ggez::timer::fps(ctx));
            }
            self.game_scene.collision();
            self.game_scene.clear_dead_elem();
            self.game_scene.level_up();
            self.game_scene.game_over();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        function::draw_image(&self.game_scene.images.background, ctx);

        match self.switch_scene {
            0 => self.main_menu.draw_welcome(ctx),
            1 => self.game_scene.draw_elem(ctx),
            2 => self.game_over.draw_game_over(ctx),
            _ => (),
        }

        present(ctx)?;
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Space => {
                self.game_scene
                    .new_shot(self.game_scene.ship.x, self.game_scene.ship.y);
            }
            KeyCode::P => {
                self.switch_scene = 1;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}
/// Manage the creation of the game's context and launch the game
fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("AstroRust", "Daouda, Claire")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(game::SCREEN_WIDTH, game::SCREEN_HEIGHT),
        )
        .window_setup(WindowSetup::default().title("AstrooooRuuuust"))
        .add_resource_path("./src")
        .build()?;
    let main_state = &mut MainState::new(ctx);
    event::run(ctx, event_loop, main_state)
}
