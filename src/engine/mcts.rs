use itertools::Itertools;
use std::cell::RefCell;
use crate::hexo::state::{Coord, State};

struct Node {
    tile: Coord,
    children: Vec<usize>,
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
            // select child (always first one in this case)
            let idx = node.children[0];
            node = self.nodes[idx].get_mut();
            path.push(0usize);
        }

        // Expand
        // Only search in the vicinity of the last move bc idc
        let tile = node.tile;
        for (x,y) in (-8..=8).cartesian_product(-8..=8) {
            if self.state.is_open((tile.0 + x, tile.1 + y)) {
                node.children.push(nlen);
                self.nodes.push( RefCell::new(Node {tile: (tile.0 + x, tile.1 + y), children: vec![] }) );

                break;
            }
        }

        // Goof
        
        
    }
}