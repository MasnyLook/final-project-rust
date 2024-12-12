extern crate websys;
use crate::websys::game_abstract_class::Game;
use websys::dividers_game::GameDividers;
use websys::game_utils;
use websys::gcd_game::GameGcd;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_compute_function() {
        assert_eq!(GameGcd.compute_function(54, 24), 6);
        assert_eq!(GameGcd.compute_function(48, 18), 6);
        assert_eq!(GameGcd.compute_function(101, 103), 1);
    }

    #[test]
    fn test_dividers_compute_function() {
        assert_eq!(GameDividers.compute_function(100, 0), 9); // 100 + 0 = 100, divisors: 1, 2, 4, 5, 10, 20, 25, 50, 100
        assert_eq!(GameDividers.compute_function(48, 18), 8); // 48 + 18 = 66, divisors: 1, 2, 3, 6, 11, 22, 33, 66
        assert_eq!(GameDividers.compute_function(101, 103), 12); // 101 + 103 = 204, divisors: 1, 2, 3, 4, 6, 12, 17, 34, 51, 68, 102, 204
    }

    #[test]
    fn test_increase_number_of_attempts() {
        assert_eq!(
            game_utils::increase_number_of_attempts("number of attempts: 0"),
            "number of attempts: 1"
        );
        assert_eq!(
            game_utils::increase_number_of_attempts("number of attempts: 1"),
            "number of attempts: 2"
        );
        assert_eq!(
            game_utils::increase_number_of_attempts("number of attempts: 9"),
            "number of attempts: 10"
        );
    }

    #[test]
    fn test_get_number_of_seconds() {
        assert_eq!(game_utils::get_number_of_seconds("Time: 0 seconds"), 0);
        assert_eq!(game_utils::get_number_of_seconds("Time: 42 seconds"), 42);
        assert_eq!(game_utils::get_number_of_seconds("Time: 100 seconds"), 100);
    }
}
