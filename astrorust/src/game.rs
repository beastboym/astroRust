use ggez::audio::SoundSource;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use rand::Rng;

use crate::assets::*;
use crate::function;

pub const SCREEN_WIDTH: f32 = 600.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

/// dimensions du container du vaisseau, ne modifie pas directement le vaisseau mais plutot le rect dans lequel il est
pub const SHIP_DIM: f32 = 25.0;
/// vitesse de deplacement du vaisseau
pub const SPEED: f32 = 8.0;
/// largeur du container du tir, ne modifie pas directement le tir mais plutot le rect dans lequel il est
pub const SHOT_DIMX: f32 = 20.0;
/// longueur du container du tir, ne modifie pas directement le tir mais plutot le rect dans lequel il est
pub const SHOT_DIMY: f32 = 40.0;
/// dimensions du container des meteorites, ne modifie pas directement le meteorites mais plutot le rect dans lequel il est
pub const METE_DIM: f32 = 50.0;
/// vitesse des tirs
pub const SHOTS: f32 = 3.0;

/// Concretement, exactement la meme structure que Meteor, cet separation a été faites pour une question de lisibilié
/// La seule diffrence notable est que FireShot n'implemente pas default
pub struct FireShot {
    pub ball: Rect,
    pub life: bool,
}
/// Concretement, exactement la meme structure que FireShot, cet separation a été faites pour une question de lisibilié
pub struct Meteor {
    pub rock: Rect,
    pub life: bool,
}

/// Scene principal du jeu, contient toutes les entité utiles au fonctionnement du jeu
pub(crate) struct GameScene {
    pub ship: Rect,
    pub fire: Vec<FireShot>,
    pub meteor: Vec<Meteor>,
    pub nb_rocks: u32,
    pub score: u32,
    pub level: u32,
    pub alive: bool,
    pub carole: u32,
    pub speed: f32,
    pub sound: Sound,
    pub images: Images,
}

impl Meteor {
    /// defini une valeur par defaut a notre structure Meteor
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rando: f32 = rng.gen_range(0.0 + METE_DIM, SCREEN_WIDTH - METE_DIM);
        Meteor {
            rock: Rect::new(rando, -SHIP_DIM, METE_DIM, METE_DIM),
            life: true,
        }
    }
}

impl GameScene {
    /// defini une valeur par defaut a notre structure GameScene
    pub fn default(ctx: &mut Context) -> Self {
        GameScene {
            ship: Rect::new(
                SCREEN_WIDTH / 2. - SHIP_DIM / 2.0,
                SCREEN_HEIGHT - SHIP_DIM * 2.0,
                SHIP_DIM,
                SHIP_DIM,
            ),
            fire: Vec::new(),
            meteor: Vec::new(),
            nb_rocks: 3,
            score: 0,
            level: 1,
            alive: true,
            carole: 0,
            speed: SHOTS,
            sound: Sound::default(ctx),
            images: Images::default(),
        }
    }

    /// creer un nouveaux tirs
    pub fn new_shot(&mut self, x: f32, y: f32) {
        let pew = FireShot {
            ball: Rect::new(x + SHIP_DIM / 2.0, y, SHOT_DIMX, SHOT_DIMY),
            life: true,
        };
        self.fire.push(pew);
        let _ = self.sound.shot_sound.play();
    }

    /// efface les entite qui sortes du cadres
    pub fn clear_dead_elem(&mut self) {
        self.fire.retain(|s| s.life == true);
        self.meteor.retain(|s| s.life == true);
    }
    /// traite les collision des rocher avec les tirs
    pub fn collision(&mut self) {
        for shot in self.fire.iter_mut() {
            for rock in self.meteor.iter_mut() {
                if shot.ball.overlaps(&rock.rock) {
                    shot.life = false;
                    rock.life = false;
                    self.score += 1;
                    self.carole += 1;
                    let _ = self.sound.collision.play();
                }
            }
        }
    }
    // defini les conditions de defaites
    pub fn game_over(&mut self) {
        if self.alive == false {
            let _ = self.sound.game_over.play();
            // event::quit(ctx);
            self.alive = true;
            self.remove_all();
            self.nb_rocks = 3;
            self.level = 1;
            self.score = 0;
            self.carole = 0;
            self.speed = SHOTS;
        }
    }

    /// defini les touches qui feront bougé le vaisseaux
    pub fn ship_event(&mut self, ctx: &Context) {
        if is_key_pressed(ctx, KeyCode::Right) {
            if self.ship.right() < SCREEN_WIDTH {
                self.ship.x += SPEED;
            }
        }
        if is_key_pressed(ctx, KeyCode::Left) {
            if self.ship.left() > 0.0 {
                self.ship.x -= SPEED;
            }
        }
    }

    /// defini les conditions de creations de meteorites
    pub fn create_meteor(&mut self) {
        if self.meteor.len() < self.nb_rocks as usize {
            let met = Meteor::default();
            self.meteor.push(met);
        }
    }

    /// dessine les elements de la scene
    pub fn draw_elem(&mut self, ctx: &mut Context) {
        function::draw_e(self.ship, ctx, &self.images.ship);
        for elem in self.fire.iter_mut() {
            function::draw_e(elem.ball, ctx, &self.images.shot);
        }
        for elem in self.meteor.iter_mut() {
            function::draw_e(elem.rock, ctx, &self.images.meteor);
        }
        let score = graphics::Text::new(format!("Score : {}", self.score));
        let level = graphics::Text::new(format!("Level : {}", self.level));
        let coord = [0.0 + score.width(ctx) as f32, 20.0];
        let lvl_coord = [0.0 + score.width(ctx) as f32, 40.0];
        function::draw_text(ctx, score, coord[0], coord[1]);
        function::draw_text(ctx, level, lvl_coord[0], lvl_coord[1]);
    }

    /// defini les conditions de passage de niveaux
    pub fn level_up(&mut self) {
        if self.carole == self.nb_rocks {
            self.nb_rocks += 1;
            self.level += 1;
            self.speed += 0.5;
            self.carole = 0;
            // self.remove_all();
        }
    }

    /// Efface toutes les entite du jeu
    pub fn remove_all(&mut self) {
        function::erase_vec(&mut self.fire);
        function::erase_vec(&mut self.meteor);
    }
}
