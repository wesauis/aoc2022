use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win  = 6,
}

impl Outcome {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _   => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Move {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3,
}

impl Move {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _   => None,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Self::Rock     => Self::Scissors,
            Self::Paper    => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Scissors => Self::Rock,
            Self::Rock     => Self::Paper,
            Self::Paper    => Self::Scissors,
        }
    }

    fn cmp(&self, opponent: &Self) -> Outcome {
        if self == opponent {
          return Outcome::Draw;
        }

        if &self.loses_against() == opponent {
          return Outcome::Lose;
        }

        Outcome::Win
    }

    fn move_for_outcome(&self, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Lose => self.wins_against(),
            Outcome::Draw => self.clone(),
            Outcome::Win  => self.loses_against(),
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1() {
    let rounds = read_lines("2.input").unwrap();

    let mut my_total_score = 0u32;

    for round in rounds {
        let round = round.expect("Failied to read line");
        let mut round = round.chars();

        let elf_move = Move::from_char(&round.nth(0).unwrap()).unwrap();
        let my_move  = Move::from_char(&round.nth(1).unwrap()).unwrap();

        let outcome = my_move.cmp(&elf_move);
        let score = (*&my_move as u8 as u32) + (*&outcome as u8 as u32);

        my_total_score += score;
    }

    println!("--- Day 2: Rock Paper Scissors ---");
    println!("Following the strategy guide my total score will be {my_total_score} points");
}

fn part_2() {
  let rounds = read_lines("2.input").unwrap();

  let mut my_total_score = 0u32;

  for round in rounds {
      let round = round.expect("Failied to read line");
      let mut round = round.chars();

      let elf_move = Move::from_char(&round.nth(0).unwrap()).unwrap();

      let desidered_outcome = Outcome::from_char(&round.nth(1).unwrap()).unwrap();
      let my_move = elf_move.move_for_outcome(&desidered_outcome);

      let outcome = my_move.cmp(&elf_move);
      let score = (*&my_move as u8 as u32) + (*&outcome as u8 as u32);

      my_total_score += score;
  }

  println!("\n--- Part Two ---");
  println!("Following the updated strategy guide my total score woll be {my_total_score} points");
}
