# Projet Connect 4
Développé par des étudiants de l'ESEO Angers dans le cadre d'une spécialité de dernière année.

## Description du jeu

Connect 4, ou Puissance 4 en français, est un jeu à dont le but est d'aligner 4 pièces.  
C'est un jeu à deux joueurs, tour par tour, sur une grille.  
Pour gagner, il faut placer 4 de ses pièces verticalement, horizontalement ou en diagonale.  
Pour cette version du jeu, le temps des joueurs est chronométré. Un joueur qui épuise son temps est déclaré perdant.  
La grille est affiché dans le terminal et le temps des joueurs dans une fenêtre à part. 

## Déroulement du jeu 

### Lancement 
Dans un terminal, exécuter la commande dans le répertoire du jeu :  
cargo run

Le jeu demandera le nom du joueur 1 à saisir et à valider en appuyant sur "Entrée".  
Puis, le nom du joueur 2 à saisir et à valider en appuyant sur "Entrée".

Note : La fenêtre du timer sera bloquée en attente de la saisie des noms des joueurs.  

### Tour par tour

Chaque joueur place une pièce dans une colonne l'un après l'autre. Les pièces tombent dans la case libre la plus basse
de la colonne choisie.  
Un joueur est déclaré vainqueur quand :  
- Il aligne 4 pièces verticalement, horizontalement, ou en diagonale. 
- L'autre joueur n'a plus de temps restant.

Autrement, la partie se termine quand toutes les colonnes sont remplies.  

## Architecture logicielle

### Objets actifs

* main - timer_manager  
Lancement de l'application et affichage de la fenêtre du timer.

* game_manager  
Responsable de la récupération des choix de colonnes des joueurs, de l'affichage de la grille et des conditions de 
jeu de placement des pièces (victoire ou nul).

* timer_tick  
Compteur de secondes pour le temps écoulé des joueurs.

### Objets supports

* connect_4_errors  
Type d'erreur créé pour le jeu. Utilisée pour la levée d'erreur en encapsulant l'erreur probable.  

## Elements demandés

1. [x] Gestion d'erreurs  
Les erreurs sont gérés localement, en levant l'erreur à la fonction parente de manière recursive.  
Les warnings sont traités et évités au maximum.
2. [x] Trait  
Un trait est utilisé pour les objets de gestion (game_manager et timer_manager).
3. [x] Gestion de concurrence  
Objets actifs détaillés dans l'architecture. 
On utilise des canaux de communication unidirectionnels (tx et rx), sans utilisation de clone. 
4. [x] Modularité/Réutilisabilité  
L'architecture du projet est décomposé en fonction des fonctionnalités du système.  
Le code est commenté au niveau des fonctions et fonctionnalités importantes ou complexes.

## Dépendances 

* macroquad  
Utilisé pour l'affichage de la fenêtre du timer.

* clearscreen  
Utilisé pour rafraichir le terminal. 

