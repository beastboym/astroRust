# AstroRust

Un vaisseaux spacial qui fait pew. Avec [Ggez](https://docs.rs/ggez/0.5.1/ggez/).

Un jeu de type Space shooter réalisé en Rust !

## Lancer le programme
Pour lancer le jeu il suffit de faire:
```cargo build```

Puis :

```cargo run --release```

à noter que le ```--release``` est important, il vous permettra de jouer dans les meilleurs conditions possibles (du moins dans les conditions que nous avons juger optimal).

Si vous tombez sur l'erreur suivante :
```platform::platform::x11::util::input::PointerState` uninitialized```
vous pouvez le regler en suivant ces [consignes](https://github.com/ggez/ggez/issues/843#issuecomment-736985569)

Nous avons volontairement bloqué le jeu a 60fps.
## Plan : 
- ~~Faire une fenêtre vide~~
- ~~Ajouter un carré et réussir à le faire bouger dans les limites de la fenêtre~~
- ~~Ajouter des obstacles qui tombent du haut de la fenêtre depuis des positions aléatoires~~
- ~~Ajout des collisions et partie perdue si collision~~
- ~~Ajout de tir pour le vaisseau~~
- ~~Destruction d'un obstacle si le tir fait but~~
- ~~Ajout d'une barre de vie (3 collisions pour perdre la partie)~~
- ~~Système de point~~
- ~~Graphisme (apparence du vaisseau, des obstacles, des tirs, background)~~

##### Bonus si on a le temps et la motivation :
- ~~Faire un menu principal~~
- ~~Faire un menu game over~~
- ~~Gerer les FPS~~
- Une base de données pour sauvegarder les anciens scores
- ~~Musique, bruitage~~
- Animation d'explosion pour les collisions
- ~~Plusieurs niveaux donc augmentation de la difficultés (différentes vitesses et un maximum d'obstacles dans la fenêtre)~~
- Mode entraînement où on peut choisir le niveau
  
## Resources

- Son et bruitages récuperer en totalité sur:  [freesound](https://freesound.org/).
- Le background à été recuperer sur : [unsplash](https://unsplash.com/backgrounds/nature/space) et photographier par [bryan_goff](https://unsplash.com/@bryangoffphoto).
- Les images du vaisseaux, des astéroides et des tire ont ete recuperer sur : [hiclipart](https://www.hiclipart.com/).
