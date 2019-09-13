extern crate piston_window;
extern crate specs;

pub mod util;
pub mod math;
pub mod geometry;
pub mod ui;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
