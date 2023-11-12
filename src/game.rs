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

use thiserror::Error;

#[derive(Error, Debug)]
pub(super) enum GameError {
  #[error("the game is already over ({0})")]
  GameIsOver(Outcome),
  #[error("{0} square is not empty")]
  SquareIsNotEmpty(Square),
}

type Bitboard = u32;

trait BitboardConstants {
  const TOP_LEFT_X: Bitboard = 0b1;
  const TOP_MIDDLE_X: Bitboard = Self::TOP_LEFT_X << 1;
  const TOP_RIGHT_X: Bitboard = Self::TOP_MIDDLE_X << 1;
  const MIDDLE_LEFT_X: Bitboard = Self::TOP_RIGHT_X << 1;
  const MIDDLE_MIDDLE_X: Bitboard = Self::MIDDLE_LEFT_X << 1;
  const MIDDLE_RIGHT_X: Bitboard = Self::MIDDLE_MIDDLE_X << 1;
  const BOTTOM_LEFT_X: Bitboard = Self::MIDDLE_RIGHT_X << 1;
  const BOTTOM_MIDDLE_X: Bitboard = Self::BOTTOM_LEFT_X << 1;
  const BOTTOM_RIGHT_X: Bitboard = Self::BOTTOM_MIDDLE_X << 1;
  const TOP_LEFT_O: Bitboard = Self::BOTTOM_RIGHT_X << 1;
  const TOP_MIDDLE_O: Bitboard = Self::TOP_LEFT_O << 1;
  const TOP_RIGHT_O: Bitboard = Self::TOP_MIDDLE_O << 1;
  const MIDDLE_LEFT_O: Bitboard = Self::TOP_RIGHT_O << 1;
  const MIDDLE_MIDDLE_O: Bitboard = Self::MIDDLE_LEFT_O << 1;
  const MIDDLE_RIGHT_O: Bitboard = Self::MIDDLE_MIDDLE_O << 1;
  const BOTTOM_LEFT_O: Bitboard = Self::MIDDLE_RIGHT_O << 1;
  const BOTTOM_MIDDLE_O: Bitboard = Self::BOTTOM_LEFT_O << 1;
  const BOTTOM_RIGHT_O: Bitboard = Self::BOTTOM_MIDDLE_O << 1;
  const EMPTY_SQUARE: Bitboard = !Bitboard::TOP_LEFT_X & Bitboard::TOP_LEFT_X;
  const X_MASK: Bitboard = Self::TOP_LEFT_X
    | Self::TOP_MIDDLE_X
    | Self::TOP_RIGHT_X
    | Self::MIDDLE_LEFT_X
    | Self::MIDDLE_MIDDLE_X
    | Self::MIDDLE_RIGHT_X
    | Self::BOTTOM_LEFT_X
    | Self::BOTTOM_MIDDLE_X
    | Self::BOTTOM_RIGHT_X;
  const TOP_LEFT_TO_BOTTOM_RIGHT_X: Bitboard =
    Self::TOP_LEFT_X | Self::MIDDLE_MIDDLE_X | Self::BOTTOM_RIGHT_X;
  const TOP_RIGHT_TO_BOTTOM_LEFT_X: Bitboard =
    Self::TOP_RIGHT_X | Self::MIDDLE_MIDDLE_X | Self::BOTTOM_LEFT_X;
  const ROW_TOP_X: Bitboard = Self::TOP_LEFT_X | Self::TOP_MIDDLE_X | Self::TOP_RIGHT_X;
  const ROW_MIDDLE_X: Bitboard = Self::MIDDLE_LEFT_X | Self::MIDDLE_MIDDLE_X | Self::MIDDLE_RIGHT_X;
  const ROW_BOTTOM_X: Bitboard = Self::BOTTOM_LEFT_X | Self::BOTTOM_MIDDLE_X | Self::BOTTOM_RIGHT_X;
  const COLUMN_LEFT_X: Bitboard = Self::TOP_LEFT_X | Self::MIDDLE_LEFT_X | Self::BOTTOM_LEFT_X;
  const COLUMN_MIDDLE_X: Bitboard =
    Self::TOP_MIDDLE_X | Self::MIDDLE_MIDDLE_X | Self::BOTTOM_MIDDLE_X;
  const COLUMN_RIGHT_X: Bitboard = Self::TOP_RIGHT_X | Self::MIDDLE_RIGHT_X | Self::BOTTOM_RIGHT_X;
  const TOP_LEFT_TO_BOTTOM_RIGHT_O: Bitboard =
    Self::TOP_LEFT_O | Self::MIDDLE_MIDDLE_O | Self::BOTTOM_RIGHT_O;
  const TOP_RIGHT_TO_BOTTOM_LEFT_O: Bitboard =
    Self::TOP_RIGHT_O | Self::MIDDLE_MIDDLE_O | Self::BOTTOM_LEFT_O;
  const ROW_TOP_O: Bitboard = Self::TOP_LEFT_O | Self::TOP_MIDDLE_O | Self::TOP_RIGHT_O;
  const ROW_MIDDLE_O: Bitboard = Self::MIDDLE_LEFT_O | Self::MIDDLE_MIDDLE_O | Self::MIDDLE_RIGHT_O;
  const ROW_BOTTOM_O: Bitboard = Self::BOTTOM_LEFT_O | Self::BOTTOM_MIDDLE_O | Self::BOTTOM_RIGHT_O;
  const COLUMN_LEFT_O: Bitboard = Self::TOP_LEFT_O | Self::MIDDLE_LEFT_O | Self::BOTTOM_LEFT_O;
  const COLUMN_MIDDLE_O: Bitboard =
    Self::TOP_MIDDLE_O | Self::MIDDLE_MIDDLE_O | Self::BOTTOM_MIDDLE_O;
  const COLUMN_RIGHT_O: Bitboard = Self::TOP_RIGHT_O | Self::MIDDLE_RIGHT_O | Self::BOTTOM_RIGHT_O;
  const TOP_LEFT: Bitboard = Self::TOP_LEFT_X | Self::TOP_LEFT_O;
  const TOP_MIDDLE: Bitboard = Self::TOP_MIDDLE_X | Self::TOP_MIDDLE_O;
  const TOP_RIGHT: Bitboard = Self::TOP_RIGHT_X | Self::TOP_RIGHT_O;
  const MIDDLE_LEFT: Bitboard = Self::MIDDLE_LEFT_X | Self::MIDDLE_LEFT_O;
  const MIDDLE_MIDDLE: Bitboard = Self::MIDDLE_MIDDLE_X | Self::MIDDLE_MIDDLE_O;
  const MIDDLE_RIGHT: Bitboard = Self::MIDDLE_RIGHT_X | Self::MIDDLE_RIGHT_O;
  const BOTTOM_LEFT: Bitboard = Self::BOTTOM_LEFT_X | Self::BOTTOM_LEFT_O;
  const BOTTOM_MIDDLE: Bitboard = Self::BOTTOM_MIDDLE_X | Self::BOTTOM_MIDDLE_O;
  const BOTTOM_RIGHT: Bitboard = Self::BOTTOM_RIGHT_X | Self::BOTTOM_RIGHT_O;
  const EMPTY: Bitboard = !Self::TOP_LEFT & Self::TOP_LEFT
    | !Self::TOP_MIDDLE & Self::TOP_MIDDLE
    | !Self::TOP_RIGHT & Self::TOP_RIGHT
    | !Self::MIDDLE_LEFT & Self::MIDDLE_LEFT
    | !Self::MIDDLE_MIDDLE & Self::MIDDLE_MIDDLE
    | !Self::MIDDLE_RIGHT & Self::MIDDLE_RIGHT
    | !Self::BOTTOM_LEFT & Self::BOTTOM_LEFT
    | !Self::BOTTOM_MIDDLE & Self::BOTTOM_MIDDLE
    | !Self::BOTTOM_RIGHT & Self::BOTTOM_RIGHT;
}

