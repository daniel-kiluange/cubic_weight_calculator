pub trait Cubing{
    fn volume(&self) -> f32;
    fn cubic_weight(&self) -> f32;
    fn pack_weight(&self) -> f32;
}