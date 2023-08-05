use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use crate::vectors::Vector3;
use crate::matrices::Matrix3;


pub struct Drawable {
    vertices: Vec<Vector3>,
    indices: Vec<(usize, usize)>,
    rotation: Matrix3,
    origin: Vector3,

    children: Vec<Drawable>
}


impl Drawable {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<(usize, usize)>) -> Self {
        Drawable { vertices, indices, rotation: Matrix3::identity(), origin: Vector3::new(0.0, 0.0, 0.0), children: vec![] }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, size: f64) {
        let projected: Vec<Vector3> = self.vertices.iter()
            .map(|point| &Matrix3::identity() * &(&(&self.rotation * &(point * size)) + &self.origin)).collect();

        self.draw_from_projected(canvas, projected, size);
    }

    fn draw_rec(&mut self, canvas: &mut Canvas<Window>, parent_rotation: Matrix3, parent_origin: Vector3, size: f64) {
        let projected: Vec<Vector3> = self.vertices.iter()
            .map(|point| -> Vector3 {
                let checkpoint = &(&parent_rotation * &(&(point * size) + &self.origin)) - &self.origin;
                let checkpoint = &(&(&self.rotation * &checkpoint) + &self.origin) + &parent_origin;
                &Matrix3::identity() * &checkpoint
            }).collect();


        self.draw_from_projected(canvas, projected, size);
    }

    fn draw_from_projected(&mut self, canvas: &mut Canvas<Window>, projected: Vec<Vector3>, size: f64) {
        let corner_square_size = 10u32;
        for point in projected.iter() {
            let x = point.x as i32 - (corner_square_size / 2) as i32;
            let y = point.y as i32 - (corner_square_size / 2) as i32;
            canvas.fill_rect(Rect::new(x, y, corner_square_size, corner_square_size)).unwrap();
        }

        for (i, j) in self.indices.iter() {
            let p1 = &projected[*i];
            let p2 = &projected[*j];

            let x1 = p1.x as i32;
            let y1 = p1.y as i32;
            
            let x2 = p2.x as i32;
            let y2 = p2.y as i32;

            canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2)).unwrap();
        }

        for child in self.children.iter_mut() {
            child.draw_rec(canvas, self.rotation.clone(), self.origin.clone(), size / 2.0);
        }
    }

    pub fn set_rotation(&mut self, rotation: Matrix3) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn set_origin(&mut self, origin: Vector3) -> &mut Self {
        self.origin = origin;
        self
    }

    pub fn add_child(&mut self, child: Drawable) -> &mut Self {
        self.children.push(child);
        self
    }
}
