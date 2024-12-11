pub trait Game {
    fn compute_function (&self, input_value : u32, secret_value : u32) -> u32;
}
