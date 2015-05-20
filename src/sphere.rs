extern crate glutin;

extern crate glium;

extern crate nalgebra as na;

use std::f32::consts::PI;
use na::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);


#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Sphere {
    pub mass: f32,
    pub radius: f32,
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub rotation: Rot3<f32>,
    pub angular_velocity: Vec3<f32>,
}

impl Sphere {
    pub fn new(radius: f32, mass: f32) -> Sphere {
        Sphere {
            mass: mass,
            radius: radius,
            position: na::zero(),
            velocity: na::zero(),
            rotation: Rot3::new(na::zero()),
            angular_velocity: na::zero(),
        }
    }

    // todo remove self from all methods
    // set size to 1
    fn get_points(&self, subdivs: u32) -> Vec<Vec3<f32>> {
        let mut points: Vec<Vec3<f32>> = vec![];
        let y_subdivs = subdivs;
        let x_subdivs = subdivs * 2;

        let y_angle_iter = (0..y_subdivs)
            .map(|a| a * (90 / y_subdivs) )
            .map(|a| a as f32 / 180.0f32 * PI );
        let next_y_angle_iter =(1..y_subdivs + 1)
            .map(|a| a * (90 / y_subdivs) )
            .map(|a| a as f32 / 180.0f32 * PI );

        for (y_angle, next_y_angle) in y_angle_iter.zip(next_y_angle_iter)
        {
            let y_dist = y_angle.cos() * self.radius;
            let xz_dist = y_angle.sin() * self.radius;

            let next_y_dist = next_y_angle.cos() * self.radius;
            let next_xz_dist = next_y_angle.sin() * self.radius;

            let xz_angle_iter = (0..x_subdivs)
                .map(|a| a * 180 / x_subdivs )
                .map(|a| a as f32 / 180.0 * PI );
            let next_xz_angle_iter = (1..x_subdivs + 1)
                .map(|a| a * 180 / x_subdivs )
                .map(|a| a as f32 / 180.0 * PI );

            for (xz_angle, next_xz_angle) in xz_angle_iter.zip(next_xz_angle_iter)
            {
                let tl_x_dist = xz_angle.cos() * xz_dist;
                let tl_z_dist = xz_angle.sin() * xz_dist;

                let tr_x_dist = next_xz_angle.cos() * xz_dist;
                let tr_z_dist = next_xz_angle.sin() * xz_dist;

                let bl_x_dist = xz_angle.cos() * next_xz_dist;
                let bl_z_dist = xz_angle.sin() * next_xz_dist;

                let br_x_dist = next_xz_angle.cos() * next_xz_dist;
                let br_z_dist = next_xz_angle.sin() * next_xz_dist;

                let mut make_face = |tl_x, t_y, tl_z, tr_x, tr_z, bl_x, b_y, bl_z, br_x, br_z| {
                    let top_left = Vec3::new(tl_x, t_y, tl_z);
                    let top_right = Vec3::new(tr_x, t_y, tr_z);
                    let bottom_left = Vec3::new(bl_x, b_y, bl_z);
                    let bottom_right = Vec3::new(br_x, b_y, br_z);

                    points.push(top_left);
                    points.push(top_right);
                    points.push(bottom_left);

                    points.push(top_right);
                    points.push(bottom_right);
                    points.push(bottom_left);

                };

                make_face(tl_x_dist, y_dist, tl_z_dist, 
                    tr_x_dist, tr_z_dist, 
                    bl_x_dist, next_y_dist, bl_z_dist, 
                    br_x_dist, br_z_dist);
                make_face(tl_x_dist, -y_dist, tl_z_dist, 
                    tr_x_dist, tr_z_dist, 
                    bl_x_dist, -next_y_dist, bl_z_dist, 
                    br_x_dist, br_z_dist);

                make_face(tl_x_dist, y_dist, -tl_z_dist, 
                    tr_x_dist, -tr_z_dist, 
                    bl_x_dist, next_y_dist, -bl_z_dist, 
                    br_x_dist, -br_z_dist);
                make_face(tl_x_dist, -y_dist, -tl_z_dist, 
                    tr_x_dist, -tr_z_dist, 
                    bl_x_dist, -next_y_dist, -bl_z_dist, 
                    br_x_dist, -br_z_dist);
            }
        }
        points
    }
    fn into_vertex_list(&self, subdivs: u32) -> Vec<Vertex> {
        self.get_points(subdivs).iter().map(|pt| {
            Vertex { position: pt.as_array().clone()}
        }).collect()
    }
    pub fn into_buffer<F>(&self, display: &F, subdivs: u32) -> glium::VertexBuffer<Vertex> where F: glium::backend::Facade {
        glium::VertexBuffer::new(display, self.into_vertex_list(subdivs))
    }
    // until here

    pub fn update(&mut self) {
        self.rotation = Rot3::new(self.angular_velocity) * self.rotation;
        self.position = self.position + self.velocity;
        println!("position: {:?}", self.position);
    }
    pub fn get_homogeneous(&self) -> na::Mat4<f32> {
        na::Iso3::new_with_rotmat(self.position, self.rotation).to_homogeneous()
    }
}