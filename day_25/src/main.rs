/**
 * The list of fixed points in spacetime (your puzzle input) is a set of
 * four-dimensional coordinates. To align your device, acquire the hot
 * chocolate, and save the reindeer, you just need to find the number of
 * constellations of points in the list.
 *
 * Constellation: two pionts not more than 3 points apart
 * 
 *  This is similar to a [DBSCAN](https://en.wikipedia.org/wiki/DBSCAN) algorithm I think 
 * 
 */

const MAX_DIST: usize = 3;

fn main() {
}

struct State {
    stars: Vec<Pos>
}

impl std::str::FromStr for State {
    type Err = Box<dyn std::error::Error>;

    /// input looks like 4 ordered pairs
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stars = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(State{stars})
    }
}


#[derive(Copy, Clone, Debug)]
struct Pos(i32, i32, i32, i32);

impl Pos {
    fn dist(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

impl std::str::FromStr for Pos {
    type Err = Box<dyn std::error::Error>;

    /// input looks like 4 ordered pairs
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("st is {}", s);
        let mut parts = s.split(",");
        Ok (
            Pos ( 
                parts.next().unwrap().trim().parse().unwrap(),
                parts.next().unwrap().trim().parse().unwrap(),
                parts.next().unwrap().trim().parse().unwrap(),
                parts.next().unwrap().trim().parse().unwrap(),
            )
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let st = include_str!("../test.txt");
        let state: State = st.parse().unwrap();
    }
}
