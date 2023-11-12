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
use std::{collections::HashSet, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum UserError {
  #[error(transparent)]
  Io(#[from] io::Error),
}

pub(crate) struct User {
  empty_squares: HashSet<Square>,
  input: String,
}

impl User {
  pub(super) fn new() -> Self {
    const INPUT_CAPACITY: usize = 2;
    Self {
      empty_squares: HashSet::with_capacity(Square::COUNT as usize),
      input: String::with_capacity(INPUT_CAPACITY),
    }
  }

  pub(super) fn get_move(&mut self, game: &Game) -> Result<Square, UserError> {
    self.empty_squares.clear();
    self.empty_squares.extend(game.get_empty_squares());

    loop {
      let square = self.get_input()?;

      if self.empty_squares.contains(&square) {
        return Ok(square);
      }

      println!("{} square is not empty", square)
    }
  }

  fn get_input(&mut self) -> Result<Square, UserError> {
    loop {
      self.input.clear();
      io::stdin().read_line(&mut self.input)?;

      match self.input.trim().parse::<u8>() {
        Ok(number) => match number {
          1 => return Ok(Square::BottomLeft),
          2 => return Ok(Square::BottomMiddle),
          3 => return Ok(Square::BottomRight),
          4 => return Ok(Square::MiddleLeft),
          5 => return Ok(Square::MiddleMiddle),
          6 => return Ok(Square::MiddleRight),
          7 => return Ok(Square::TopLeft),
          8 => return Ok(Square::TopMiddle),
          9 => return Ok(Square::TopRight),
          _ => eprintln!("number entered is not within the acceptable range"),
        },
        Err(error) => eprintln!("{}", error),
      }

      eprintln!("please try again with a number between 1 and 9");
    }
  }
}
