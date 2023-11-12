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

use crate::game::{Game, Square};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RandomError {
  #[error("unable to choose a move because there are no empty squares")]
  UnableToChooseMove,
}

pub(crate) struct Random {
  random: ThreadRng,
}

impl Random {
  pub(super) fn new() -> Self {
    Self {
      random: rand::thread_rng(),
    }
  }

  pub(super) fn get_move(&mut self, game: &Game) -> Result<Square, RandomError> {
    match game.get_empty_squares().choose(&mut self.random) {
      Some(square) => Ok(*square),
      None => Err(RandomError::UnableToChooseMove),
    }
  }
}
