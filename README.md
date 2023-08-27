# Générateur de mot de passe
L'objectif du projet est de faire une commande exécutable avec divers paramètres et qui génère un mot de passe aléatoire à la fin, le tout en faire en Rust pour apprendre le langage et pourquoi pas ensuite évoluer sur un autre projet de Gestionnaire de mots de passes plus grand.

Le nom de la commande est genpass.

# Configuration
La commande genpass prend plusieurs paramètres non obligatoires en compte :
- **--size** : taille du mot de passe, valeurs possibles : nombre entier, 8 par défaut
- **--without-numbers** : avec des chiffres ou sans, valeurs possibles : true | false, false par défaut
- **--without-symbols** : avec des caractères spéciaux ou sans, valeurs possibles : true | false, false par défaut
- **--without-letters** : avec des lettres ou sans, valeurs possibles : true | false, true par défaut
- **--without-uppercase** : avec des lettres majuscules ou sans, valeurs possibles : true | false, false par défaut
- **--without-lowercase** : avec des lettres minuscules ou sans, valeurs possibles : true | false, false par défaut
- **--strong** : force le mot de passe à être considéré comme fort : 8 caractères minimum, au moins une majuscule, au moins une minuscule, au moins un symbole et au moins un chiffre

Par défaut, lorsque ces paramètres sont sélectionnés sans écriture explicite de "true" ou "false", la valeur true est sélectionnée (sauf pour size où une erreur est renvoyée).

# Exemples
Avec la configuration par défaut :
```bash
genpass
```

En ajoutant des paramètres :
```bash
genpass --size 24 --without-symbols
```
