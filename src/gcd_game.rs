use crate::game_abstract_class::Game;

pub struct Game_gcd;

impl Game for Game_gcd {
    fn compute_function (&self, input_value : u32, secret_value : u32) -> u32 {
        _gcd(input_value, secret_value)
    }
}

fn _gcd(x : u32, y : u32) -> u32 {
    if x < y {
        return _gcd(y, x);
    }
    if y == 0 {
        return x;
    }
    _gcd(y, x % y)
}