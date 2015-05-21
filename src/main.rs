extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;
extern crate itertools;

mod sphere;
mod vec_tools;
mod plane;
mod vm;
mod parser;
mod softbody;

use itertools::Itertools;
use sphere::*;
use vec_tools::*;
use plane::*;
use softbody::*;
use parser::*;

use na::*;

fn main() {

    use glium::DisplayBuild;
    use glium::index;
    use glium::Surface;

    use std::io::BufRead;

    let mut k = 0.01;
    let mut g = -0.01;
    let mut dampening = 0.03;
    let mut restitution = 1.0;

    let mut cr_registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
    cr_registers.insert("p", 0);
    cr_registers.insert("other_p", 1);
    cr_registers.insert("mass", 2);
    let mut collision_response = vm::VM::compile(vm::VM::optimize(parse_expr(& mut Tokenizer::new("(0 - p + other_p) / mass\n"))), &cr_registers);

    let mut sf_registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
    sf_registers.insert("x", 0);
    sf_registers.insert("v", 1);
    sf_registers.insert("dampening", 2);
    sf_registers.insert("k", 3);
    let mut spring_force = vm::VM::compile(vm::VM::optimize(parse_expr(& mut Tokenizer::new("0 - k * x - dampening * v\n"))), &sf_registers);
    match std::fs::File::open("eq.txt") {
        Ok(f) => {
            let f = std::io::BufReader::new(f);
            for line in f.lines() {
                match line {
                    Ok(line) => {
                        if !line.starts_with("//") {
                            let mut toks = Tokenizer::new(&line[..]);
                            let Line::Assign(name, expr) = parse_line(& mut toks);
                            match &name[..] {
                                "spring_force" => {
                                    spring_force = vm::VM::compile(vm::VM::optimize(expr), &sf_registers);
                                },
                                "collision_response" => {
                                    collision_response = vm::VM::compile(vm::VM::optimize(expr), &cr_registers);
                                },
                                "k" => {
                                    let registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
                                    let k_vm = vm::VM::compile(vm::VM::optimize(expr), &registers);
                                    let data = vec![];
                                    k = k_vm.run(&data) as f32;
                                },
                                "dampening" => {
                                    let registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
                                    let d_vm = vm::VM::compile(vm::VM::optimize(expr), &registers);
                                    let data = vec![];
                                    dampening = d_vm.run(&data) as f32;
                                },
                                "g" => {
                                    let registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
                                    let g_vm = vm::VM::compile(vm::VM::optimize(expr), &registers);
                                    let data = vec![];
                                    g = g_vm.run(&data) as f32;
                                },
                                "restitution" => {
                                    let registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
                                    let r_vm = vm::VM::compile(vm::VM::optimize(expr), &registers);
                                    let data = vec![];
                                    restitution = r_vm.run(&data) as f32;
                                },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
               } 
            }
        },
        _ => ()
    }
    


    let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Hello world"))
            .build_glium().unwrap();

    let depth_buffer = glium::render_buffer::DepthRenderBuffer::new(&display, glium::texture::DepthFormat::I24, 1024, 768);
    let color_buffer = glium::texture::Texture2d::empty(&display, 1024, 768);
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_buffer, &depth_buffer);

    let mut sphere1 = Sphere::new(1.0f32, 1.0f32);
    sphere1.position = Vec3::new(6.0f32, 5.0, 20.0);
    sphere1.velocity = Vec3::new(0.0f32, 0.009, 0.005);
    sphere1.angular_velocity = Vec3::new(0.0f32, 0.0, 0.0);
    sphere1.mass = 1.0f32;

    let sphere_buf = Sphere::into_buffer(&display, 10);

    let mut sphere2 = Sphere::new(1.0f32, 1.0f32);
    sphere2.position = Vec3::new(-6.0f32, 6.0, 20.0);
    sphere2.velocity = Vec3::new(0.05f32, 0.005, 0.0);
    sphere2.angular_velocity = Vec3::new(0.0f32, 0.0, -0.0);
    sphere2.mass = 1.0f32;

    let mut pair_list: Vec<_> = {
        let object_list = vec![sphere1, sphere2]; 
        object_list.iter().map(|s| (s.clone(), Vec3::new(1.0, 0.0, 0.0))).collect()
    };

    let mut softsphere = SoftBody::new(Vec3::new(0.0f32, 2.0, 20.0), 2.0f32);

    let bottom_plane = Plane::new(Vec3::new(0.0f32, -5.0, 0.0), Vec3::new(0.0f32, 1.0, 0.0), restitution);
    let right_plane = Plane::new(Vec3::new(10.0f32, 0.0, 0.0), Vec3::new(-1.0f32, 0.0, 0.0), restitution);
    let left_plane = Plane::new(Vec3::new(-10.0f32, 0.0, 0.0), Vec3::new(1.0f32, 0.0, 0.0), restitution);
    let back_plane = Plane::new(Vec3::new(0.0f32, 0.0, 25.0), Vec3::new(0.0f32, 0.0, -1.0), restitution);
    let front_plane = Plane::new(Vec3::new(0.0f32, 0.0, 10.0), Vec3::new(0.0f32, 0.0, 1.0), restitution);

    let plane_list: Vec<Plane> = vec![bottom_plane, right_plane, left_plane, back_plane, front_plane];

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
        uniform vec3 color;

        varying vec4 normal;

        void main() {
            float mult = clamp(dot(vec4(0.0, 1.0, 0.0, 1.0), normal), 0.0, 1.0);
            gl_FragColor = vec4(color, 1.0) * mult;
        }
        ",

        // optional geometry shader
        None
    ).unwrap();
    
    let persp = Persp3::new(640.0 / 480.0f32, 3.1415962535 / 4.0, 0.01, 200.0).to_mat();

    let params = glium::DrawParameters {
        depth_test: glium::DepthTest::IfLess,
        depth_write: true,
        .. std::default::Default::default()
    };


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
    'main_loop: loop {
        for e in display.poll_events()
        {
            match e
            {
                glutin::Event::Closed => break 'main_loop,
                _ => ()
            }
        }

        for & mut (ref mut s, ref mut c) in pair_list.iter_mut() {
            s.update();
            s.velocity.y += g;
                
            *c = Vec3::new(1.0, 0.0, 0.0);
        }
        //softbody particle update
        softsphere.update(g, k, dampening, &spring_force);
        
        let color_update = {
            let mut update_index_list = vec![];
            for ((l_index, &(ref lhs, _)), (r_index, &(ref rhs, _))) in pair_list.iter().enumerate().combinations() {
                let test_result = hit_test(lhs, rhs);
                match test_result {
                    Some(x) => update_index_list.push((l_index, r_index, x)),
                    None => (),
                }
            }

            update_index_list
        };

        for & mut (ref mut sph, _) in pair_list.iter_mut(){
            'out: for ref mut point in softsphere.get_points_mut().iter_mut() {
                let test_result = hit_test(point, sph);
                match test_result {
                    Some(x) => resolve_collision(point, sph, x, &collision_response) ,
                    None => (),
                }
            }
        }

        for (li, ri, result) in color_update {
            let (& mut (ref mut lhs, ref mut c1), & mut (ref mut rhs, ref mut c2)) = pair_list.get_pair_mut(li, ri);

            resolve_collision(lhs, rhs, result, &collision_response);
            *c1 = Vec3::new(0.0, 1.0, 0.0);
            *c2 = Vec3::new(0.0, 1.0, 0.0);
        }
        for plane in plane_list.iter(){
            for & mut (ref mut s, _) in pair_list.iter_mut() {
                if plane.check_collision(s) {
                    plane.bounce_sphere(s);
                }
            }
            for ref mut s in softsphere.get_points_mut().iter_mut() {
                if plane.check_collision(s) {
                    plane.bounce_sphere(s);
                }
            }
        }

        frame_buffer.clear_color(0.0, 0.0, 0.0, 0.0);  
        frame_buffer.clear_depth(1.0);
        for &(ref s, ref color) in pair_list.iter() {
            let uniforms = uniform! {
                vp_matrix: *(persp * s.get_homogeneous()).as_array(),
                color: *color.as_array(),
            };

            frame_buffer.draw(&sphere_buf, &sphere_indices, &program, &uniforms, &params).unwrap();
        }
        
        for ref s in softsphere.get_points().iter() {
           let uniforms = uniform! {
                vp_matrix: *(persp * s.get_homogeneous()).as_array(),
                color: *Vec3::new(1.0, 1.0, 1.0).as_array(),
            };

            frame_buffer.draw(&sphere_buf, &sphere_indices, &program, &uniforms, &params).unwrap();
        }

        frame_buffer.blit_color(&source_rect, & mut display.draw(), &dest_rect, glium::uniforms::MagnifySamplerFilter::Nearest);
    }
}

