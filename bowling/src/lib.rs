#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Throw {
    Normal,
    Spare,
    Strike,
}

#[derive(Clone, Debug, PartialEq)]
struct Frame {
    first: Option<u16>,
    second: Option<u16>,
    throw: Throw,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            first: None,
            second: None,
            throw: Throw::Normal,
        }
    }
}

impl Frame {
    fn first(&self) -> u16 {
        self.first.unwrap_or(0)
    }

    fn second(&self) -> u16 {
        self.second.unwrap_or(0)
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: Option<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            current_frame: Some(Frame::default()),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let mut current = self.current_frame.take().ok_or_else(|| Error::GameComplete)?;

        if (current.first() + pins > 10 && current.throw != Throw::Strike) || pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if current.first.is_some() {
            current.second = Some(pins);
        } else {
            current.first = Some(pins);
        }

        if current.first() + current.second() == 10 {
            current.throw = if current.second.is_none() {
                Throw::Strike
            } else {
                Throw::Spare
            };
        }

        // special handling of strikes in the bonus frame, since we allow both the first and second
        // throw to be performed irregardles of the outcome of the first throw
        if current.second.is_some() || (current.throw == Throw::Strike && self.frames.len() < 10) {
            self.frames.push(current.clone());
            current = Frame::default();
        }

        if self.frames.len() < 10 {
            self.current_frame = Some(current);
        } else if self.frames.len() == 10 {
            if self.frames[9].throw == Throw::Spare {
                if current.first.is_some() {
                    self.frames.push(current.clone());
                } else {
                    self.current_frame = Some(current);
                }
            } else if self.frames[9].throw == Throw::Strike && current.second.is_none() {
                self.current_frame = Some(current);
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.current_frame.is_some() {
            return None;
        }
        let bonus = if self.frames.len() == 11 {
            (self.frames[10].first(), self.frames[10].second())
        } else {
            (0, 0)
        };
        let (score, _) = self.frames[0..10]
            .iter()
            .rev()
            .fold((0, bonus), |(score, bonus), frame| {
                let frame_score = frame.first() + frame.second();
                match frame.throw {
                    Throw::Normal => {
                        (score + frame_score, (frame.first(), frame.second()))
                    },
                    Throw::Spare => {
                        (score + frame_score + bonus.0, (frame.first(), frame.second()))
                    },
                    Throw::Strike => {
                        (score + frame_score + bonus.0 + bonus.1, (10, bonus.0))
                    },
                }
            });
        Some(score)
    }
}
