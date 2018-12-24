fn main() {
    part_a();
}

fn part_a() {
    let depth = 10914;
    let target = Pos {x: 9, y: 739};
    let mut state = State::new(depth, target);
    state.update_all();
    let score = state.score_to_target();
    println!("score is {}", score);
}

const X_MUL: usize = 16807;
const Y_MUL: usize = 48271;
const MOD: usize = 20183;
const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

struct State {
    depth: usize,
    current: Vec<Vec<Option<usize>>>,
    target: Pos
}

impl State {
    fn elevel(&mut self, pos: &Pos) -> usize {
        let val = (pos.gidx(self) + self.depth) % MOD;
        self.current[pos.y][pos.x] = Some(val);
        return val;
    }
    fn new(depth: usize, target: Pos) -> State {
        State {
            depth,
            current: vec![vec![None; target.x + 1]; target.y + 1],
            target
        }
    }
    fn update_all(&mut self) {
        for y in 0..=self.target.y{
            for x in 0..=self.target.x {
                let p = Pos{x, y};
                self.elevel(&p);
            }
        }
    }
    fn score_to_target(&self) -> usize {
        let mut s = 0;
        for y in self.current.iter() {
            for spot in y {
                let n = match spot.expect("update before scoring") % 3 {
                    ROCKY => 0,
                    WET => 1,
                    NARROW => 2,
                    _ => 0
                };
                s += n;
            }
        }
        s
    }
}

struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn gidx(&self, state: &mut State) -> usize {
        if (self.x == 0 && self.y == 0) ||
            (self.x == state.target.x && self.y == state.target.y) {
            0
        } else if self.y == 0 {
            self.x * X_MUL
        } else if self.x == 0 {
            self.y * Y_MUL
        } else {
            let a = match state.current[self.y][self.x - 1] {
                Some(val) => val,
                None => {
                    let a = Pos{x: self.x -1, y: self.y};
                    let a = state.elevel(&a);
                    a
                }
            };
            let b = match state.current[self.y - 1][self.x] {
                Some(val) => val,
                None => {
                    let b = Pos{x: self.x, y: self.y - 1};
                    let b = state.elevel(&b);
                    b
                }
            };
            a * b
        }
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        let mut st = String::with_capacity(2 * self.depth * self.depth);
        for (yi, row) in self.current.iter().enumerate() {
            for (xi, col) in row.iter().enumerate() {
                let c = match col {
                    Some(v) =>  match v % 3 {
                            ROCKY => '.',
                            WET => '=',
                            NARROW => '|',
                            _ => '?'
                        },
                    None =>  '?' 
                };
                if yi == 0 && xi == 0 {
                    st.push('M');
                } else if xi == self.target.x && yi == self.target.y {
                    st.push('T');
                } else {
                    st.push(c);
                }
            }
            st.push('\n');
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut state = State::new(510, Pos{x: 10, y: 10});
        assert_eq!(state.elevel(&Pos{x: 0, y: 0}), 510);
        assert_eq!(state.elevel(&Pos{x: 1, y: 0}), 17317);
        assert_eq!(state.elevel(&Pos{x: 0, y: 1}), 8415);
        assert_eq!(state.elevel(&Pos{x: 1, y: 1}), 1805);
    }
    #[test]
    fn test_example() { 
        let mut state = State::new(510, Pos{x: 10, y: 10});
        state.update_all();
        let s = state.score_to_target();
        assert_eq!(s, 114);
    }
}