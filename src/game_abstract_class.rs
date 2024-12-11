pub trait Game {
    fn compute_function (&self, input_value : u32, secret_value : u32) -> u32;
    fn result_string(&self) -> String;
    fn click_button(&self) -> String;
}
