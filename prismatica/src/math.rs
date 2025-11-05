pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0]
        }
    }
    
    pub fn to_array(&self) -> [f32; 16] {
        self.data.clone()
    }
}