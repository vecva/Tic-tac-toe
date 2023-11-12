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

mod mcts;
mod minimax;
mod random;
mod user;
use super::game::{Game, Square};
use clap::ValueEnum;
use thiserror::Error;
use {
  mcts::Mcts, mcts::MctsError, minimax::MinimaxError, random::Random, random::RandomError,
  user::User, user::UserError,
};

#[derive(Error, Debug)]
pub(super) enum PlayerError {
  #[error(transparent)]
  Mcts(#[from] MctsError),
  #[error(transparent)]
  Minimax(#[from] MinimaxError),
  #[error(transparent)]
  Random(#[from] RandomError),
  #[error(transparent)]
  User(#[from] UserError),
}

#[derive(ValueEnum, Clone)]
pub(super) enum PlayerType {
  Mcts,
  Minimax,
  Random,
  User,
}

impl PlayerType {
  pub(super) const PLAYER_O_DEFAULT: PlayerType = PlayerType::Mcts;
  pub(super) const PLAYER_X_DEFAULT: PlayerType = PlayerType::User;

  pub(super) fn get_player(self) -> Player {
    match self {
      Self::Mcts => Player::Mcts(Mcts::new()),
      Self::Minimax => Player::Minimax,
      Self::Random => Player::Random(Random::new()),
      Self::User => Player::User(User::new()),
    }
  }
}

pub(super) enum Player {
  Mcts(Mcts),
  Minimax,
  Random(Random),
  User(User),
}

impl Player {
  pub(super) fn get_move(&mut self, game: &Game) -> Result<Square, PlayerError> {
    Ok(match self {
      Self::Mcts(mcts) => mcts.get_move(game)?,
      Self::Minimax => minimax::get_move(game)?,
      Self::Random(random) => random.get_move(game)?,
      Self::User(user) => user.get_move(game)?,
    })
  }
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Mcts(_) => write!(f, "monte carlo tree search"),
      Self::Minimax => write!(f, "minimax"),
      Self::Random(_) => write!(f, "random"),
      Self::User(_) => write!(f, "user"),
    }
  }
}
