pub mod newsarticle;
pub mod tweet;

pub trait Summary {
    fn summarize(&self) -> String;
}
