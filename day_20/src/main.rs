use std::str::FromStr;
use std::collections::HashSet;
fn main() {
    let reg = "^ENWWW(NEEE|SSE(EE|N))$";
    let reg: Reg = reg.parse().unwrap();
    let mut state = State::new();
    state.paths = state.step(&reg, vec![vec![Pos{row: SIZE/2, col: SIZE/2}]]);
    for p in &state.paths{
        println!("{:?}", p);
    }
    let doors = state.update();
    println!("{}", state.to_string());
    println!("{}", doors);

}

#[derive(Debug, Clone)]
struct Room {
    N: bool,
    S: bool,
    E: bool,
    W: bool,
}

#[derive(Clone)]
enum Space {
    Wall,
    Empty,
    Door,
    Unknown,
}

impl From<&Space> for char {
    fn from(sp: &Space) -> char {
        match sp {
            Space::Wall => '#',
            Space::Empty => ' ',
            Space::Door => ' ',
            Space::Unknown => '?',
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

struct State {
    paths: Vec<Vec<Pos>>,
    set: HashSet<(Pos, Pos)>,
}

const SIZE: usize = 30;

impl ToString for State {
    fn to_string(&self) -> String {
        let mut st = String::new();
        for r in 0..SIZE * 2 {
            for c in 0..SIZE * 2 {
                // odds are the transition columns
                if r % 2 == 1 || c % 2 == 1 {
                    // if self.set.contains(())
                } 
            }
            st.push('\n')
        }
        st
    }
}

impl State {
    fn new() -> Self {
        Self {
            paths: vec![vec![Pos {
                row: SIZE / 2,
                col: SIZE / 2,
            }]],
            set: HashSet::new(),
        }
    }

    // number of doors.
    fn update(&mut self) -> usize {
        let mut set = HashSet::new();
        for path in &self.paths {
            println!("path: {:?}", path);
            let mut path = path.iter();
            let last = path.nth(0).unwrap().clone();
            for spot in path {
                set.insert((last.clone(), spot.clone()));
                set.insert((spot.clone(), last.clone()));
            }
        }
        self.set = set;
        self.set.len() / 2
    }

    /**
     * Each *path* represents a possible path our program can take.
     * Each 'normal' step will try to move EACH tail of each path by that direction
     * &if this fails, remove that path
     *
     * Ors will create a sub path for each sub expression.
     * For each path
     *  for each sub path that doesn't fail
     *    a new path is created (replacing the path that it went from)
     */
    fn step(&mut self, re: &Reg, current_path: Vec<Vec<Pos>>) -> Vec<Vec<Pos>> {
        let mut all_paths = vec![];
        match re {
            Reg::Or(parts) => {
                for group in parts {
                    let mut option_path = current_path.clone();
                    println!("option group from: {:?}", option_path);
                    for exp in group {
                        option_path = self.step(exp, option_path);
                    }
                    all_paths.append(&mut option_path);
                }
            }
            _ => {
                for i in 0..current_path.len() {
                    let mut new_step = current_path[i].last().unwrap().clone();
                    match re {
                        Reg::N  => new_step.row -= 1,
                        Reg::S  => new_step.row += 1,
                        Reg::E  => new_step.col += 1,
                        Reg::W  => new_step.col -= 1, 
                        _ => {}
                    }
                    println!("Stepping {:?} from {:?} to {:?}", re, current_path[i].last().unwrap().clone(), new_step);
                    let mut current_sub_path = current_path[i].clone();
                    current_sub_path.push(new_step);
                    all_paths.push(current_sub_path);
                }
            },
        }
        all_paths
    }
}

#[derive(Debug, PartialEq)]
enum Reg {
    N,
    E,
    W,
    S,
    Or(Vec<Vec<Reg>>),
}

impl FromStr for Reg {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut stack: Vec<Vec<Reg>> = Vec::new();
        stack.push(Vec::new());
        // build a stack of the 'current' reg group we are building.
        // starting with the first outer group
        while let Some(c) = chars.nth(0) {
            match c {
                '^' | '$' => continue,
                '(' => {
                    // push an or onto the current group
                    // push a new group onto the stack
                    stack.last_mut().unwrap().push(Reg::Or(vec![]));
                    stack.push(Vec::new());
                }
                ')' => {
                    // pop the current group
                    // place it into the last element of the last group. (which should be an Or)
                    let cur_group = stack.pop().unwrap();
                    let or = stack.last_mut().unwrap().last_mut().unwrap();
                    match or {
                        Reg::Or(branches) => branches.push(cur_group),
                        _ => panic!("last element isn't an or group?"),
                    }
                }
                '|' => {
                    // Same as the close group, but also add a new group to keep building
                    let cur_group = stack.pop().unwrap();
                    let or = stack.last_mut().unwrap().last_mut().unwrap();
                    match or {
                        Reg::Or(branches) => branches.push(cur_group),
                        _ => panic!("last element isn't an or group?"),
                    }
                    stack.push(Vec::new())
                }
                'N' => stack.last_mut().unwrap().push(Reg::N),
                'S' => stack.last_mut().unwrap().push(Reg::S),
                'E' => stack.last_mut().unwrap().push(Reg::E),
                'W' => stack.last_mut().unwrap().push(Reg::W),
                _ => panic!("I can't handle: {}", c),
            };
        }
        let main = stack.pop().unwrap();
        Ok(Reg::Or(vec![main]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let reg = "^WNE$";
        let reg: Reg = reg.parse().unwrap();
        assert_eq!(reg, Reg::Or(vec![vec![Reg::W, Reg::N, Reg::E]]))
    }
    #[test]
    fn test_or() {
        let reg = "^W(NE|SW|)$";
        let reg: Reg = reg.parse().unwrap();
        assert_eq!(
            reg,
            Reg::Or(vec![vec![
                Reg::W,
                Reg::Or(vec![vec![Reg::N, Reg::E], vec![Reg::S, Reg::W], vec![]])
            ],])
        )
    }
}
