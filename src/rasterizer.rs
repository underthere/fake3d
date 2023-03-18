use nalgebra as ng;
use std::collections::HashMap;
use bitflags::bitflags;
use crate::triangle::Triangle;

bitflags! {
    pub struct Buffers: u8 {
        const Color = 0b00000001;
        const Depth = 0b00000010;
    }
}


pub enum Primitive {
    Line,
    Triangle
}

pub struct VertexBufferId(u32);
pub struct IndexBufferId(u32);

pub(crate) struct Rasterizer {
    model: ng::Matrix4<f64>,
    view: ng::Matrix4<f64>,
    projection: ng::Matrix4<f64>,

    vertices: HashMap<u32, Vec<ng::Vector3<f64>>>,
    indices: HashMap<u32, Vec<ng::Vector3<u32>>>,
    
    pub frame_buffer: Vec<ng::Vector3<u8>>,
    depth_buffer: Vec<f64>,

    width: u32,
    height: u32,

    next_id: u32,

}



fn to_vector4(v: &ng::Vector3<f64>, w: f64) -> ng::Vector4<f64> {
    ng::Vector4::new(v.x, v.y, v.z, w)
}


impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width, height,
            model: ng::Matrix4::identity(),
            view: ng::Matrix4::identity(),
            projection: ng::Matrix4::identity(),
            vertices: HashMap::new(),
            indices: HashMap::new(),
            frame_buffer: vec![ng::Vector3::new(0, 0, 0); (width * height) as usize],
            depth_buffer: vec![0.0; (width * height) as usize],
            next_id: 0,
        }
    }

    pub fn load_vertices(&mut self, vertex: &Vec<ng::Vector3<f64>>) -> VertexBufferId {
        let id = self.get_next_id();
        self.vertices.insert(id, vertex.clone());
        VertexBufferId(id)
    }

    pub fn load_indices(&mut self, index: &Vec<ng::Vector3<u32>>) -> IndexBufferId {
        let id = self.get_next_id();
        self.indices.insert(id, index.clone());
        IndexBufferId(id)
    }

    pub fn set_model(&mut self, model: ng::Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: ng::Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: ng::Matrix4<f64>) {
        self.projection = projection;
    }

    pub fn set_pixel(&mut self, point: ng::Vector3<f64>, color: ng::Vector3<u8>) {
        let x = point.x as u32;
        let y = point.y as u32;
        if x >= self.width || y >= self.height {
            return;
        }
        let index = self.get_index(x, y);
        self.frame_buffer[index] = color;
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::Color) {
            self.frame_buffer = vec![ng::Vector3::new(0, 0, 0); (self.width * self.height) as usize];
        }
        if buffers.contains(Buffers::Depth) {
            self.depth_buffer = vec![0.0; (self.width * self.height) as usize];
        }
    }

    pub fn draw(&mut self, vert_buf: &VertexBufferId, ind_buf: &IndexBufferId, primitive: Primitive) {
        match primitive {
            Primitive::Triangle => self.draw_triangle(vert_buf, ind_buf),
            _ => unimplemented!(),
        }
    }

    fn draw_triangle(&mut self, vert_buf: &VertexBufferId, ind_buf: &IndexBufferId) {
        let vertices = self.vertices.get(&vert_buf.0).unwrap().clone();
        let indices = self.indices.get(&ind_buf.0).unwrap().clone();

        let f1: f64 = (100.0 - 0.1) / 2.0;
        let f2: f64 = (100.0 + 0.1) / 2.0;
        let mvp = self.projection * self.view * self.model;

        for i in indices {
            let mut triangle = Triangle::default();
            let mut v = [
                mvp * to_vector4(&vertices[i.x as usize], 1.0),
                mvp * to_vector4(&vertices[i.y as usize], 1.0),
                mvp * to_vector4(&vertices[i.z as usize], 1.0),
            ];

            for _v in v.as_mut() {
                *_v = *_v / _v.w;
            }

            for _v in v.as_mut() {
                _v.x = 0.5 * self.width as f64 * (_v.x + 1.0);
                _v.y = 0.5 * self.height as f64 * (_v.y + 1.0);
                _v.z = f1 + f2;
            }

            for (i, _v) in v.iter().enumerate() {
                triangle.set_vertex(i, &ng::Vector3::new(_v.x, _v.y, _v.z));
            }

            triangle.set_color(0, &ng::Vector3::new(255.0, 0.0, 0.0));
            triangle.set_color(1, &ng::Vector3::new(0.0, 255.0, 0.0));
            triangle.set_color(2, &ng::Vector3::new(0.0, 0.0, 255.0));

            self.rasterize_wireframe(&triangle);
        }
    }

    fn draw_line(&mut self, begin: ng::Vector3<f64>, end: ng::Vector3<f64>) {
        let x1 = begin.x;
        let y1 = begin.y;
        let x2 = end.x;
        let y2 = end.y;

        let dx = (x2 - x1) as i32;
        let dy = (y2 - y1) as i32;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        let (mut x, mut y, xe, ye): (i32, i32, i32, i32);

        let color = ng::Vector3::new(255, 0, 0);

        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                xe = x2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                xe = x1 as i32;
            }
            let mut point = ng::Vector3::new(x as f64, y as f64, 1.0);
            self.set_pixel(point, color);

            while x < xe {
                x = x + 1;
                if px < 0 {
                    px = px + 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y = y + 1;
                    } else {
                        y = y - 1;
                    }
                    px = px + 2 * (dy1 - dx1);
                }
                point = ng::Vector3::new(x as f64, y as f64, 1.0);
                self.set_pixel(point, color);
            }
        } else {
            if dy >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                ye = y2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                ye = y1 as i32;
            }

            let mut point = ng::Vector3::new(x as f64, y as f64, 1.0);
            self.set_pixel(point, color);

            while y < ye {
                y = y + 1;

                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }
                    py = py + 2 * (dx1 - dy1);
                }

                point = ng::Vector3::new(x as f64, y as f64, 1.0);
                self.set_pixel(point, color);
            }
        }
        
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(t.c(), t.a());
        self.draw_line(t.c(), t.b());
        self.draw_line(t.b(), t.a());
    }

    fn get_index(&mut self, x: u32, y: u32) -> usize {
        ((self.height - y) * self.width + x) as usize
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn as_raw_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute(self.frame_buffer.as_ptr()),
                self.frame_buffer.len() * 3)
        }
    }


}