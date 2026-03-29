use itertools::Itertools;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::hexo::state::{Coord, Player, State};

struct Node {
    tile: Coord,
    children: Option<(usize, usize)>,
    visits: usize,
    score: f32
}

pub struct Searcher {
    nodes: Vec<Rc<RefCell<Node>>>,
    state: State,
    
}

impl Searcher {
    pub fn grow_once(&mut self) {

        let nlen = self.nodes.len();
        let mut path = vec![0];
        let mut node = self.nodes[0].borrow_mut();
        self.state.play(node.tile);

        while !node.children.is_none() {
            // Select child
            let idx = self.select_puct(&node);
            node = self.nodes[idx].borrow_mut();
            self.state.play(node.tile);
            path.push(idx);
        }


        let mut eval = 0.0;
        let mut i = 0;
        let mut newnodes = vec![];

        // if node.tile == (-1,0) {
        //     println!("{:?} {:?}", self.state.get_winner(), path);
        // }

        match self.state.get_winner() {
            None => {
                // Expand
                // Only search in the vicinity of the last move bc idc
                for (x,y) in self.state.get_focused_tiles() {
                    self.state.play((x, y));
                    eval += sigmoid(self.state.eval() as f32);
                    newnodes.push( Rc::new(RefCell::new(Node {tile: (x, y), children: None, visits: 1, score: sigmoid(self.state.eval() as f32) })) );
                    self.state.unplay();
                    i+=1;
                }
                node.children = Some((nlen,nlen+i));
            },
            Some(Player::Plr1) => {
                // println!("Plr1 win found {path:?}");
                eval = 2.0;
            },
            Some(Player::Plr2) => {
                // println!("Plr2 win found {path:?}");
                eval = -2.0;
            }
        }

        drop(node);

        // Update evaluations
        for p in path.iter().rev() {
            let mut node= self.nodes[*p].borrow_mut();
            node.visits += 1;
            node.score += eval / (i + 1) as f32;
            self.state.unplay();
        }

        self.nodes.extend(newnodes);
    }

    fn select_puct(&self, node: &Node) -> usize {
        let mut best_uct = -9999.0;
        let mut best_idx = 0;


        for idx in node.children.unwrap().0..node.children.unwrap().1 {
            let child = self.nodes[idx].borrow();
            let wr = child.score / child.visits as f32 / 2.0 * if self.state.is_player_one_turn { 1.0 } else { -1.0 };
            let e = (node.visits as f32 * 2.0).sqrt();
            let p = 1.0 / (node.children.unwrap().1 - node.children.unwrap().0) as f32;


            let uct = wr + e * p / (1.0 + child.visits as f32);
            
            if uct > best_uct {
                best_uct = uct;
                best_idx = idx;
            }
        }

        return best_idx;
    }

    pub fn get_move(&mut self) -> Coord {
        let mut best_wr = -999.0;
        let mut best_coord = (0,0);
        
        let node = self.nodes[0].borrow();
        self.state.play(node.tile);

        for idx in node.children.unwrap().0..node.children.unwrap().1 {
            let n2 = self.nodes[idx].borrow();
            let wr = n2.score / n2.visits as f32  * if self.state.is_player_one_turn { 1.0 } else { -1.0 };

            if wr > best_wr {
                best_wr = wr;
                best_coord = n2.tile;
            }

        }
        self.state.unplay();

        best_coord
    }

    pub fn go(&mut self, cnt: usize) -> Coord {
        for _ in 0..cnt {
            self.grow_once();
        }

        self.get_move()
    }
}

impl Searcher {
    pub fn new(state: State, last_move: Coord) -> Self {
        let mut nstate= state.clone();
        nstate.unplay();
        Self {
            nodes: vec![Rc::new(RefCell::new(Node {tile: last_move, children: None, visits: 0, score: 0.0}))],
            state
        }
    }
}

impl Searcher {
    pub fn print(&self) {
        for (i, node) in self.nodes.iter().clone().enumerate().take(100000) {
            let nd = node.borrow();
            if nd.visits > 1 {
                println!("{i}: Move: {:?} Visits: {} Score: {} ({}) Children: {:?}", nd.tile, nd.visits, nd.score, nd.score/nd.visits as f32, nd.children);
            }
        }
    }
}

fn sigmoid(x: f32) -> f32 {
    return 2.0 / (1.0 + (-x).exp()) - 1.0;
}