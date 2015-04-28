extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra;

use nalgebra::{Vec3, ScalarMul, Persp3, Mat4};


#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

#[derive(Clone, Copy)]
struct RectPrism {
    center: Vec3<f32>,
    extents: Vec3<f32>,
    axes: Vec3<Vec3<f32>>,
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
}


implement_vertex!(Vertex, position);

fn main() {

    use glium::DisplayBuild;
    use glium::index;
    use glium::Surface;

    let prism = RectPrism { center: Vec3::new(-0.2f32, 0.0, 0.0),
        extents: Vec3::new(0.2f32, 0.2, 0.2),
        axes: Vec3::new(
            Vec3::new(1.0f32, 0.0, 0.0),
            Vec3::new(0.0f32, 1.0, 0.0),
            Vec3::new(0.0f32, 0.0, 1.0)
            ),
    };


    println!("{:?}", prism.get_points());

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
        ]);//index::NoIndices(index::PrimitiveType::TrianglesList);
    
    let program = glium::Program::from_source(&display,
            // vertex shader
            "   #version 110

            uniform mat4 matrix;

            attribute vec3 position;

            void main() {
                gl_Position =  matrix * vec4(position, 1.0);
            }
            ",

            // fragment shader
            "   #version 110

            void main() {
                gl_FragColor = vec4(1.0, 1.0, 0.0, 1.0);
            }
            ",

            // optional geometry shader
            None
    ).unwrap();
    
    let uniforms = uniform! {
            matrix: *Persp3::new(480.0f32 / 640.0, 3.14159 / 2.0, 0.01, 200.0).to_mat().as_array()
            /*[
                    [ 1.0, 0.0, 0.0, 0.0 ],
                    [ 0.0, 1.0, 0.0, 0.0 ],
                    [ 0.0, 0.0, 1.0, 0.0 ],
                    [ 0.0, 0.0, 0.0, 1.0 ]
            ]*/
    };
    
    loop {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);  // filling the output with the black color
            target.draw(&vertex_buffer, &indices, &program, &uniforms,
                        &std::default::Default::default()).unwrap();
            target.finish();        
            
    }
}

