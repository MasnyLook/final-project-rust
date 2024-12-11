use crate::game_abstract_class::Game;

pub struct Game_dividers;

impl Game for Game_dividers {
    fn compute_function (&self, input_value : u32, secret_value : u32) -> u32 {
        _dividers(input_value, secret_value)
    }
}

fn _dividers(x : u32, y : u32) -> u32 {
    let value = x + y;
    let mut counter = 0;
    let mut i = 1;

    while i <= value {
        if value % i == 0 {
            if i * i == value {
                counter += 1;
            } else {
                counter += 2;
            }
        }
        i += 1;
    }

    counter
}