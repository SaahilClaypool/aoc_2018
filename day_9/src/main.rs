use std::collections::HashSet;
use std::collections::LinkedList;

fn main() {
    let mut g = Game::new(466, 71436);
    let max_score = g.play_game();

    println!("Max score: {}", max_score);

    let mut g = Game::new(466, 71436 * 100);
    let max_score = g.play_game();

    println!("Max score: {}", max_score);
}

struct Game {
    players: Vec<u32>,
    total_marbles: u32,
    current_marble: u32,
    marbles: LinkedList<u32>,
    current_player: usize,
    last_placed: usize,
}

impl Game {
    fn new(num_players: u32, num_marbles: u32) -> Self {
        let mut players = vec![];
        for _ in 0..num_players {
            players.push(0)
        }
        // let mut marbles = Vec::with_capacity(num_marbles as usize);
        let mut marbles = LinkedList::new();
        marbles.push_back(0);

        Game {
            players: players,
            total_marbles: num_marbles,
            current_marble: 1,
            marbles: marbles,
            current_player: 1,
            last_placed: 0,
        }
    }

    fn get_index(&self) -> usize {
        ((self.last_placed + 1) as usize % self.active_marbles()) + 1
    }

    fn play_game(&mut self) -> u32 {
        for _ in 0..self.total_marbles {
            self.round();
        }
        self.highest_score()
    }

    fn round(&mut self) {
        if self.current_marble % 23 != 0 {
            let index = self.get_index();
            self.insert_marble(index);
            self.last_placed = index;
        } else {
            // keep marble. Add to score
            *self.players.get_mut(self.current_player).unwrap() += self.current_marble;
            // this is a modular ring. Just add the length so its never negative
            let remove_index = (self.last_placed + self.active_marbles() - 7) % self.active_marbles();
            *self.players.get_mut(self.current_player).unwrap() += self.remove_marble(remove_index);
            let new_index = remove_index % self.active_marbles();
            self.last_placed = new_index;
        }

        self.current_marble += 1;
        self.current_player += 1;
        self.current_player %= self.players.len();
    }

    fn insert_marble(&mut self, index: usize) {
        let mut at = self.marbles.split_off(index);
        at.push_front(self.current_marble);
        self.marbles.append(&mut at);
    }

    fn remove_marble(&mut self, remove_index: usize) -> u32 {
        let mut at = self.marbles.split_off(remove_index);
        let removed = at.pop_front().unwrap();
        self.marbles.append(&mut at);
        removed
    }

    fn active_marbles(&self) -> usize {
        self.marbles.len()
    }

    fn highest_score(&self) -> u32 {
        self.players.iter().map(|v| *v).max().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testind() {
        let g = Game::new(9, 25);
        let ind = g.get_index();
        assert_eq!(ind, 1);
    }

    #[test]
    fn testmarbles() {
        let mut g = Game::new(9, 25);
        g.round();
        assert_eq!(vec![0, 1], g.marbles.iter().cloned().collect::<Vec<u32>>());
        g.round();
        assert_eq!(vec![0, 2, 1], g.marbles.iter().cloned().collect::<Vec<u32>>());
        g.round();
        assert_eq!(vec![0, 2, 1, 3], g.marbles.iter().cloned().collect::<Vec<u32>>());
        g.round();
        assert_eq!(vec![0, 4, 2, 1, 3], g.marbles.iter().cloned().collect::<Vec<u32>>());
    }

    #[test]
    fn test_larger() {
        let mut g = Game::new(9, 25);
        for i in 0..23 {
            g.round();
        }
        assert_eq!(g.marbles.split_off(g.last_placed).pop_front().unwrap(), 19);
    }

    #[test]
    fn test_examples() {
        let mut g = Game::new(9, 25);
        assert_eq!(g.play_game(), 32);

        let mut g = Game::new(30, 5807);
        assert_eq!(g.play_game(), 37305)
    }
}