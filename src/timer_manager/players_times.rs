
/**
Minutes par défault des joueurs.
*/
const START_TIME_MINUTES: f64 = 50.;
/**
Secondes par défault des joueurs.
 */
const START_TIME_SECONDS: f64 = 0.;

/**
Enumération qui contient les deux joueurs possibles
*/
#[derive(PartialEq)]
pub enum Player {
    Player1,
    Player2
}

/**
Structure qui définie le temps d'un joueur avec ; son nom, ses minutes courantes, ses secondes
courantes.
*/
pub struct PlayerTimes {
    pub name_player: String,
    pub minutes : f64,
    pub seconds : f64
}

/**
Implémentation de la structure de PlayerTimes
*/
impl PlayerTimes {
    /**
    Création d'une instance de PlayerTimes prenant le nom du joueur en paramètre.
    Les minutes et secondes par défault sont attribuées par les constantes définies
    START_TIME_MINUTES et START_TIME_SECONDS.
    */
    fn new(name_player: String) -> PlayerTimes {
        PlayerTimes{
            name_player,
            minutes: START_TIME_MINUTES,
            seconds: START_TIME_SECONDS,
        }
    }
}


/**
Structure qui regroupe une énumération désignant le joueur courant, et les temps des deux joueurs.
*/
pub struct PlayersTimes {
    pub current_player: Player,
    pub timer_player_1: PlayerTimes,
    pub timer_player_2: PlayerTimes
}

/**
Implémentation des fonctions de PlayersTimes
 */
impl PlayersTimes {
    /**
    Création d'une instance de PlayerTimes comprenant le joueur courant et les noms des joueurs.
    Prend en paramètre les noms des joueurs.
    */
    pub fn new(name_player_1: String, name_player_2: String) -> PlayersTimes {
        PlayersTimes {
            current_player: Player::Player1,
            timer_player_1: PlayerTimes::new(name_player_1),
            timer_player_2: PlayerTimes::new(name_player_2),
        }
    }

    /**
    Décompte le temps du joueur courant (en secondes et minutes).
    Retourne vrai si le temps est écoulé. Sinon, retourne faux.
    */
    pub fn tick_time(&mut self) -> bool {

        // On récupère les secondes du joueur courant
        let mut seconds_current_player = &mut self.timer_player_1.seconds;
        if self.current_player == Player::Player2 {
            seconds_current_player = &mut self.timer_player_2.seconds;
        }

        // On décompte le temps
        *seconds_current_player -= 1.;
        // Si les secondes sont à zero
        if *seconds_current_player <= 0. {
            // On récupère les minutes pour les décompter de 1
            let mut minutes_current_player = &mut self.timer_player_1.minutes;
            if self.current_player == Player::Player2 {
                minutes_current_player = &mut self.timer_player_2.minutes;
            }
            // Si le temps n'est pas fini
            if *minutes_current_player > 0. {
                // On décrémente minutes et on remet les secondes à 59
                *minutes_current_player -= 1.;
                *seconds_current_player = 59.;
            }
            // Sinon, temps écoulé, on retourne vrai
            else {
                return true;
            }
        }
        // Le temps n'est pas fini
        false
    }

    /**
    Renvoie l'identifiant du joueur courant.
    */
    pub fn id_current_player(&self) -> i8 {
        if self.current_player == Player::Player1 { 1 } else { 2 }
    }

    /**
    Change de joueur courant.
    */
    pub fn change_player(&mut self) {
        if self.current_player == Player::Player1 {
            self.current_player = Player::Player2;
        } else {
            self.current_player = Player::Player1;
        }
    }

    /**
    Renvoie le nom du joueur courant
    */
    pub fn get_current_player_name(&self) -> &String {
        match self.current_player {
            Player::Player1 => &self.timer_player_2.name_player,
            Player::Player2 => &self.timer_player_1.name_player
        }
    }
}
