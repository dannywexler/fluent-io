use crate::file::{
    DEFAULT_PHASH_SIMILARITY_THRESHOLD, ImagePhash, VideoFile, VideoFileActionResult,
};

pub struct VideoPhash(Vec<ImagePhash>);

impl VideoPhash {
    pub fn distance_to(&self, other: &VideoPhash) -> Vec<usize> {
        let self_len = self.0.len();
        let other_len = other.0.len();
        if self_len != other_len {
            panic!(
                "Cannot compare 2 VideoPhashes generated with a different number of ImagePhashes! Self had {self_len} ImagePhashes and other had {other_len} ImagePhashes."
            )
        }
        self.0
            .iter()
            .zip(&other.0)
            .map(|(a, b)| a.distance_to(b))
            .collect::<Vec<_>>()
    }

    pub fn is_similar_to(&self, other: &VideoPhash) -> bool {
        self.is_similar_to_custom_threshold(other, DEFAULT_PHASH_SIMILARITY_THRESHOLD)
    }

    pub fn is_similar_to_custom_threshold(&self, other: &VideoPhash, threshold: usize) -> bool {
        self.distance_to(other)
            .iter()
            .all(|dist| dist <= &threshold)
    }
}

impl VideoFile {
    pub fn phashes(&self, phashes_count: u8) -> VideoFileActionResult<VideoPhash> {
        todo!()
    }
}
