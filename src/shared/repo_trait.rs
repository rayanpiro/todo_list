use std::error::Error;

pub trait Repo<T, Y> 
    where T: Sized,
          Y: Sized {
              
    fn create(&mut self, object: &T) -> Result<T, Box<dyn Error>>;
    fn update(&mut self, id: &Y) -> Result<T, Box<dyn Error>>;
    fn remove(&mut self, id: &Y) -> Result<(), Box<dyn Error>>;
    fn fetch(self, id: &Y) -> Result<T, Box<dyn Error>>;
    fn fetch_all(self) -> Result<Vec<T>, Box<dyn Error>>;
}