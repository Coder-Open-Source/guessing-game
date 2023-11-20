use rand::Rng;

pub fn get_value() -> u8 {   
   rand::thread_rng().gen_range(0..100)
}