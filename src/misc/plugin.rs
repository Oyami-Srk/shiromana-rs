use super::*;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn trigger(&self) -> &'static str;
}