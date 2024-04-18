//! I built this crate to practice lifetimes, a concept most beginner Rust programmers find difficult to grasp. I followed through
//! this tutorial, [here](https://www.youtube.com/watch?v=rAl-9HwD858&t=70s), by Jon Gjengset. What we are implementing is already in the
//! standard library, but what's a better way to learn lifetimes than by building something with them?

mod strsplit;

pub use strsplit::until_char;
pub use strsplit::*;
