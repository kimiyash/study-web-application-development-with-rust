use std::u32;

use mockall::predicate::*;
use mockall::*;

#[automock]
trait example_trait {
    fn example_method(&self, x: u32) -> u32;
}

fn example_func(
    x: &dyn example_trait, v: u32
) -> u32 {
    x.example_method(v)
}

fn main() {
    let mut mock = Mockexample_trait::new();
    mock.expect_example_method().returning(|x| x + 1);
    assert_eq!(10, example_func(&mock, 9));
}