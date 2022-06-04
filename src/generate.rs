use indicatif::ProgressBar;

pub fn generate_progress_bar(temperature: u64, max: u64) {
  let pb = ProgressBar::new(max);

  pb.set_position(temperature);

  pb.finish_at_current_pos();
}