extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;

trait Cubing{
    fn volume(&self) -> f32;
    fn cubic_weight(&self) -> f32;
    fn pack_weight(&self) -> f32;
}

struct Parcel{
    weight: f32,
    height: f32,
    width: f32,
    lenght: f32,
    quantity: u16,
}

impl Cubing for Parcel{
    fn volume(&self) -> f32 { 
        self.quantity as f32 * (self.width * self.height * self.lenght)
    }

    fn cubic_weight(&self) -> f32 {
        self.volume() * dotenv!("BASE_CUBIC_FACTOR").parse::<f32>().unwrap()
    }

    fn pack_weight(&self) -> f32 {
        self.weight * self.quantity as f32
    }
}

fn real_weight(list:&Vec<Parcel>) -> f32 {
    list.iter()
    .map(Parcel::pack_weight)
    .reduce(|a,b| a + b).unwrap()
}

fn cubic_weight(list:&Vec<Parcel>) -> f32 {
    list.iter()
    .map(Parcel::cubic_weight)
    .reduce(|a,b|a+ b).unwrap()
}

fn main() {

    dotenv().ok();

    let x = vec![Parcel{weight:266.67, height:1.2, width:1.2, lenght:1.5, quantity:3}, Parcel{weight:1000.0, height:2.0, width:1.5, lenght:0.8, quantity:1}];

    let rw = real_weight(&x);

    let cw = cubic_weight(&x);

    println!("Real weight: {} Kg\nCubic weight: {} Kg", rw, cw);

    println!("weight to collect: {} Kg", if rw > cw {rw} else {cw});
    
}
