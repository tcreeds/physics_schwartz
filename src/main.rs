extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;

use na::{Vec3, ScalarMul, Persp3, Mat4, ToHomogeneous};


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
    angularVelocity: Vec3<f32>,
}

impl Sphere {
    fn get_points(&self) -> Vec<Vec3<f32>> {
        let mut points = Vec::<Vec3<f32>>::new();
        points.push(Vec3::new(-0.5f32,    0.0,    0.85,));
        points.push(Vec3::new(0.5f32,     0.0,    0.85,));
        points.push(Vec3::new(-0.5f32,    0.0,    -0.85,));
        points.push(Vec3::new(0.5f32,     0.0,    -0.85,));
        points.push(Vec3::new(0.0f32,     0.85,   0.5,));
        points.push(Vec3::new(0.0f32,     0.85,   -0.5,));
        points.push(Vec3::new(0.0f32,     -0.85,  0.5,));
        points.push(Vec3::new(0.0f32,     -0.85,  -0.5,));
        points.push(Vec3::new(0.85f32,    0.52,   0.0,));
        points.push(Vec3::new(-0.85f32,   0.52,   0.0,));
        points.push(Vec3::new(0.85f32,    -0.52,  0.0,));
        points.push(Vec3::new(-0.85f32,   -0.52,  0.0,));
        points
    }
    fn into_vertex_list(&self) -> Vec<Vertex> {
        self.get_points().iter().map(|pt| {
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
    angularVelocity: Vec3<f32>,
}

impl RectPrism {
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
        velocity: Vec3::new(0.01f32, 0.0, 0.0),
        angularVelocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let mut sphere2 = Sphere { 
        position: Vec3::new(0.0f32, 0.0, 10.0),
        rotation: Vec3::new(0.6f32, 0.0, 0.0),
        radius: 1.0f32,
        velocity: Vec3::new(0.01f32, 0.0, 0.0),
        angularVelocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let mut prism = RectPrism { center: Vec3::new(0.0f32, 2.0, 10.0),
        extents: Vec3::new(size, size, size),
        axes: Vec3::new(
            Vec3::new(1.0f32, 0.0, 0.0),
            Vec3::new(0.0f32, 1.0, 0.0),
            Vec3::new(0.0f32, 0.0, 1.0)
            ),
        rotation: Vec3::new(0.0f32, 0.0, 0.0),
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angularVelocity: Vec3::new(0.0f32, 0.0, 0.0),
    };
    let mut prism2 = RectPrism { center: Vec3::new(0.0f32, -2.0, 10.0),
        extents: Vec3::new(size, size, size),
        axes: Vec3::new(
            Vec3::new(1.0f32, 0.0, 0.0),
            Vec3::new(0.0f32, 1.0, 0.0),
            Vec3::new(0.0f32, 0.0, 1.0)
            ),
        rotation: Vec3::new(0.3f32, 0.0, 0.0),
        velocity: Vec3::new(0.0f32, 0.0, 0.0),
        angularVelocity: Vec3::new(0.0f32, 0.0, 0.0),
    };

    let display = glutin::WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title(format!("Hello world"))
            .build_glium().unwrap();

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
    let sphere_vertex_buffer = glium::VertexBuffer::new(&display, sphere1.into_vertex_list());
    let sphere_indices = glium::index::TrianglesList(vec![0u32, 1, 2,
        0,4,1,
        0,9,4,
        9,5,4,
        4,5,8,
        4,8,1,
        8,10,1,
        8,3,10,
        5,3,8,
        5,2,3,
        2,7,3,
        7,10,3,
        7,6,10,
        7,11,6,
        11,0,6,
        0,1,6,
        6,1,10,
        9,0,11,
        9,11,2,
        9,2,5,
        7,2,11,
    ]);
    //index::NoIndices(index::PrimitiveType::TrianglesList);
    
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
    
    loop {

        sphere1.update();

        let smat = persp * na::Iso3::new(sphere1.position, sphere1.rotation).to_homogeneous();
        let mat1 = persp * na::Iso3::new(prism.center, prism.rotation).to_homogeneous();
        let mat2 = persp * na::Iso3::new(prism2.center, prism2.rotation).to_homogeneous();

        let mut target = display.draw();
        //let mut target = glium::SimpleFrameBuffer::new(&display,  
        target.clear_color(0.0, 0.0, 0.0, 0.0);  // filling the output with the black color

        let sphere_uniforms = uniform! {
            vp_matrix: *smat.as_array(),
        };

        let uniforms1 = uniform! {
            vp_matrix: *mat1.as_array(),// *na::Ortho3::new(640.0f32, 480.0, -10.0, 10.0).to_mat().as_array()
        };
        let uniforms2 = uniform! {
            vp_matrix: *mat2.as_array(),// *na::Ortho3::new(640.0f32, 480.0, -10.0, 10.0).to_mat().as_array()
        };

        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. std::default::Default::default()
        };
        
        target.draw(&sphere_vertex_buffer, &sphere_indices, &program, &sphere_uniforms,
            &std::default::Default::default()).unwrap();

        target.draw(&vertex_buffer, &indices, &program, &uniforms1,
            &std::default::Default::default()).unwrap();

        target.draw(&vertex_buffer, &indices, &program, &uniforms2,
            &std::default::Default::default()).unwrap();

        target.finish();      
    }
}

fn test_collisions(&mut a: Sphere, &mut b: Sphere) {

    let dist = (a.position - b.position).norm();
    if ( dist < a.radius + b.radius){
        let normal = (a.position - b.position).normalize();
        let point_of_contact  = (a.position - b.position) / 2.0f32;
        let a_along_normal = point_of_contact - a.position;
        let b_along_normal = point_of_contact - b.position;
        let vel_AB = a.velocity * na::dot(a.velocity, a_along_normal) - b.velocity * na::dot(b.velocity, b_along_normal);

        let mut impulse = (-(0.95+1.0) * na::dot(vel_AB, normal));

        //let a_perp_normal = na::cross(a_along_normal, )
    }

}



