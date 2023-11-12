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

use crate::game::{Game, GameError, Outcome, Side, Square};
use std::cmp;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MinimaxError {
  #[error(transparent)]
  Game(#[from] GameError),
  #[error("no empty squares are available on the current node")]
  NoEmptySquares,
}

type Depth = u8;
type Value = u8;

const X_WIN: Value = 32;
const DRAW: Value = X_WIN / 2;
const O_WIN: Value = 0;

pub(super) fn get_move(game: &Game) -> Result<Square, MinimaxError> {
  if *game.get_side_to_move() == Side::X {
    get_best_square_max(game)
  } else {
    get_best_square_min(game)
  }
}

fn get_value(outcome: &Outcome, depth: &Depth) -> Value {
  match outcome {
    Outcome::Draw => DRAW,
    Outcome::XWin => X_WIN - depth,
    Outcome::OWin => O_WIN + depth,
  }
}

fn max(node: &Game, depth: &Depth) -> Result<Value, MinimaxError> {
  let mut value = O_WIN;

  for square in node.get_empty_squares() {
    let mut game = node.clone();
    game.place_mark(&square)?;
    value = cmp::max(
      match game.get_outcome() {
        Some(outcome) => get_value(outcome, depth),
        None => min(&game, &(depth + 1))?,
      },
      value,
    );
  }

  Ok(value)
}

fn min(node: &Game, depth: &Depth) -> Result<Value, MinimaxError> {
  let mut value = X_WIN;

  for square in node.get_empty_squares() {
    let mut game = node.clone();
    game.place_mark(&square)?;
    value = cmp::min(
      match game.get_outcome() {
        Some(outcome) => get_value(outcome, depth),
        None => max(&game, &(depth + 1))?,
      },
      value,
    );
  }

  Ok(value)
}

fn get_best_square_max(game: &Game) -> Result<Square, MinimaxError> {
  let mut best_square = None;
  let mut best_value = O_WIN;

  for square in game.get_empty_squares() {
    let mut node = game.clone();
    node.place_mark(&square)?;

    let value = match node.get_outcome() {
      Some(outcome) => get_value(outcome, &0),
      None => min(&node, &1)?,
    };

    if value > best_value {
      best_value = value;
      best_square = Some(square);
    }
  }

  match best_square {
    Some(square) => Ok(square),
    None => Err(MinimaxError::NoEmptySquares),
  }
}

fn get_best_square_min(game: &Game) -> Result<Square, MinimaxError> {
  let mut best_square = None;
  let mut best_value = X_WIN;

  for square in game.get_empty_squares() {
    let mut node = game.clone();
    node.place_mark(&square)?;

    let value = match node.get_outcome() {
      Some(outcome) => get_value(outcome, &0),
      None => max(&node, &1)?,
    };

    if value < best_value {
      best_value = value;
      best_square = Some(square);
    }
  }

  match best_square {
    Some(square) => Ok(square),
    None => Err(MinimaxError::NoEmptySquares),
  }
}
