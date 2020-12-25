#[cfg(test)]
mod tests {
    use crate::function::*;
    use crate::main_menu::*;
    use crate::game_over::*;
    use crate::game::*;

    #[test]
    // test function.rs
    fn test_erase_vec() {
        let mut vec = vec![0,1,2,3];
        erase_vec(&mut vec);
        assert_eq!(0,vec.len());
    }
    #[test]
    //test main_menu.rs
    fn test_main_menu(){
        let main = main_menu::default();
        assert_eq!("Bienvenu sur Astro Rust".to_string(),main.welcome);
        assert_eq!("pressez sur la touche P : PLAY".to_string(),main.play);
        assert_eq!("pressez sur la touche echap : QUIT".to_string(),main.quit);

    }
    
    #[test]
    // test game.rs
    fn test_game(){
        let game_scene = GameScene::default();
        assert_eq!(game_scene.ship.x,SCREEN_WIDTH / 2. - SHIP_DIM / 2.0);
        assert_eq!(game_scene.ship.y,SCREEN_HEIGHT - SHIP_DIM * 2.0);
        assert_eq!(game_scene.w,SHIP_DIM);
        assert_eq!(game_scene.h,SHIP_DIM);
        assert_eq!(game_scene.carole,0);
        game_scene.create_meteor();
        assert_eq!(game_scene.meteor.len(),3);
        game_scene.remove_all();
        assert_eq!(game_scene.meteor.len(),0);
        assert_eq!(game_scene.meteor.len(),0);

    }
}