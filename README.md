# AgroFlightSim

**AgroFlightSim** est un simulateur logiciel de missions de drones
agricoles, développé progressivement en Rust.

L'objectif est de construire un moteur capable de représenter une
parcelle, de configurer un drone et une mission, puis de simuler le
déroulement du vol dans différentes conditions environnementales.

Le projet commence volontairement avec un modèle simple et déterministe.
La complexité physique et environnementale sera ajoutée par étapes, afin
de garder un logiciel compréhensible, testable et extensible.

> **Périmètre :** AgroFlightSim est un projet de simulation et
> d'expérimentation. Il ne constitue pas un outil certifié de
> préparation de vol ni un système garantissant la sécurité d'une
> opération réelle.

## Objectifs du projet

-   Apprendre Rust à travers un projet concret et évolutif.
-   Modéliser les éléments d'une mission agricole : parcelle, drone,
    trajectoire et environnement.
-   Simuler des missions de cartographie, de pulvérisation et
    d'inspection.
-   Mesurer les performances d'une mission : durée, distance, autonomie,
    consommation et couverture.
-   Étudier l'effet de conditions variables, notamment le vent, la pluie
    et la température.
-   Construire progressivement un moteur réutilisable, indépendant de
    son interface graphique.

## Feuille de route

Les phases sont conçues pour être réalisées dans l'ordre. Chaque phase
doit aboutir à un résultat fonctionnel et testable avant de passer à la
suivante.

### Phase 0 --- Fondations Rust et structure du projet

**But :** mettre en place l'environnement de développement et apprendre
les premiers éléments du langage.

Travail prévu : - Installer et vérifier Rust, `rustup` et Cargo. - Créer
le projet Cargo et organiser les modules. - Découvrir les variables, les
types, les `struct`, les `enum` et les fonctions. - Définir les
premières structures `Drone`, `Field` et `Mission`. - Compiler et
exécuter le programme avec Cargo.

**Résultat attendu :** un programme en ligne de commande qui décrit un
drone, une parcelle et une mission, et calcule la surface de la
parcelle.

### Phase 1 --- Géométrie des parcelles et planification des trajectoires

**But :** générer un parcours simple permettant de couvrir une parcelle
agricole.

Travail prévu : - Représenter des points et des parcelles
rectangulaires. - Calculer les dimensions et la surface d'une
parcelle. - Définir une largeur de couverture ou de pulvérisation. -
Générer des lignes de passage parallèles. - Ajouter les demi-tours et
les marges de sécurité géométriques. - Calculer la longueur totale de la
trajectoire.

**Résultat attendu :** à partir d'une parcelle et de paramètres de
mission, AgroFlightSim génère une trajectoire de couverture et estime la
distance à parcourir.

### Phase 2 --- Modèle du drone

**But :** représenter les caractéristiques propres au drone et ses
capacités opérationnelles.

Travail prévu : - Décrire la masse, la vitesse, la batterie et la charge
utile. - Modéliser la capacité et l'état de la batterie. - Modéliser le
réservoir et la consommation de produit. - Distinguer les
caractéristiques communes des paramètres spécifiques à chaque modèle de
drone. - Introduire des interfaces et abstractions lorsque plusieurs
modèles seront nécessaires.

**Résultat attendu :** plusieurs configurations de drones peuvent être
utilisées par le simulateur sans modifier le planificateur de mission.

### Phase 3 --- Moteur de simulation

**But :** faire évoluer le drone au cours du temps et simuler
l'exécution d'une mission.

Travail prévu : - Définir un pas de temps de simulation. - Faire évoluer
la position du drone le long de sa trajectoire. - Suivre l'état de la
batterie et de la charge utile. - Gérer les états d'une mission :
préparation, en cours, terminée ou interrompue. - Enregistrer les
événements et les résultats. - Ajouter des tests unitaires et des tests
d'intégration.

**Résultat attendu :** une mission complète peut être exécutée dans le
moteur, avec un état du drone mis à jour au fil du temps.

### Phase 4 --- Environnement et perturbations

**But :** introduire des conditions environnementales qui influencent le
déroulement de la mission.

Travail prévu : - Modéliser la vitesse et la direction du vent. -
Ajouter des rafales et des variations temporelles. - Introduire la pluie
et la température. - Définir comment ces paramètres influencent la
mission. - Ajouter des scénarios reproductibles grâce à une graine
aléatoire configurable.

**Résultat attendu :** une même mission peut être simulée dans plusieurs
scénarios météorologiques, avec des résultats comparables et
reproductibles.

### Phase 5 --- Enrichissement du modèle physique

**But :** améliorer progressivement le réalisme de la dynamique du
drone.

Travail prévu : - Ajouter les effets simplifiés de la masse et de la
charge utile. - Modéliser la résistance de l'air et l'effet du vent. -
Affiner la consommation énergétique. - Introduire progressivement la
dynamique des moteurs et les mécanismes de contrôle, par exemple un
régulateur PID. - Évaluer la pertinence d'un modèle dynamique plus
détaillé selon les besoins du projet.

