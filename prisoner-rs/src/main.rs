use rand::thread_rng;
use std::cell::RefCell;
use rand::seq::SliceRandom;

struct Cooperator {
    points: u32,
}

struct Defector {
    points: u32,
}

struct Random {
    points: u32,
    choices: [String; 2],
}

struct Resentful {
    points: u32,
    next_move: String,
}

struct TitForTat {
    points: u32,
    next_move: String,
}

trait Agent {
    fn make_move(&self) -> &str;
    fn add_points(&mut self, points: u32);
    fn get_points(&self) -> u32;
    fn get_class(&self) -> &str;
}

trait ComplexAgent: Agent {
    fn reset(&mut self);
}

impl Agent for Cooperator {
    fn make_move(&self) -> &str {
        "cooperate"
    }
    fn add_points(&mut self, points: u32) {
        self.points += points;
    }
    fn get_points(&self) -> u32 {
        self.points
    }
    fn get_class(&self) -> &str {
        "cooperator"
    }
}

impl Agent for Defector {
    fn make_move(&self) -> &str {
        "defect"
    }
    fn add_points(&mut self, points: u32) {
        self.points += points;
    }
    fn get_points(&self) -> u32 {
        self.points
    }
    fn get_class(&self) -> &str {
        "defector"
    }
}

impl Agent for Random {
    fn make_move(&self) -> &str {
        &self.choices.choose(&mut thread_rng()).unwrap()
    }
    fn add_points(&mut self, points: u32) {
        self.points += points;
    }
    fn get_points(&self) -> u32 {
        self.points
    }
    fn get_class(&self) -> &str {
        "random"
    }
}

impl Agent for Resentful {
    fn make_move(&self) -> &str {
        &self.next_move
    }
    fn add_points(&mut self, points: u32) {
        self.points += points;
        if points < 2 {
            self.next_move = "defect".to_string();
        };
    }
    fn get_points(&self) -> u32 {
        self.points
    }
    fn get_class(&self) -> &str {
        "resentful"
    }
}

impl ComplexAgent for Resentful {
    fn reset(&mut self) {
        self.next_move = "cooperate".to_string();
    }
}

impl Agent for TitForTat {
    fn make_move(&self) -> &str {
        &self.next_move
    }
    fn add_points(&mut self, points: u32) {
        self.points += points;
        if points < 2 {
            self.next_move = "defect".to_string();
        } else {
            self.next_move = "cooperate".to_string();
        };
    }
    fn get_points(&self) -> u32 {
        self.points
    }
    fn get_class(&self) -> &str {
        "titfortat"
    }
}

impl ComplexAgent for TitForTat {
    fn reset(&mut self) {
        self.next_move = "cooperate".to_string();
    }
}


struct Tournament {
    players: Vec<RefCell<Box<dyn Agent>>>,
}

impl Tournament {
    fn play_round(&self, p1idx: usize, p2idx: usize) {
        let mut p1 = self.players[p1idx].borrow_mut();
        let mut p2 = self.players[p2idx].borrow_mut();
        let p1move = p1.make_move();
        let p2move = p2.make_move();
        if p1move == p2move && p1move == "cooperate" {
            p1.add_points(3);
            p2.add_points(3);
        } else if p1move == p2move && p1move == "defect" {
            p1.add_points(1);
            p2.add_points(1);
        } else if p1move == "defect" {
            p1.add_points(5);
            p2.add_points(0);
        } else if p2move == "defect" { 
            p1.add_points(0);
            p2.add_points(5);
        }
    }
    fn get_size(&self) -> usize {
        self.players.len()
    }
    fn get_scores(&self) {
        for i in 0..self.get_size() {
            let player = self.players[i as usize].borrow();
            println!("{}: \t {}", player.get_class(), player.get_points());
        }
    }
}


fn main() {
    let tourn: Tournament = Tournament {
        players: vec![
            RefCell::new(Box::new(Cooperator { points: 0 })),
            RefCell::new(Box::new(Defector { points: 0 })),
            RefCell::new(Box::new(Random { points: 0, choices: ["cooperate".to_string(), "defect".to_string()]})),
            RefCell::new(Box::new(Resentful { points: 0, next_move: "cooperate".to_string() })),
            RefCell::new(Box::new(TitForTat { points: 0, next_move: "cooperate".to_string() })),
        ]
    };
    for i in 0..tourn.get_size() {
        for j in i+1..tourn.get_size() {
            for _ in 0..1000 {
                tourn.play_round(i as usize, j as usize);
            }
        }
    };
    tourn.get_scores();
}
