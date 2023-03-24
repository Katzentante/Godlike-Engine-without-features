pub trait Vector {
    fn multiply(&mut self, factor: f32);
    fn normalize(&mut self) {
        self.stretch(1.0);
    }
    fn stretch(&mut self, desired_len: f32) {
        let r = desired_len / self.len();
        self.multiply(r);
    }

    fn len(&self) -> f32;
    fn cross_angle(&self, other: &Self) -> f32;
    fn dot_product(&self, other: &Self) -> f32;
}

