use shared::read_file;

enum Chose {
    Rock,
    Paper,
    Scissors,
}

impl Chose {
    fn to_score(&self) -> usize {
        match self {
            Chose::Rock => 1,
            Chose::Paper => 2,
            Chose::Scissors => 3,
        }
    }
}

struct Round {
    opponent_chose: Chose,
    mine_chose: Chose,
}

impl Round {
    fn outcome_score(&self) -> (usize, usize) {
        match (&self.opponent_chose, &self.mine_chose) {
            (Chose::Rock, Chose::Rock) => (3, 3),
            (Chose::Rock, Chose::Paper) => (0, 6),
            (Chose::Rock, Chose::Scissors) => (6, 0),
            (Chose::Paper, Chose::Rock) => (6, 0),
            (Chose::Paper, Chose::Paper) => (3, 3),
            (Chose::Paper, Chose::Scissors) => (0, 6),
            (Chose::Scissors, Chose::Rock) => (0, 6),
            (Chose::Scissors, Chose::Paper) => (6, 0),
            (Chose::Scissors, Chose::Scissors) => (3, 3),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Game {
    mine_score: usize,
    opponent_score: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            mine_score: 0,
            opponent_score: 0,
        }
    }

    fn play_round(&mut self, round: &Round) {
        let opponent_score = round.opponent_chose.to_score();
        let mine_score = round.mine_chose.to_score();
        let (opponent_win_score, mine_win_score) = round.outcome_score();
        self.opponent_score += opponent_score + opponent_win_score;
        self.mine_score += mine_score + mine_win_score;
    }
}

fn main() {
    let content = read_file();
    // day 2 - 1
    play_strategy(&content, parse_strategy1);
    // day 2 - 2
    play_strategy(&content, parse_strategy2);
}

fn play_strategy(content: &String, parse_round: fn(&str) -> Option<Round>) {
    let mut rounds = Vec::new();
    for line in content.lines() {
        if let Some(round) = parse_round(line) {
            rounds.push(round)
        }
    }
    let mut game = Game::new();
    rounds.iter().for_each(|round| game.play_round(round));
    println!("{:?}", game);
}

fn parse_strategy1(line: &str) -> Option<Round> {
    let mut split = line.split(" ");
    let opponent = split.next()?;
    let mine = split.next()?;
    let opponent_chose = match opponent {
        "A" => Chose::Rock,
        "B" => Chose::Paper,
        "C" => Chose::Scissors,
        _ => return None,
    };
    let mine_chose = match mine {
        "X" => Chose::Rock,
        "Y" => Chose::Paper,
        "Z" => Chose::Scissors,
        _ => return None,
    };
    Some(Round {
        opponent_chose,
        mine_chose,
    })
}

fn parse_strategy2(line: &str) -> Option<Round> {
    let mut split = line.split(" ");
    let opponent = split.next()?;
    let outcome = split.next()?;
    let opponent_chose = match opponent {
        "A" => Chose::Rock,
        "B" => Chose::Paper,
        "C" => Chose::Scissors,
        _ => return None,
    };
    let outcome = match outcome {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => return None,
    };
    let mine_chose = match (&opponent_chose, outcome) {
        (Chose::Rock, Outcome::Lose) => Chose::Scissors,
        (Chose::Rock, Outcome::Draw) => Chose::Rock,
        (Chose::Rock, Outcome::Win) => Chose::Paper,
        (Chose::Paper, Outcome::Lose) => Chose::Rock,
        (Chose::Paper, Outcome::Draw) => Chose::Paper,
        (Chose::Paper, Outcome::Win) => Chose::Scissors,
        (Chose::Scissors, Outcome::Lose) => Chose::Paper,
        (Chose::Scissors, Outcome::Draw) => Chose::Scissors,
        (Chose::Scissors, Outcome::Win) => Chose::Rock,
    };
    Some(Round {
        opponent_chose,
        mine_chose,
    })
}
