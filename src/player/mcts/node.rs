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

pub(super) struct Node {
  childrens: Vec<usize>,
  game: Game,
  wins: f64,
  parent: usize,
  playouts: u32,
}

impl Node {
  pub(super) fn new(game: Game, parent: usize) -> Self {
    Self {
      childrens: Vec::with_capacity(Square::COUNT as usize),
      game,
      wins: 0.0,
      parent,
      playouts: 0,
    }
  }

  pub(super) fn get_childrens(&self) -> &Vec<usize> {
    &self.childrens
  }

  pub(super) fn get_game(&self) -> &Game {
    &self.game
  }

  pub(super) fn get_wins(&self) -> f64 {
    self.wins
  }

  pub(super) fn get_parent(&self) -> usize {
    self.parent
  }

  pub(super) fn get_playouts(&self) -> u32 {
    self.playouts
  }

  pub(super) fn add_win(&mut self) {
    self.wins += 1.0;
  }

  pub(super) fn add_draw(&mut self) {
    self.wins += 0.5;
  }

  pub(super) fn add_children(&mut self, index: usize) {
    self.childrens.push(index);
  }

  pub(super) fn add_playout(&mut self) {
    self.playouts += 1;
  }
}
