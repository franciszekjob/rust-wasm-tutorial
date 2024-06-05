use wasm_bindgen::prelude::*;

extern crate js_sys;

#[wasm_bindgen]
pub struct Shuffler {
    numbers: Vec<u32>,
    size: u32,
}

#[wasm_bindgen]
impl Shuffler {
    pub fn new(size: u32) -> Shuffler {
        let numbers: Vec<u32> = (0..size).collect();
        Shuffler { numbers, size }
    }

    pub fn add(&mut self, number: u32) {
        self.numbers.push(number);
        self.size += 1;
    }

    pub fn shuffle(&mut self) {
        for _ in 0..self.size {
            js_sys::Math::random() * self.size
        }
    }

    fn randint(min: u32, max: u32) -> u32 {
        js_sys::Math::floor(js_sys::Math::random() * (max - min) + min) as u32
    }
}
