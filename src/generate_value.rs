use rand::Rng;

pub fn get_value() -> u8 {   
   return rand::thread_rng().gen_range(0..100);
}