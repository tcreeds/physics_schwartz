extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;

use na::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

#[derive(Clone, Copy)]
struct Sphere {
    position: Vec3<f32>,
    rotation: Vec3<f32>,
    radius: f32,
    velocity: Vec3<f32>,
    angular_velocity: Vec3<f32>,
}

impl Sphere {
    fn new(radius: f32) -> Sphere {
        Sphere {
            position: na::zero(),
            rotation: na::zero(),
            radius: radius,
            velocity: na::zero(),
            angular_velocity: na::zero(),
        }
    }

    fn get_points(&self, subdivs: u32) -> Vec<Vec3<f32>> {
        let mut points: Vec<Vec3<f32>> = vec![];
        let y_subdivs = subdivs;
        let x_subdivs = subdivs * 2;

        let mut y_angle_iter = (0..y_subdivs)
            .map(|a| a * (90 / y_subdivs) )
            .map(|a| a as f32 / 180.0f32 * std::f32::consts::PI );
        let mut next_y_angle_iter =(1..y_subdivs + 1)
            .map(|a| a * (90 / y_subdivs) )
            .map(|a| a as f32 / 180.0f32 * std::f32::consts::PI );

        for (y_angle, next_y_angle) in y_angle_iter.zip(next_y_angle_iter)
        {
            let y_dist = y_angle.cos() * self.radius;
            let xz_dist = y_angle.sin() * self.radius;

            let next_y_dist = next_y_angle.cos() * self.radius;
            let next_xz_dist = next_y_angle.sin() * self.radius;

            let mut xz_angle_iter = (0..x_subdivs)
                .map(|a| a * 180 / x_subdivs )
                .map(|a| a as f32 / 180.0 * std::f32::consts::PI );
            let mut next_xz_angle_iter = (1..x_subdivs + 1)
                .map(|a| a * 180 / x_subdivs )
                .map(|a| a as f32 / 180.0 * std::f32::consts::PI );

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
    fn update(&mut self) {
        self.rotation.x += 0.01f32;
        self.position = self.position + self.velocity;
    }
}

#[derive(Clone, Copy)]
struct RectPrism {
    center: Vec3<f32>,
    extents: Vec3<f32>,
    axes: Vec3<Vec3<f32>>,
    rotation: Vec3<f32>,
    velocity: Vec3<f32>,
    angular_velocity: Vec3<f32>,
}

impl RectPrism {
    fn new(size: f32) -> RectPrism {
        RectPrism { 
            center: na::zero(),
            extents: Vec3::new(size, size, size),
            axes: Vec3::new(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0)
                ),
            rotation: na::zero(),
            velocity: na::zero(),
            angular_velocity: na::zero(),
        }
    }
    fn get_points(&self) -> Vec<Vec3<f32>> {
        let rotated_extents = { 
            let mut rotated_extents = self.axes;
            rotated_extents.x = rotated_extents.x.mul_s(&self.extents.x);
            rotated_extents.y = rotated_extents.y.mul_s(&self.extents.y);
            rotated_extents.z = rotated_extents.z.mul_s(&self.extents.z);
            rotated_extents
        };
        let mut ret = Vec::<Vec3<f32>>::new();
        for x_mult in (-1..2).filter(|i| { i % 2 != 0 }) {
            for y_mult in (-1..2).filter(|i| { i % 2 != 0 }) {
                for z_mult in(-1..2).filter(|i| { i % 2 != 0 }) {
                    let mult = Vec3::new(x_mult as f32, y_mult as f32, z_mult as f32);
                    let extents = rotated_extents.mul_s(&mult);
                    let offset = extents.x + extents.y + extents.z;
                    ret.push(offset + self.center);
                }
            }
        }
        ret
    }

    fn into_vertex_list(&self) -> Vec<Vertex> {
        self.get_points().iter().map(|pt| {
            Vertex { position: pt.as_array().clone()}
        }).collect()
    }

    fn into_buffer<F>(&self, display: &F) -> glium::VertexBuffer<Vertex> where F: glium::backend::Facade {
        glium::VertexBuffer::new(display, self.into_vertex_list())
    }

    fn update(&self, dt: f32) {
        //self.center = self.center + self.velocity * dt;
    }
}


implement_vertex!(Vertex, position);

fn main() {

    use glium::DisplayBuild;
    use glium::index;
    use glium::Surface;

    let size = 1.0f32;

    let mut sphere1 = Sphere { 
        position: Vec3::new(0.0f32, 0.0, 10.0),
        rotation: Vec3::new(0.6f32, 0.0, 0.0),
        radius: 1.0f32,
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angular_velocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let mut sphere2 = Sphere { 
        position: Vec3::new(0.0f32, 0.0, 10.0),
        rotation: Vec3::new(0.6f32, 0.0, 0.0),
        radius: 1.0f32,
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angular_velocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let mut prism = RectPrism::new(size);/*RectPrism { center: Vec3::new(0.0f32, 2.0, 10.0),
        extents: Vec3::new(size, size, size),
        axes: Vec3::new(
            Vec3::new(1.0f32, 0.0, 0.0),
            Vec3::new(0.0f32, 1.0, 0.0),
            Vec3::new(0.0f32, 0.0, 1.0)
            ),
        rotation: Vec3::new(0.0f32, 0.0, 0.0),
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angular_velocity: Vec3::new(0.0f32, 0.0, 0.0),
    };*/
    let mut prism2 = RectPrism { center: Vec3::new(0.0f32, -2.0, 10.0),
        extents: Vec3::new(size, size, size),
        axes: Vec3::new(
            Vec3::new(1.0f32, 0.0, 0.0),
            Vec3::new(0.0f32, 1.0, 0.0),
            Vec3::new(0.0f32, 0.0, 1.0)
            ),
        rotation: Vec3::new(0.3f32, 0.0, 0.0),
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angular_velocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Hello world"))
            .build_glium().unwrap();

    let depth_buffer = glium::render_buffer::DepthRenderBuffer::new(&display, glium::texture::DepthFormat::I24, 1024, 768);
    let color_buffer = glium::texture::Texture2d::empty(&display, 1024, 768);
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_buffer, &depth_buffer);

    let vertex_buffer = glium::VertexBuffer::new(&display, prism.into_vertex_list());
    let indices = glium::index::TrianglesList(vec![0u32, 1, 2,
        1, 2, 3,
        4, 5, 6,
        5, 6, 7,
        0, 1, 4,
        1, 4, 5,
        2, 3, 6,
        3, 6, 7,
        0, 2, 4,
        2, 4, 6,
        1, 3, 5,
        3, 5, 7,
    ]);
    let sphere_vertex_buffer = glium::VertexBuffer::new(&display, sphere1.into_vertex_list(10));
    let sphere_indices = index::NoIndices(index::PrimitiveType::TrianglesList);
    
    let program = glium::Program::from_source(&display,
        // vertex shader
        "   #version 110

        uniform mat4 vp_matrix;

        attribute vec3 position;
        varying vec4 normal;

        void main() {
            gl_Position =  vp_matrix * vec4(position, 1.0);
            normal = vec4(position, 1.0);
        }
        ",

        // fragment shader
        "   #version 110

        varying vec4 normal;

        void main() {
            float mult = clamp(dot(vec4(0.0, 1.0, 0.0, 1.0), normal), 0.0, 1.0);
            gl_FragColor = vec4(1.0 * mult, 1.0 * (1.0-mult), 0.0, 1.0);
        }
        ",

        // optional geometry shader
        None
    ).unwrap();
    let persp = Persp3::new(640.0 / 480.0f32, 3.1415962535 / 4.0, 0.01, 200.0).to_mat();
    let translate = na::Iso3::new(prism.center, Vec3::new(3.14159f32 / 4.0,0.0,0.0));
    
    let source_rect = glium::Rect {
        left: 0,
        bottom: 0,
        width: 1024,
        height: 768,
    };
    let dest_rect = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: 1024,
        height: 768,
    };

    loop {

        sphere1.update();

        let smat = persp * na::Iso3::new(sphere1.position, sphere1.rotation).to_homogeneous();
        let mat1 = persp * na::Iso3::new(prism.center, prism.rotation).to_homogeneous();
        let mat2 = persp * na::Iso3::new(prism2.center, prism2.rotation).to_homogeneous();

        frame_buffer.clear_color(0.0, 0.0, 0.0, 0.0);  
        frame_buffer.clear_depth(1.0);


        let sphere_uniforms = uniform! {
            vp_matrix: *smat.as_array(),
        };

        let uniforms1 = uniform! {
            vp_matrix: *mat1.as_array(),
        };
        let uniforms2 = uniform! {
            vp_matrix: *mat2.as_array(),
        };

        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. std::default::Default::default()
        };
        
        frame_buffer.draw(&sphere_vertex_buffer, &sphere_indices, &program, &sphere_uniforms,
            &params).unwrap();

        frame_buffer.draw(&vertex_buffer, &indices, &program, &uniforms1,
            &std::default::Default::default()).unwrap();

        frame_buffer.draw(&vertex_buffer, &indices, &program, &uniforms2,
            &std::default::Default::default()).unwrap();

        frame_buffer.blit_color(&source_rect, & mut display.draw(), &dest_rect, glium::uniforms::MagnifySamplerFilter::Nearest);

    }
}


fn test_collisions(a: & mut Sphere, b: & mut Sphere) -> () {

    let dist = (a.position - b.position).norm();
    if dist < a.radius + b.radius {
        /*let normal = (a.position - b.position).normalize();
        let point_of_contact  = (a.position - b.position) / 2.0f32;
        let a_along_normal = point_of_contact - a.position;
        let b_along_normal = point_of_contact - b.position;
        let vel_AB = a.velocity * na::dot(a.velocity, a_along_normal) - b.velocity * na::dot(b.velocity, b_along_normal);

        let mut impulse = (-(0.95+1.0) * na::dot(vel_AB, normal));
        */

        //let a_perp_normal = na::cross(a_along_normal, )
    }

}



