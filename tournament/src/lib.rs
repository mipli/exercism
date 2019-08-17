use std::str::FromStr;
use std::fmt;
use std::iter::once;
use std::collections::{HashMap};
use std::cmp::Ordering;

type Error = Box<std::error::Error>;

#[derive(Debug, Eq, PartialEq)]
enum Score {
    Home,
    Draw,
    Away,
}

#[derive(Debug)]
struct Match {
    home: String,
    away: String,
    result: Score,
}

impl<'a> FromStr for Match {
    type Err = Error;

    fn from_str(input: &str) -> Result<Match, Error> {
        if let [home, away, res] = input.trim().split(';').collect::<Vec<_>>()[..] {
            let result = match res {
                "win" => Score::Home,
                "loss" => Score::Away,
                "draw" => Score::Draw,
                _ => return Err(Box::<std::error::Error>::from("Invalid result information"))
            };

            Ok(Match {
                home: home.to_string(),
                away: away.to_string(),
                result,
            })
        } else {
            Err(Box::<std::error::Error>::from("Invalid match information"))
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
struct Row {
    team: String,
    matches: u16,
    wins: u16,
    draws: u16,
    losses: u16,
    points: u16,
}

impl Row {
    fn new(name: &str) -> Self {
        Row { 
            team: name.to_string(),
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }

    fn loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Row) -> Ordering {
        self.points.cmp(&other.points).reverse().then(self.team.cmp(&other.team))
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Row) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", self.team, self.matches, self.wins, self.draws, self.losses, self.points)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<String, Row> = HashMap::default();

    match_results
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|m: Match| {
            match m.result {
                Score::Home => {
                    table
                        .entry(m.home.to_string())
                        .or_insert_with(|| Row::new(&m.home))
                        .win();

                    table
                        .entry(m.away.to_string())
                        .or_insert_with(|| Row::new(&m.away))
                        .loss();
                },
                Score::Away => {
                    table
                        .entry(m.home.to_string())
                        .or_insert_with(|| Row::new(&m.home))
                        .loss();

                    table
                        .entry(m.away.to_string())
                        .or_insert_with(|| Row::new(&m.away))
                        .win();
                }
                Score::Draw => {
                    table
                        .entry(m.home.to_string())
                        .or_insert_with(|| Row::new(&m.home))
                        .draw();

                    table
                        .entry(m.away.to_string())
                        .or_insert_with(|| Row::new(&m.away))
                        .draw();
                }
            }
        });

    let mut rows = table
        .values()
        .collect::<Vec<&Row>>();
    rows.sort();

    let header = "Team                           | MP |  W |  D |  L |  P";

    once(header.to_string())
        .chain(rows.iter().map(|r| r.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
