#![warn(clippy::all, clippy::pedantic)]

mod command;
mod control;
mod hooks;
mod rules;
pub mod scheduler;
mod task;
#[cfg(test)]
mod tests;
