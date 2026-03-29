use itertools::Itertools;
use std::{cell::RefCell, intrinsics::expf32};
use crate::hexo::state::{Coord, State};

struct Node {
    tile: Coord,
    children: Vec<usize>,
    visits: usize,
    score: f32
}

pub struct Searcher {
    nodes: Vec<RefCell<Node>>,
    state: State,
    
}

impl Searcher {
    pub fn grow_once(&mut self) {
        let nlen = self.nodes.len();
        let mut path = vec![];
        let mut node = self.nodes[0].get_mut();

        while !node.children.is_empty() {
            // select child (always first one until I write actual code)
            let idx = node.children[0];
            node = self.nodes[idx].get_mut();
            path.push(idx);
        }

        // Expand
        // Only search in the vicinity of the last move bc idc
        let mut eval = 0;
        let tile = node.tile;
        for (x,y) in (-8..=8).cartesian_product(-8..=8) {
            if self.state.is_open((tile.0 + x, tile.1 + y)) {
                node.children.push(nlen);
                self.state.play((tile.0 + x, tile.1 + y));
                eval = self.state.eval();
                self.nodes.push( RefCell::new(Node {tile: (tile.0 + x, tile.1 + y), children: vec![], visits: 1, score: sigmoid(eval as f32) }) );

                break;
            }
        }

        // Update evaluations
        for p in path {
            let node = self.nodes[p].get_mut();
            node.visits += 1;
            node.score += sigmoid(eval as f32);
        }
    }
}

fn sigmoid(x: f32) -> f32 {
    return 2 / (1 + expf32(-x)) - 1;
}