#[derive(Debug)]
struct CollisionResult {
    normal: Vec3<f32>,
    mtv: Vec3<f32>,
}

//doesn't deal with rotation yet
fn resolve_collision(lhs: & mut Sphere, rhs: & mut Sphere, res: CollisionResult, mac: & vm::VM) -> () {
    let total_radius = lhs.radius + rhs.radius;    
    lhs.position = lhs.position + res.mtv * (lhs.radius / total_radius);
    rhs.position = rhs.position - res.mtv * (rhs.radius / total_radius);

    let p_lhs = na::dot(&lhs.velocity, &res.normal) * lhs.mass;
    let p_rhs = na::dot(&rhs.velocity, &res.normal) * rhs.mass;
    
    let data = vec![p_lhs as f64, p_rhs as f64, lhs.mass as f64];
    let d_s_f_lhs = mac.run(&data) as f32;

    let data = vec![p_rhs as f64, p_lhs as f64, rhs.mass as f64];
    let d_s_f_rhs = mac.run(&data) as f32;
/*    let p_f_lhs = -p_lhs + p_rhs;
    let p_f_rhs = -p_rhs + p_lhs;

    let d_s_f_lhs = p_f_lhs / lhs.mass;
    let d_s_f_rhs = p_f_rhs / rhs.mass;*/

    let d_v_f_lhs = res.normal * d_s_f_lhs;
    let d_v_f_rhs = res.normal * d_s_f_rhs;

    lhs.velocity = lhs.velocity + d_v_f_lhs;
    rhs.velocity = rhs.velocity + d_v_f_rhs;
}

fn hit_test(lhs: & Sphere, rhs: & Sphere) -> Option<CollisionResult> {

    let dist = (lhs.position - rhs.position).norm();
    if dist <= lhs.radius + rhs.radius {
        
        let contact_normal = (lhs.position - rhs.position).normalize();
        let mtv = contact_normal * (lhs.radius + rhs.radius - dist);

        let result = CollisionResult {
            normal: contact_normal,
            mtv: mtv,
        };
        Some(result)

    } else {
        None
    }
}