impl BitboardConstants for Bitboard {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(super) enum Square {
  TopLeft,
  TopMiddle,
  TopRight,
  MiddleLeft,
  MiddleMiddle,
  MiddleRight,
  BottomLeft,
  BottomMiddle,
  BottomRight,
}

impl Square {
  pub(super) const COUNT: u8 = 9;

  fn get_bitboard_square(&self, side: &Side) -> BitboardSquare {
    match side {
      Side::X => match self {
        Self::TopLeft => BitboardSquare::TopLeftX,
        Self::TopMiddle => BitboardSquare::TopMiddleX,
        Self::TopRight => BitboardSquare::TopRightX,
        Self::MiddleLeft => BitboardSquare::MiddleLeftX,
        Self::MiddleMiddle => BitboardSquare::MiddleMiddleX,
        Self::MiddleRight => BitboardSquare::MiddleRightX,
        Self::BottomLeft => BitboardSquare::BottomLeftX,
        Self::BottomMiddle => BitboardSquare::BottomMiddleX,
        Self::BottomRight => BitboardSquare::BottomRightX,
      },
      Side::O => match self {
        Self::TopLeft => BitboardSquare::TopLeftO,
        Self::TopMiddle => BitboardSquare::TopMiddleO,
        Self::TopRight => BitboardSquare::TopRightO,
        Self::MiddleLeft => BitboardSquare::MiddleLeftO,
        Self::MiddleMiddle => BitboardSquare::MiddleMiddleO,
        Self::MiddleRight => BitboardSquare::MiddleRightO,
        Self::BottomLeft => BitboardSquare::BottomLeftO,
        Self::BottomMiddle => BitboardSquare::BottomMiddleO,
        Self::BottomRight => BitboardSquare::BottomRightO,
      },
    }
  }

