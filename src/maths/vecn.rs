pub struct VecN<D> {
    vals: [f32; D],
}

impl VecN {
    pub fn len(&self) -> f32 {
        self.vals.iter().map(|x| x*x).sum::<f32>().sqrt()
    }
    
    pub fn 
}
