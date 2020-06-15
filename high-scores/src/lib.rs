use std::cmp;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|s| *s)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|s| *s)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut copy = self.scores.clone();
        copy.sort_by(|a,b| b.cmp(a));
        copy[..cmp::min(3, copy.len())].to_vec()
    }
}