  fn get_bitboard(&self) -> Bitboard {
    match self {
      Self::TopLeft => Bitboard::TOP_LEFT,
      Self::TopMiddle => Bitboard::TOP_MIDDLE,
      Self::TopRight => Bitboard::TOP_RIGHT,
      Self::MiddleLeft => Bitboard::MIDDLE_LEFT,
      Self::MiddleMiddle => Bitboard::MIDDLE_MIDDLE,
      Self::MiddleRight => Bitboard::MIDDLE_RIGHT,
      Self::BottomLeft => Bitboard::BOTTOM_LEFT,
      Self::BottomMiddle => Bitboard::BOTTOM_MIDDLE,
      Self::BottomRight => Bitboard::BOTTOM_RIGHT,
    }
  }
}

impl std::fmt::Display for Square {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::TopLeft => write!(f, "top left"),
      Self::TopMiddle => write!(f, "top middle"),
      Self::TopRight => write!(f, "top right"),
      Self::MiddleLeft => write!(f, "middle left"),
      Self::MiddleMiddle => write!(f, "middle middle"),
      Self::MiddleRight => write!(f, "middle right"),
      Self::BottomLeft => write!(f, "bottom left"),
      Self::BottomMiddle => write!(f, "bottom middle"),
      Self::BottomRight => write!(f, "bottom right"),
    }
  }
}

enum BitboardSquare {
  TopLeftX,
  TopMiddleX,
  TopRightX,
  MiddleLeftX,
  MiddleMiddleX,
  MiddleRightX,
  BottomLeftX,
  BottomMiddleX,
  BottomRightX,
  TopLeftO,
  TopMiddleO,
  TopRightO,
  MiddleLeftO,
  MiddleMiddleO,
  MiddleRightO,
  BottomLeftO,
  BottomMiddleO,
  BottomRightO,
}