**Résultat attendu :** les trajectoires et les consommations dépendent
davantage des caractéristiques physiques du drone et des conditions de
vol.

### Phase 6 --- Expérimentations et optimisation

**But :** utiliser le simulateur pour comparer des missions et mesurer
leur robustesse.

Travail prévu : - Exécuter automatiquement de nombreuses simulations. -
Faire varier les paramètres du drone, de la mission et de
l'environnement. - Réaliser des simulations de Monte-Carlo. - Estimer
les distributions de durée, de consommation et de couverture. - Mesurer
les probabilités de réussite ou d'interruption selon les hypothèses du
modèle. - Explorer l'optimisation des trajectoires et des paramètres de
mission. - Paralléliser les expérimentations lorsque cela devient utile.

**Résultat attendu :** un outil d'expérimentation permettant de comparer
des configurations et d'étudier leur sensibilité aux perturbations.

### Phase 7 --- Interface et exploitation des résultats

**But :** rendre le simulateur plus facile à utiliser et à explorer.

Travail prévu : - Charger ou dessiner une parcelle. - Configurer le
drone, la mission et la météo. - Lancer une simulation depuis une
interface. - Visualiser la parcelle et la trajectoire. - Afficher les
indicateurs et les graphiques de résultats. - Comparer plusieurs
simulations. - Envisager une API ou une interface web si le besoin
apparaît.

**Résultat attendu :** une application utilisable pour préparer des
scénarios, lancer des simulations et analyser les résultats.

## Architecture envisagée

L'architecture sera construite progressivement. Les composants suivants
constituent une cible, pas une structure à créer intégralement dès le
départ.

  Composant       Responsabilité
  --------------- -----------------------------------------------
  `geometry`      Points, polygones et opérations géométriques
  `mission`       Paramètres et définition des missions
  `drone`         Caractéristiques, batterie et charge utile
  `environment`   Vent, pluie, température et scénarios
  `simulation`    Évolution temporelle et état de la simulation
  `control`       Modèles de commande et régulation
  `analysis`      Indicateurs, expériences et comparaisons

Le moteur de simulation devra rester indépendant de l'interface
graphique. Cette séparation permettra de tester le cœur du simulateur et
de le réutiliser dans une application de bureau ou derrière une API.

## Principes de conception

-   **Progressivité :** ajouter une seule couche de complexité à la
    fois.
-   **Modularité :** séparer le drone, la mission, l'environnement et le
    moteur.
-   **Reproductibilité :** permettre de rejouer un scénario avec les
    mêmes paramètres et la même graine aléatoire.
-   **Testabilité :** couvrir les calculs et comportements importants
    par des tests.
-   **Unités explicites :** utiliser le système international (mètres,
    secondes, kilogrammes, mètres par seconde, etc.).
-   **Simplicité avant réalisme :** commencer par des modèles
    déterministes simples, puis les enrichir lorsque cela apporte une
    réelle valeur.
-   **Pas de dépendance prématurée à l'interface :** le cœur de
    simulation ne doit pas dépendre de la visualisation.

## Technologies envisagées

Les dépendances seront ajoutées uniquement lorsque le projet en aura
besoin.

  Technologie / bibliothèque   Utilisation envisagée
  ---------------------------- -----------------------------------------------
  Rust et Cargo                Langage et gestion du projet
  `serde`                      Sérialisation des configurations et résultats
  `clap`                       Interface en ligne de commande
  `nalgebra`                   Algèbre linéaire et calculs numériques
  `rand`                       Scénarios aléatoires et perturbations
  `thiserror`                  Gestion structurée des erreurs
  `rayon`                      Exécution parallèle des expérimentations
  `egui` / `eframe`            Interface graphique de bureau

## Démarrage

Prérequis : installation de Rust via [rustup](https://rustup.rs/).

Cloner le dépôt puis se placer dans le répertoire du projet :

``` bash
git clone <URL_DU_DEPOT>
cd agroflight-sim
```

Vérifier le projet :

``` bash
cargo check
```

Exécuter le programme :

``` bash
cargo run
```

Lancer les tests :

``` bash
cargo test
```

Formater le code :

``` bash
cargo fmt
```

Analyser les avertissements et les bonnes pratiques :

``` bash
cargo clippy
```

## État d'avancement

-   [x] Phase 0 --- Projet Cargo créé, modules `drone`, `field` et
    `mission` définis, première compilation réussie.
-   [ ] Phase 1 --- Géométrie et planification des trajectoires.
-   [ ] Phase 2 --- Modèle du drone.
-   [ ] Phase 3 --- Moteur de simulation.
-   [ ] Phase 4 --- Environnement et perturbations.
-   [ ] Phase 5 --- Modèle physique enrichi.
-   [ ] Phase 6 --- Expérimentations et optimisation.
-   [ ] Phase 7 --- Interface et exploitation des résultats.

------------------------------------------------------------------------

*AgroFlightSim --- un projet d'apprentissage Rust construit autour de la
simulation de missions de drones agricoles.*
