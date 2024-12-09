extern crate websys;
use websys::game_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_function() {
        assert_eq!(game_utils::compute_function(54, 24), 6);
        assert_eq!(game_utils::compute_function(48, 18), 6);
        assert_eq!(game_utils::compute_function(101, 103), 1);
    }

    #[test]
    fn test_increase_number_of_attempts() {
        assert_eq!(game_utils::increase_number_of_attempts("number of attempts: 0"), "number of attempts: 1");
        assert_eq!(game_utils::increase_number_of_attempts("number of attempts: 1"), "number of attempts: 2");
        assert_eq!(game_utils::increase_number_of_attempts("number of attempts: 9"), "number of attempts: 10");
    }

    #[test]
    fn test_get_number_of_seconds() {
        assert_eq!(game_utils::get_number_of_seconds("Time: 0 seconds"), 0);
        assert_eq!(game_utils::get_number_of_seconds("Time: 42 seconds"), 42);
        assert_eq!(game_utils::get_number_of_seconds("Time: 100 seconds"), 100);
    }
}