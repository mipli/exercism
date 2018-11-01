pub struct RailFence {
    rails: u32
}
impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rows = (0..self.rails).map(|_| String::new()).collect::<Vec<_>>();
        text
            .chars()
            .zip(Pendulum::new(0, (self.rails - 1) as isize))
            .for_each(|(c, i)| {
                rows[i as usize].push(c)
            });
        rows.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let len = cipher.chars().count();
        let mut row_lengths = vec![0; self.rails as usize];
        Pendulum::new(0, (self.rails - 1) as isize)
            .take(len)
            .for_each(|i| {
                row_lengths[i as usize] += 1;
            });
        let mut rows = self.split_into_rows(cipher, row_lengths);

        Pendulum::new(0, (self.rails - 1) as isize)
            .take(len)
            .fold(String::new(), |mut text, i| {
                text.push(rows[i as usize].pop().expect("rows should not be empty"));
                text
            })
    }

    fn split_into_rows(&self, cipher: &str, row_lengths: Vec<usize>) -> Vec<Vec<char>> {
        let mut rails = vec![];
        let mut prev = 0;
        for size in row_lengths {
            let rail = cipher[prev..(prev + size)]
                .chars()
                .rev()
                .collect::<Vec<_>>();
            prev += size;
            rails.push(rail);
        }

        rails
    }
}

struct Pendulum {
    val: isize,
    lower: isize,
    upper: isize,
    step: isize
}

impl Pendulum {
    fn new(lower: isize, upper: isize) -> Self {
        assert!(upper > lower);
        Pendulum {
            val: lower,
            lower,
            upper,
            step: 1
        }
    }
}

impl Iterator for Pendulum {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.val;

        self.val += self.step;
        if self.val == self.lower {
            self.step = 1;
        } else if self.val == self.upper {
            self.step = -1;
        }
        Some(val)
    }
}
