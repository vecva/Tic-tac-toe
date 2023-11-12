// Copyright 2023 Kamil Gloc

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//  http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
  game::{Game, GameError, Outcome, Side},
  player::{Player, PlayerError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub(super) enum ControllerError {
  #[error(transparent)]
  Player(#[from] PlayerError),
  #[error(transparent)]
  Game(#[from] GameError),
}

pub(super) type GameCount = u16;

pub(super) struct Controller {
  player_x: Player,
  player_o: Player,
  game_count: GameCount,
  x_win: GameCount,
  o_win: GameCount,
  draw: GameCount,
}

impl Controller {
  pub(super) const MINIMUM_GAMES_COUNT: GameCount = 1;

  pub(super) fn new(player_x: Player, player_o: Player, game_count: GameCount) -> Self {
    Self {
      player_x,
      player_o,
      game_count,
      x_win: 0,
      o_win: 0,
      draw: 0,
    }
  }

  pub(super) fn engage(mut self) -> Result<(), ControllerError> {
    self.introduce_players();
    self.play()?;
    self.print_results();
    Ok(())
  }

  fn play_one_game(&mut self) -> Result<(), ControllerError> {
    println!("game start\n");
    let mut game = Game::new();
    game.print_grid();

    loop {
      game.place_mark(&if *game.get_side_to_move() == Side::X {
        self.player_x.get_move(&game)?
      } else {
        self.player_o.get_move(&game)?
      })?;
      game.print_grid();

      if let Some(outcome) = game.get_outcome() {
        match outcome {
          Outcome::Draw => {
            println!("draw");
            self.draw += 1;
          }
          Outcome::XWin => {
            println!("x win");
            self.x_win += 1;
          }
          Outcome::OWin => {
            println!("o win");
            self.o_win += 1;
          }
        };

        return Ok(());
      }
    }
  }

  fn play(&mut self) -> Result<(), ControllerError> {
    for _ in 0..self.game_count {
      self.play_one_game()?;
    }

    Ok(())
  }

  fn introduce_players(&self) {
    println!(
      "\nplayer x: {}\nplayer o: {}\n",
      self.player_x, self.player_o
    )
  }

  fn print_results(self) {
    println!(
      "\nResults:\nx win: {}\no win: {}\ndraw:  {}\n",
      self.x_win, self.o_win, self.draw
    );
  }
}
