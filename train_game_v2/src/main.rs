pub mod multithread;
pub mod calcs;

use crate::multithread::solve;

fn main() {
    solve("1234".to_owned());
}