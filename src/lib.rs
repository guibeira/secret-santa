use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub name: String,
    picked: Option<String>,
    pub has_picked: bool,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            picked: None,
            has_picked: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Finished,
}

#[derive(Debug)]
pub struct SecretSantaGame {
    pub status: GameStatus,
    pub players: Vec<Player>,
}

impl SecretSantaGame {
    pub fn new() -> Self {
        SecretSantaGame {
            status: GameStatus::NotStarted,
            players: vec![],
        }
    }
}

impl SecretSantaGame {
    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.status != GameStatus::NotStarted {
            return Err("Game already started or finished".into());
        }
        if let Some(p) = self.players.iter().find(|p| p.name == player.name) {
            return Err(format!("Player {} already exists", p.name));
        }
        if player.name.is_empty() {
            return Err("Player name cannot be empty".into());
        }
        self.players.push(player);
        Ok(())
    }

    pub fn remove_player(&mut self, player_name: &str) -> Result<(), String> {
        if self.status != GameStatus::NotStarted {
            return Err("Game already started or finished".into());
        }
        if let Some(index) = self.players.iter().position(|p| p.name == player_name) {
            self.players.remove(index);
            Ok(())
        } else {
            Err("Player not found".into())
        }
    }

    pub fn start_game(&mut self) -> Result<(), String> {
        if self.status == GameStatus::InProgress {
            return Err("Game already started".into());
        }

        if self.status == GameStatus::Finished {
            return Err("Game already finished".into());
        }

        if self.players.len() < 2 {
            return Err("Not enough players".into());
        }

        self.sort_players();
        self.status = GameStatus::InProgress;
        Ok(())
    }

    pub fn restart_game(&mut self) {
        self.status = GameStatus::NotStarted;
        self.players = vec![];
    }
    fn sort_players(&mut self) {
        self.suffle_players();
        if !self.players.is_empty() {
            for i in 0..self.players.len() - 1 {
                let next_index = i + 1;
                let next_name = self.players[next_index].name.clone();
                self.players[i].picked = next_name.into();
            }
        }

        let head_name = self.players[0].name.clone();
        let last_index = self.players.len() - 1;
        self.players[last_index].picked = head_name.into();
    }

    fn suffle_players(&mut self) {
        let mut rng = thread_rng();
        self.players.shuffle(&mut rng);
    }

    fn check_game_status(&mut self) {
        if self.players.iter().all(|p| p.has_picked) {
            self.status = GameStatus::Finished;
        }
    }

    pub fn player_pick(&mut self, player_name: &str) -> Result<String, String> {
        if self.status == GameStatus::NotStarted {
            return Err("Game not started".into());
        }

        if self.status == GameStatus::Finished {
            return Err("Game finished".into());
        }

        let mut picked_name = None;

        if let Some(player) = self.players.iter_mut().find(|p| p.name == player_name) {
            if player.has_picked {
                return Err("Player has already picked".into());
            }

            player.has_picked = true;
            picked_name = player.picked.clone();
        } else {
            return Err("Player not found".into());
        }

        self.check_game_status();

        if picked_name.is_none() {
            return Err("Player not found".into());
        } else {
            Ok(picked_name.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_player_in_game() {
        let player = Player::new("Player 1");
        let mut game = SecretSantaGame {
            status: GameStatus::NotStarted,
            players: vec![],
        };
        assert_eq!(game.players.len(), 0);

        let _ = game.add_player(player);
        assert_eq!(game.players.len(), 1);
        assert_eq!(game.players[0].name, "Player 1");
        assert_eq!(game.players, vec![Player::new("Player 1")],);
    }

    #[test]
    fn start_game() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        assert_eq!(game.status, GameStatus::NotStarted);
        let head = game.players[0].clone();
        assert_eq!(head.picked, None);

        let _ = game.start_game();

        assert_eq!(game.status, GameStatus::InProgress);
        let head_picked_name = game.players[0].clone().picked.unwrap();
        assert!(head_picked_name.contains("Player"));
    }

    #[test]
    fn restart_game() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        assert_eq!(game.status, GameStatus::NotStarted);
        let head = game.players[0].clone();
        assert_eq!(head.picked, None);

        let _ = game.start_game();

        assert_eq!(game.status, GameStatus::InProgress);

        game.restart_game();
        assert_eq!(game.status, GameStatus::NotStarted);
        assert_eq!(game.players.len(), 0);
    }

    #[test]
    fn pick_player() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        let _ = game.start_game();

        // start picking

        for i in 0..game.players.len() {
            let player_name = format!("Player {}", i);
            let picked_name = game.player_pick(&player_name).unwrap();
            assert!(picked_name.contains("Player"));
        }

        assert_eq!(game.status, GameStatus::Finished);
    }

    #[test]
    fn start_game_with_less_than_two_players() {
        let mut game = SecretSantaGame::new();
        let player = Player::new("Player 1");
        let _ = game.add_player(player);

        let result = game.start_game();
        assert_eq!(result, Err("Not enough players".into()));
    }

    #[test]
    fn start_game_twice() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        let _ = game.start_game();

        let result = game.start_game();
        assert_eq!(result, Err("Game already started".into()));
    }

    #[test]
    fn start_game_that_already_finished() {
        let mut game = SecretSantaGame::new();

        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        let _ = game.start_game();

        // start picking

        for i in 0..game.players.len() {
            let player_name = format!("Player {}", i);
            let _ = game.player_pick(&player_name).unwrap();
        }

        assert_eq!(game.status, GameStatus::Finished);

        let result = game.start_game();
        assert_eq!(result, Err("Game already finished".into()));
    }

    #[test]
    fn pick_player_that_not_started() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }

        let player_name = "Player 1";
        let result = game.player_pick(&player_name);
        assert_eq!(result, Err("Game not started".into()));
    }

    #[test]
    fn pick_player_that_already_finished() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        let _ = game.start_game();

        // start picking

        for i in 0..game.players.len() {
            let player_name = format!("Player {}", i);
            let _ = game.player_pick(&player_name).unwrap();
        }

        assert_eq!(game.status, GameStatus::Finished);

        let player_name = "Player 1";
        let result = game.player_pick(&player_name);
        assert_eq!(result, Err("Game finished".into()));
    }

    #[test]
    fn add_player_in_a_finished_game() {
        let mut game = SecretSantaGame::new();
        for i in 0..10 {
            let player = Player::new(&format!("Player {}", i));
            let _ = game.add_player(player);
        }
        let _ = game.start_game();

        // start picking

        for i in 0..game.players.len() {
            let player_name = format!("Player {}", i);
            let _ = game.player_pick(&player_name).unwrap();
        }

        assert_eq!(game.status, GameStatus::Finished);

        let player = Player::new("Player 19");
        let result = game.add_player(player);
        assert_eq!(result, Err("Game already started or finished".into()));
    }

    #[test]
    fn add_player_with_same_name_twice() {
        let mut game = SecretSantaGame::new();
        let player = Player::new("Player 1");
        game.add_player(player.clone()).unwrap();
        let second_add = game.add_player(player);
        assert_eq!(second_add, Err("Player Player 1 already exists".into()));
    }

    #[test]
    fn add_player_with_empty_name() {
        let mut game = SecretSantaGame::new();
        let player = Player::new("");
        let result = game.add_player(player);
        assert_eq!(result, Err("Player name cannot be empty".into()));
    }

    #[test]
    fn remove_player_in_game() {
        let mut game = SecretSantaGame::new();
        let player = Player::new("Player 1");
        game.add_player(player.clone()).unwrap();
        assert_eq!(game.players.len(), 1);

        let _ = game.remove_player(&player.name);
        assert_eq!(game.players.len(), 0);
    }
}
