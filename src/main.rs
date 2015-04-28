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

        uniform mat4 vp_matrix;

        attribute vec3 position;

        void main() {
            gl_Position =  vp_matrix * vec4(position, 1.0);
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
    
    let mut uniforms = uniform! {
        vp_matrix: *Persp3::new(480.0f32 / 640.0, 3.14159 / 2.0, 0.01, 200.0).to_mat().as_array() * compute_view_matrix(Vec3(0, -4, 0), Vec3(0, 0, 0)),
    };
    
    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);  // filling the output with the black color
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &std::default::Default::default()).unwrap();
        target.finish();        
    }
}

enum Simplex {
    Point(nalgebra::Vec3),
    Line(nalgebra::Vec3, nalgebra::Vec3),
    Triangle(nalgebra::Vec3, nalgebra::Vec3, nalgebra::Vec3),
    Tetrahedron(nalgebra::Vec3, nalgebra::Vec3, nalgebra::Vec3, nalgebra::Vec3)
}

impl Simplex {
    fn add_point(&self, pt: nalgebra::Vec3) -> Simplex {
        match self {
            Point(v1) => Line(v1, pt),
            Line(v1, v2) => Triangle(v1, v2, pt),
            Triangle(v1, v2, v3) => Tetrahedron(v1, v2, v3, pt),
            _ => panic!("Can't add a point to this!"),
        }
    }
}

fn test_intersection(shape_a: Shape, shape_b: Shape) -> Option<Simplex, ()> {
    //pick random vertex
    let firstPoint = shape_a.vertices[0] - shape_b.vertices[0];
    let direction = -firstPoint;
     //create array of vertices
    let mut simplex = Simplex::Point(firstPoint);
   
    loop{
        //compute furthest point in direction with support function
        let vert = support(direction, shape_a, shape_b);
        //check if vert is closest point to origin
        if (vert::dot(direction) < 0){
            //already closest point, no intersection
            //return none?
            None
        }
        simplex = simplex.add_point(vert);
        
        match do_simplex(simplex, direction) {
            Ok(ret) => return simplex,
            Err((sim, dir)) => {
                simplex = sim;
                direction = dir;
            }
        }
    } 
}

fn do_simplex(simplex: Simplex, direction: nalgebra::Vec3) -> Result<Simplex, (Simplex, nalgebra::Vec3)> {
    match simplex {
        Line(b, a) => {
            if (b - a).dot(direction) > 0 {
                (simplex, (b - a).cross(origin - a).cross(b - a))
            } else {
                (Simplex::Point(a), -a)
            }
        },
        Triangle(c, b, a) => {
            let ab = (b - a);
            let ac = (c - a);
            let abc = ab.cross(ac);
            if abc.cross(ac).dot(direction) > 0 {
                if ac.dot(direction) > 0 {
                    (Simplex::Line(a, c), ac.cross(-a).cross(ac))
                } else {
                    if ba.dot(direction) > 0 {
                        (Simplex::Line(a, b), ab.cross(-a).cross(ab))
                    } else {
                        (Simplex::Point(a), -a)
                    }
                }
            } else {
                if ab.cross(abc).dot(direction) > 0 {
                    if ab.dot(direction) > 0 {
                        (Simplex::Line(a, b), ab.cross(-a).cross(ab))
                    } else {
                        (Simplex::Point(a), -a)
                    }
                } else {
                    if abc.dot(direction) > 0 {
                        (Simplex::Triangle(a, b, c), abc)
                    } else {
                        (Simplex::Triangle(a, c, b), -abc)
                    }
                }
            }
        },
        Tetrahedron(d, c, b, a) => {
            if (d - a).cross(b - a).dot(direction) > 0 { 
                do_simplex(Simplex::Triangle(a, b, d), (d - a).cross(b - a))
            } else if (c - b).cross(d - b).dot(direction) > 0 {
                do_simplex(Simplex::Triangle(b, c, d), (c - b).cross(d - b))
            } else if (a - c).cross(d - c).dot(direction) > 0 {
                do_simplex(Simplex::Triangle(c, a, d), (a - c).cross(d - c))
            } else if (c - a).cross(b - a).dot(direction) > 0 {
                do_simplex(Simplex::Triangle(a, c, b), (c - a).cross(b - a))
            } else {
                simplex
            }
        },
        _ => panic!("Can't check this simplex."),
    }
}

fn support(dir: nalgebra::Vec3, shape_b: Shape, shape_b: Shape) -> nalgebra::Vec3{
    let pa = farthest_along(dir, shape_a);
    let pb = farthest_along(-dir, shape_b);
    pa - pb
}

fn farthest_along(dir: Vec3, shape: Shape) -> algebra::Vec3{
    let mut max = 0;
    let mut i_of_max = 0;

    for i in 0..shape.vertices.len(){
        let dot_along_dir = dir::dot(shape.vertices[i]);
        if (dot_along_dir > max){
            max = dot_along_dir;
            i_of_max = i;
        }
    }
    shape.vertices[i_of_max]
}

fn compute_view_matrix(cam_position: Vec3, look_at: Vec3) -> Mat4 {
    let z_axis = (look_at - cam_position).normalize();
    let y_axis = Vec3(0, 1, 0).cross(z_axis).normalize();
    let x_axis = z_axis.cross(x_axis).normalize();

    Mat4(
        x_axis.x, x_axis.y, x_axis.z, -x_axis.dot(cam_position),
        y_axis.x, y_axis.y, y_axis.z, -y_axis.dot(cam_position),
        z_axis.x, z_axis.y, z_axis.z, -z_axis.dot(cam_position),
        0, 0, 0, 1,
    )
}

fn resolve_collision(shape_a: Shape, shape_b: Shape, simplex: Simplex) {

    match simplex {
        Simplex::Tetrahedron(p1, p2, p3, p4) => {
            let point_of_collision = min_point([p1, p2, p3, p4]);
            let rAP = point_of_collision - shape_a.position;
            let rBP = point_of_collision - shape_b.position;
            //moar calculations
        },
        _ => println!("Simplex is not a tetrahedron"),
    }
}

fn min_point(arr: &[Vec3; 4]) -> Vec3 {
    
    let mut point = arr[0];
    for i in 1..4 {
        if arr[i].dot(arr[i]) < point.dot(point) {
            point = arr[i]
        }
    }
    point
}
