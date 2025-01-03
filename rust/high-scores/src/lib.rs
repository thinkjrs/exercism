#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores {
            scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result = self.scores.iter().copied().collect::<Vec<u32>>();
        result.sort_by(|a, b| b.cmp(a));
        result.truncate(3);
        result
    }
}
