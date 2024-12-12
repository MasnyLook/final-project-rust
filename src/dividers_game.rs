use crate::game_abstract_class::Game;

pub struct GameDividers;

impl Game for GameDividers {
    fn compute_function(&self, input_value: u32, secret_value: u32) -> u32 {
        _dividers(input_value, secret_value)
    }

    fn result_string(&self) -> String {
        "number_of_dividers(x + secret_value)".to_string()
    }

    fn click_button(&self) -> String {
        "dividers".to_string()
    }
}

fn _dividers(x: u32, y: u32) -> u32 {
    let value = x + y;
    let mut counter = 0;
    let mut i = 1;

    while i * i <= value {
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
