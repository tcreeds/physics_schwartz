extern crate glutin;
#[macro_use]
extern crate glium;


#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::DisplayBuild;
    use glium::index;
    use glium::Surface;
        
    let display = glutin::WindowBuilder::new()
        .with_dimensions(640, 480)
        .with_title(format!("Hello world"))
        .build_glium().unwrap();
    
    let vertex_buffer = glium::VertexBuffer::new(&display, vec![
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [ 0.0,  0.5, 0.0] },
        Vertex { position: [ 0.5, -0.5, 0.0] },
    ]);
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        
    let program = glium::Program::from_source(&display,
        // vertex shader
        "   #version 110

        uniform mat4 matrix;

        attribute vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0) * matrix;
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
        matrix: [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 0.0, 1.0 ]
        ]
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

fn test_intersection(shape_a: Shape, shape_b: Shape) Option<Simplex, ()> {
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
            };
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

fn farthest_along(dir: nalgebra::Vec3, shape: Shape) -> nalgebra::Vec3{
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
