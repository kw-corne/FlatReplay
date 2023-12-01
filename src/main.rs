use std::path::Path;

mod flat_replay;
use flat_replay::FlatReplay;

fn main() {
    let replay_file = Path::new("ballchasing.replay");
    let _ = FlatReplay::try_from(replay_file).unwrap();
}
