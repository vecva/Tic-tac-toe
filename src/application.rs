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
  controller::{Controller, ControllerError, GameCount},
  player::PlayerType,
};
use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
#[command(
  version,
  about = "Tic-tac-toe game",
  long_about = "This command-line application allows you to play the classic game of Tic-tac-toe against a friend or an AI opponent"
)]
struct Arguments {
  /// Sets the player for 'x'
  #[arg(short = 'x', long, value_enum, default_value_t=PlayerType::PLAYER_X_DEFAULT)]
  player_x: PlayerType,

  /// Sets the player for 'o'
  #[arg(short = 'o', long, value_enum, default_value_t=PlayerType::PLAYER_O_DEFAULT)]
  player_o: PlayerType,

  /// Sets the number of games to be played
  #[arg(
      short,
      long,
      value_name = "GAMES",
      default_value_t = Controller::MINIMUM_GAMES_COUNT,
      value_parser = clap::value_parser!(GameCount).range(Controller::MINIMUM_GAMES_COUNT as i64..)
  )]
  game_count: GameCount,
}

#[derive(Error, Debug)]
pub(super) enum ApplicationError {
  #[error(transparent)]
  Controller(#[from] ControllerError),
}

pub(super) fn main() -> Result<(), ApplicationError> {
  let arguments = Arguments::parse();
  Controller::new(
    arguments.player_x.get_player(),
    arguments.player_o.get_player(),
    arguments.game_count,
  )
  .engage()?;
  Ok(())
}
