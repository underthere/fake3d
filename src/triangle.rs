pub(crate) struct Triangle {
    vertices: [nalgebra::Vector3<f64>; 3],
    color: [nalgebra::Vector3<f64>; 3],
    tex_coords: [nalgebra::Vector2<f64>; 3],
    normal: [nalgebra::Vector3<f64>; 3],
}

impl Default for Triangle {
    fn default() -> Self {
        Self { vertices: Default::default(), color: Default::default(), tex_coords: Default::default(), normal: Default::default() }
    }
}

impl Triangle {
    pub fn a(&self) -> nalgebra::Vector3<f64> {
        self.vertices[0]
    }
    pub fn b(&self) -> nalgebra::Vector3<f64> {
        self.vertices[1]
    }
    pub fn c(&self) -> nalgebra::Vector3<f64> {
        self.vertices[2]
    }

    pub fn set_vertex(&mut self, index: usize, vertex: &nalgebra::Vector3<f64>) {
        self.vertices[index] = vertex.clone();
    }

    pub fn set_normal(&mut self, index: usize, normal: &nalgebra::Vector3<f64>) {
        self.normal[index] = normal.clone();
    }

    pub fn set_color(&mut self, index: usize, color: &nalgebra::Vector3<f64>) {
        self.color[index] = color.clone();
    }

    pub fn set_tex_coords(&mut self, index: usize, tex_coords: &nalgebra::Vector2<f64>) {
        self.tex_coords[index] = *tex_coords;
    }

    pub fn to_vector4(&self) -> [nalgebra::Vector4<f64>; 3] {
        self.vertices
            .iter()
            .map(|v| nalgebra::Vector4::new(v.x, v.y, v.z, 1.0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
