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

mod node;
use crate::game::{Game, GameError, Outcome, Side, Square};
use node::Node;
use rand::{prelude::SliceRandom, rngs::ThreadRng};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MctsError {
  #[error(transparent)]
  Game(#[from] GameError),
  #[error("child node slice is empty")]
  ChildNodesIndexesSliceEmpty,
  #[error("no moves can be made")]
  NoSquaresAvailable,
  #[error("unable to select a move")]
  UnableToChooseMove,
}

pub(crate) struct Mcts {
  nodes: Vec<Node>,
  square_map: HashMap<usize, Square>,
  random: ThreadRng,
  outcome: Outcome,
  node_index: usize,
}

impl Mcts {
  const ROOT_NODE: usize = 0;

  pub(super) fn new() -> Self {
    const NODES_CAPACITY: usize = 262144;
    Self {
      nodes: Vec::with_capacity(NODES_CAPACITY),
      square_map: HashMap::with_capacity(Square::COUNT as usize),
      random: rand::thread_rng(),
      outcome: Outcome::Draw,
      node_index: Self::ROOT_NODE,
    }
  }

  pub(super) fn get_move(&mut self, game: &Game) -> Result<Square, MctsError> {
    self.mcts(game)
  }

  fn uct(wins: f64, playouts: f64, parent_playouts: f64) -> f64 {
    wins / playouts + std::f64::consts::SQRT_2 * (parent_playouts.ln() / playouts).sqrt()
  }

  fn select(&mut self) {
    self.node_index = Self::ROOT_NODE;

    loop {
      let node = &self.nodes[self.node_index];
      let node_childrens = node.get_childrens();

      if node_childrens.is_empty() {
        return;
      }

      let mut best_score = f64::MIN;

      for child in node_childrens {
        let child_node = &self.nodes[*child];
        let child_node_playouts = child_node.get_playouts();

        if child_node_playouts == 0 {
          self.node_index = *child;
          return;
        }

        let score = Self::uct(
          child_node.get_wins(),
          child_node_playouts as f64,
          node.get_playouts() as f64,
        );

        if score > best_score {
          best_score = score;
          self.node_index = *child;
        }
      }
    }
  }

  fn expand(&mut self) -> Result<(), MctsError> {
    if self.nodes[self.node_index]
      .get_game()
      .get_outcome()
      .is_some()
    {
      return Ok(());
    }

    for square in self.nodes[self.node_index].get_game().get_empty_squares() {
      let children = self.nodes.len();
      let mut game = self.nodes[self.node_index].get_game().clone();
      game.place_mark(&square)?;
      self.nodes.push(Node::new(game, self.node_index));
      self.nodes[self.node_index].add_children(children);
    }

    self.node_index = *match self.nodes[self.node_index]
      .get_childrens()
      .choose(&mut self.random)
    {
      Some(index) => index,
      None => Err(MctsError::ChildNodesIndexesSliceEmpty)?,
    };

    Ok(())
  }

  fn simulate(&mut self) -> Result<(), MctsError> {
    let mut game = self.nodes[self.node_index].get_game().clone();

    loop {
      let result = game.get_outcome();

      if let Some(result) = result {
        self.outcome = result.clone();
        return Ok(());
      }

      game.place_mark(match game.get_empty_squares().choose(&mut self.random) {
        Some(square) => square,
        None => Err(MctsError::NoSquaresAvailable)?,
      })?;
    }
  }

  fn backpropagate(&mut self) {
    let mut node = &mut self.nodes[self.node_index];

    if self.outcome == Outcome::Draw {
      loop {
        node.add_playout();
        node.add_draw();

        if self.node_index == Self::ROOT_NODE {
          return;
        }

        self.node_index = node.get_parent();
        node = &mut self.nodes[self.node_index];
      }
    }

    let side = node.get_game().get_side_to_move();

    let mut add_win = self.outcome == Outcome::XWin && *side == Side::O
      || self.outcome == Outcome::OWin && *side == Side::X;

    loop {
      node.add_playout();

      if add_win {
        node.add_win();
      }

      if self.node_index == Self::ROOT_NODE {
        return;
      }

      self.node_index = node.get_parent();
      node = &mut self.nodes[self.node_index];
      add_win = !add_win;
    }
  }

  fn search(&mut self) -> Result<(), MctsError> {
    const ROUNDS: u32 = 8190;

    for _ in 0..ROUNDS {
      self.select();
      self.expand()?;
      self.simulate()?;
      self.backpropagate();
    }

    Ok(())
  }

  fn initialize(&mut self, game: &Game) -> Result<(), MctsError> {
    self.nodes.clear();
    self.nodes.push(Node::new(game.clone(), usize::MAX));
    self.square_map.clear();

    for square in game.get_empty_squares() {
      let children = self.nodes.len();
      let mut game_clone = game.clone();
      game_clone.place_mark(&square)?;
      self.nodes.push(Node::new(game_clone, Self::ROOT_NODE));
      self.nodes[Self::ROOT_NODE].add_children(children);
      self.square_map.insert(children, square);
    }

    self.node_index = *match self.nodes[Self::ROOT_NODE]
      .get_childrens()
      .choose(&mut self.random)
    {
      Some(index) => index,
      None => Err(MctsError::ChildNodesIndexesSliceEmpty)?,
    };

    self.simulate()?;
    self.backpropagate();
    Ok(())
  }

  fn choose(&mut self) -> Result<Square, MctsError> {
    let mut playouts = 0;
    let mut best_square = None;

    for (index, square) in &self.square_map {
      let node_playouts = self.nodes[*index].get_playouts();

      if playouts < node_playouts {
        playouts = node_playouts;
        best_square = Some(square);
      }
    }

    match best_square {
      Some(square) => Ok(*square),
      None => Err(MctsError::UnableToChooseMove),
    }
  }

  fn mcts(&mut self, game: &Game) -> Result<Square, MctsError> {
    self.initialize(game)?;
    self.search()?;
    self.choose()
  }
}
