#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self {
                rank,
                file,
            }),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition { rank: a_rank, file: a_file } = self.position;
        let ChessPosition { rank: b_rank, file: b_file } = other.position;

        a_rank == b_rank ||
            a_file == b_file ||
            (a_rank - b_rank).abs() == (a_file - b_file).abs()
    }
}
