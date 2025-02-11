use bevy::math::UVec2;

pub struct NDim<T> {
    data: Vec<T>,
    dims: UVec2,
}

impl<T: Clone> NDim<T> {
    pub fn new(dims: UVec2, init_val: T) -> Self {
        let mut data = Vec::new();
        data.resize((dims.x * dims.y) as usize, init_val);

        Self { data, dims }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let pos = x + (y * self.dims.y as usize);
        self.data.get(pos)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let pos = x + (y * self.dims.y as usize);
        self.data.get_mut(pos)
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        let pos = x + (y * self.dims.y as usize);
        self.data[pos] = val;
    }

    pub fn dims(&self) -> UVec2 {
        self.dims
    }
}
