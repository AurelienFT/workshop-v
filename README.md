# workshop-v

Clone Vlang et build :
```
https://github.com/vlang/v.git
cd v
make
# HINT: Using Windows?: run make.bat in the cmd.exe shell
```

Ajouter au path :
UNIX/MAC :
```
sudo ./v symlink
```

WINDOWS :
```
.\v.exe symlink
```

Dans ce dossier il y a un dossier `tetris/` pour le compiler `cd tetris && v tetris.v`. Vous aurez des erreurs lors du lancement car j'ai méticuleusement supprimé des parties du code sur les conditions de suppression de ligne et l'initialisation du plateau. 

Cependant vous trouverez ici des fichiers avec les codes dans vos languages de prédilection.

Première modification dans le `tetris.v` à la ligne 294 vous devez rajouter un check pour voir si la ligne est full. Vous trouverez les equivalents dans vos languages dans le dossier `check/`.

Deuxième et dernière modification dans le `tetris.v` à la ligne 189 vous devez initialiser le tableau avec des 0 au milieu et des -1 sur les bords. Vous trouverez les equivalents dans vos languages dans le dossier `init/`.

Vous avez la solution et donc un tetris qui marche si vous voulez essayer dans le dossier `solution/`.