impl BitboardSquare {
  fn get_bitboard(&self) -> Bitboard {
    match self {
      Self::TopLeftX => Bitboard::TOP_LEFT_X,
      Self::TopMiddleX => Bitboard::TOP_MIDDLE_X,
      Self::TopRightX => Bitboard::TOP_RIGHT_X,
      Self::MiddleLeftX => Bitboard::MIDDLE_LEFT_X,
      Self::MiddleMiddleX => Bitboard::MIDDLE_MIDDLE_X,
      Self::MiddleRightX => Bitboard::MIDDLE_RIGHT_X,
      Self::BottomLeftX => Bitboard::BOTTOM_LEFT_X,
      Self::BottomMiddleX => Bitboard::BOTTOM_MIDDLE_X,
      Self::BottomRightX => Bitboard::BOTTOM_RIGHT_X,
      Self::TopLeftO => Bitboard::TOP_LEFT_O,
      Self::TopMiddleO => Bitboard::TOP_MIDDLE_O,
      Self::TopRightO => Bitboard::TOP_RIGHT_O,
      Self::MiddleLeftO => Bitboard::MIDDLE_LEFT_O,
      Self::MiddleMiddleO => Bitboard::MIDDLE_MIDDLE_O,
      Self::MiddleRightO => Bitboard::MIDDLE_RIGHT_O,
      Self::BottomLeftO => Bitboard::BOTTOM_LEFT_O,
      Self::BottomMiddleO => Bitboard::BOTTOM_MIDDLE_O,
      Self::BottomRightO => Bitboard::BOTTOM_RIGHT_O,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Outcome {
  Draw,
  XWin,
  OWin,
}

impl std::fmt::Display for Outcome {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Draw => write!(f, "draw"),
      Self::XWin => write!(f, "x win"),
      Self::OWin => write!(f, "o win"),
    }
  }
}

#[derive(Clone, PartialEq)]
pub(super) enum Side {
  X,
  O,
}

impl Side {
  fn switch(&mut self) {
    *self = match self {
      Side::X => Side::O,
      Side::O => Side::X,
    };
  }
}

#[derive(PartialEq)]
enum SquareState {
  Empty,
  Occupied,
}

#[derive(Clone)]
pub(super) struct Game {
  outcome: Option<Outcome>,
  side: Side,
  bitboard: Bitboard,
}

impl Game {
  pub(super) fn new() -> Self {
    Self {
      outcome: None,
      side: Side::X,
      bitboard: Bitboard::EMPTY,
    }
  }

  pub(super) fn place_mark(&mut self, square: &Square) -> Result<(), GameError> {
    if let Some(outcome) = &self.outcome {
      return Err(GameError::GameIsOver(outcome.clone()));
    }

    if self.is_square_empty(square) == SquareState::Occupied {
      return Err(GameError::SquareIsNotEmpty(*square));
    }

    self.mark(&square.get_bitboard_square(&self.side));
    self.update_outcome_if_necessary();
    self.side.switch();
    Ok(())
  }

  pub(super) fn get_empty_squares(&self) -> Vec<Square> {
    let mut squares = Vec::with_capacity(Square::COUNT as usize);

    for square in [
      Square::MiddleMiddle,
      Square::TopLeft,
      Square::TopRight,
      Square::BottomLeft,
      Square::BottomRight,
      Square::TopMiddle,
      Square::MiddleLeft,
      Square::MiddleRight,
      Square::BottomMiddle,
    ] {
      if self.is_square_empty(&square) == SquareState::Empty {
        squares.push(square);
      }
    }

    squares
  }

  pub(super) fn get_side_to_move(&self) -> &Side {
    &self.side
  }

  pub(super) fn get_outcome(&self) -> &Option<Outcome> {
    &self.outcome
  }

