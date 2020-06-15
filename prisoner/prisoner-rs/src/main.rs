use rand::thread_rng;
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
    fn new() -> Self;
    fn make_move(&self) -> &str;
}

trait ComplexAgent: Agent {
    fn add_points(&mut self, points: u32);
    fn reset(&mut self);
}

impl Agent for Cooperator {
    fn new() -> Cooperator {
        Cooperator { points: 0 }
    }
    fn make_move(&self) -> &str {
        "cooperate"
    }
}

impl Agent for Defector {
    fn new() -> Defector {
        Defector { points: 0 }
    }
    fn make_move(&self) -> &str {
        "defect"
    }
}

impl Agent for Random {
    fn new() -> Random {
        Random {
            points: 0,
            choices: ["cooperate".to_string(), "defect".to_string()],
        }
    }
    fn make_move(&self) -> &str {
        &self.choices.choose(&mut thread_rng()).unwrap()
    }
}

impl Agent for Resentful {
    fn new() -> Resentful {
        Resentful {
            points: 0,
            next_move: "cooperate".to_string(),
        }
    }
    fn make_move(&self) -> &str {
        &self.next_move
    }
}

impl ComplexAgent for Resentful {
    fn add_points(&mut self, points: u32) {
        self.points += points;
        if points < 2 {
            self.next_move = "defect".to_string();
        }
    }
    fn reset(&mut self) {
        self.next_move = "cooperate".to_string();
    }
}

impl Agent for TitForTat {
    fn new() -> TitForTat {
        TitForTat {
            points: 0,
            next_move: "cooperate".to_string(),
        }
    }
    fn make_move(&self) -> &str {
        &self.next_move
    }
}

impl ComplexAgent for TitForTat {
    fn add_points(&mut self, points: u32) {
        self.points += points;
        if points < 2 {
            self.next_move = "defect".to_string();
        } else {
            self.next_move = "cooperate".to_string();
        }
    }
    fn reset(&mut self) {
        self.next_move = "cooperate".to_string();
    }
}

struct Tournament<T: Agent> {
    p1: T,
    p2: T
}

impl<T: Agent> Tournament<T> {
    fn new(p1: T, p2: T) -> Tournament<T> {
        p1.reset();
        p2.reset();
        Tournament {
            p1,
            p2
        }
    }
    fn add_points(&mut self, p1points: u32, p2points: u32) {
        self.p1.add_points(p1points);
        self.p2.add_points(p2points);
    }
    fn play_round(&self) {
        let p1move = self.p1.make_move();
        let p2move = self.p2.make_move();
        if p1move == p2move && p1move == "cooperate" {
            self.add_points(3, 3);
        } else if p1move == p2move && p1move == "defect" {
            self.add_points(1, 1);
        } else if p1move == "defect" {
            self.add_points(5, 0);
        } else if p2move == "defect" { 
            self.add_points(0, 5);
        }
    }
}

fn main() {
}
