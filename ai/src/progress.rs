use std::fmt::Write;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::configs::NUM_BOTS;

#[derive(Debug, Clone)]
pub struct Pbar(ProgressBar);
impl Pbar {
    pub fn new() -> Self {
        let pb = ProgressBar::new(NUM_BOTS as u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
        Self(pb)
    }
    pub fn inc(&self, n: u64) {
        self.0.inc(n);
    }

    pub fn finish(&self) {
        self.0.finish();
    }
}