  pub(super) fn print_grid(&self) {
    println!(
      "|{} {} {}|\n|{} {} {}|\n|{} {} {}|\n",
      match self.bitboard & Bitboard::TOP_LEFT {
        Bitboard::TOP_LEFT_X => 'x',
        Bitboard::TOP_LEFT_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::TOP_MIDDLE {
        Bitboard::TOP_MIDDLE_X => 'x',
        Bitboard::TOP_MIDDLE_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::TOP_RIGHT {
        Bitboard::TOP_RIGHT_X => 'x',
        Bitboard::TOP_RIGHT_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::MIDDLE_LEFT {
        Bitboard::MIDDLE_LEFT_X => 'x',
        Bitboard::MIDDLE_LEFT_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::MIDDLE_MIDDLE {
        Bitboard::MIDDLE_MIDDLE_X => 'x',
        Bitboard::MIDDLE_MIDDLE_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::MIDDLE_RIGHT {
        Bitboard::MIDDLE_RIGHT_X => 'x',
        Bitboard::MIDDLE_RIGHT_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::BOTTOM_LEFT {
        Bitboard::BOTTOM_LEFT_X => 'x',
        Bitboard::BOTTOM_LEFT_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::BOTTOM_MIDDLE {
        Bitboard::BOTTOM_MIDDLE_X => 'x',
        Bitboard::BOTTOM_MIDDLE_O => 'o',
        _ => ' ',
      },
      match self.bitboard & Bitboard::BOTTOM_RIGHT {
        Bitboard::BOTTOM_RIGHT_X => 'x',
        Bitboard::BOTTOM_RIGHT_O => 'o',
        _ => ' ',
      }
    );
  }

  fn update_outcome_if_necessary(&mut self) {
    if self.bitboard & Bitboard::TOP_LEFT_TO_BOTTOM_RIGHT_X == Bitboard::TOP_LEFT_TO_BOTTOM_RIGHT_X
      || self.bitboard & Bitboard::TOP_RIGHT_TO_BOTTOM_LEFT_X
        == Bitboard::TOP_RIGHT_TO_BOTTOM_LEFT_X
      || self.bitboard & Bitboard::ROW_TOP_X == Bitboard::ROW_TOP_X
      || self.bitboard & Bitboard::ROW_MIDDLE_X == Bitboard::ROW_MIDDLE_X
      || self.bitboard & Bitboard::ROW_BOTTOM_X == Bitboard::ROW_BOTTOM_X
      || self.bitboard & Bitboard::COLUMN_LEFT_X == Bitboard::COLUMN_LEFT_X
      || self.bitboard & Bitboard::COLUMN_MIDDLE_X == Bitboard::COLUMN_MIDDLE_X
      || self.bitboard & Bitboard::COLUMN_RIGHT_X == Bitboard::COLUMN_RIGHT_X
    {
      self.outcome = Some(Outcome::XWin);
    } else if self.bitboard & Bitboard::TOP_LEFT_TO_BOTTOM_RIGHT_O
      == Bitboard::TOP_LEFT_TO_BOTTOM_RIGHT_O
      || self.bitboard & Bitboard::TOP_RIGHT_TO_BOTTOM_LEFT_O
        == Bitboard::TOP_RIGHT_TO_BOTTOM_LEFT_O
      || self.bitboard & Bitboard::ROW_TOP_O == Bitboard::ROW_TOP_O
      || self.bitboard & Bitboard::ROW_MIDDLE_O == Bitboard::ROW_MIDDLE_O
      || self.bitboard & Bitboard::ROW_BOTTOM_O == Bitboard::ROW_BOTTOM_O
      || self.bitboard & Bitboard::COLUMN_LEFT_O == Bitboard::COLUMN_LEFT_O
      || self.bitboard & Bitboard::COLUMN_MIDDLE_O == Bitboard::COLUMN_MIDDLE_O
      || self.bitboard & Bitboard::COLUMN_RIGHT_O == Bitboard::COLUMN_RIGHT_O
    {
      self.outcome = Some(Outcome::OWin);
    } else if (self.bitboard | self.bitboard >> 9) & Bitboard::X_MASK == Bitboard::X_MASK {
      self.outcome = Some(Outcome::Draw);
    }
  }

  fn is_square_empty(&self, square: &Square) -> SquareState {
    if self.bitboard & square.get_bitboard() == Bitboard::EMPTY {
      SquareState::Empty
    } else {
      SquareState::Occupied
    }
  }

  fn mark(&mut self, bitboard_square: &BitboardSquare) {
    self.bitboard |= bitboard_square.get_bitboard();
  }
}
