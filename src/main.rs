extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;
extern crate itertools;

mod sphere;
mod vec_tools;
mod plane;
mod softbody;
mod parser;
mod vm;

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

    /*let test = "d = 1 + 2 * 3 - 4 * (5 + 6)";
    let mut toks = Tokenizer::new(&test[..]);
    let line_test = parse_line(& mut toks);
    let Line::Assign(name, expr) = line_test;
    let mut registers: std::collections::HashMap<_, _> = std::collections::HashMap::new();
    registers.insert("x", 0);
    registers.insert("y", 1);
    println!("{:?}", vm::VM::optimize(expr.clone()));
    let vm_test = vm::VM::compile(vm::VM::optimize(expr), &registers); 
    let data = vec![1.0, 1.0];
    println!("{}: {:?}", name, vm_test.run(&data));
    println!("{:?}", vm_test);*/

    //panic!(); 

    let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Hello world"))
            .build_glium().unwrap();

    let depth_buffer = glium::render_buffer::DepthRenderBuffer::new(&display, glium::texture::DepthFormat::I24, 1024, 768);
    let color_buffer = glium::texture::Texture2d::empty(&display, 1024, 768);
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_buffer, &depth_buffer);


    let mut sphere1 = Sphere::new(2.0f32, 1.0f32);
    sphere1.position = Vec3::new(3.0f32, 5.4, 20.0);
    sphere1.velocity = Vec3::new(0.0f32, 0.005, 0.0);
    sphere1.angular_velocity = Vec3::new(0.0f32, 0.0, 0.1);
    sphere1.mass = 1.0f32;

    let sphere_buf = Sphere::into_buffer(&display, 10);

    let mut sphere2 = Sphere::new(1.0f32, 1.0f32);
    sphere2.position = Vec3::new(-2.0f32, 10.0, 21.0);
    sphere2.velocity = Vec3::new(0.03f32, -0.01, 0.0);
    sphere2.angular_velocity = Vec3::new(0.0f32, 0.0, -0.0);
    sphere2.mass = 1.0f32;

    let mut softsphere = SoftBody::new(Vec3::new(-5.0f32, 3.0, 21.0), 1.0f32);

    let bottom_plane = Plane::new(Vec3::new(0.0f32, 0.0, 0.0), Vec3::new(0.0f32, 1.0, 0.0), 0.95f32);
   
    let mut pair_list: Vec<_> = {
        let object_list = vec![sphere1, sphere2]; 
        object_list.iter().map(|s| (s.clone(), Vec3::new(1.0, 0.0, 0.0))).collect()
    };

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
            *c = Vec3::new(1.0, 0.0, 0.0);
        }
        //softbody particle update
        softsphere.update();
        
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
                    Some(x) => {println!("{:?}", x); resolve_collision(point, sph, x); break 'out},
                    None => (),
                }
            }
        }

        for (li, ri, result) in color_update {
            let (& mut (ref mut lhs, ref mut c1), & mut (ref mut rhs, ref mut c2)) = pair_list.get_pair_mut(li, ri);

            resolve_collision(lhs, rhs, result);
            //let & mut (ref mut s1, _, ref mut c1) = pair_list.get_mut(li).unwrap();
            *c1 = Vec3::new(0.0, 1.0, 0.0);
            //let & mut (ref mut s2, _, ref mut c2) = pair_list.get_mut(ri).unwrap();
            *c2 = Vec3::new(0.0, 1.0, 0.0);
            //resolve_collision(s1, s2, result);
            
        }
        for & mut (ref mut s, _) in pair_list.iter_mut() {
            if bottom_plane.check_collision(s) {
                bottom_plane.bounce_sphere(s);
            }
            //gravity
            s.velocity.y -= 0.001f32;
            
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
    contact_point: Vec3<f32>,
    relative_velocity: Vec3<f32>,
    relative_perp_velocity: Vec3<f32>,
    restitution: f32,
}

//doesn't deal with rotation yet
fn resolve_collision(lhs: & mut Sphere, rhs: & mut Sphere, res: CollisionResult) -> () {

    //mass of objects
    let combined_mass = 1.0f32 / lhs.mass + 1.0f32 / rhs.mass;

    //info to construct intertia tensors, inertia tensors
    let coeff_lhs = 2.0f32 / 5.0 * lhs.mass * lhs.radius * lhs.radius;
    let inertia_tensor_lhs = na::Mat3::new(coeff_lhs, 0.0, 0.0, 0.0, coeff_lhs, 0.0, 0.0, 0.0, coeff_lhs);
    let coeff_rhs = 2.0f32 / 5.0 * rhs.mass * rhs.radius * rhs.radius;
    let inertia_tensor_rhs = na::Mat3::new(coeff_rhs, 0.0, 0.0, 0.0, coeff_rhs, 0.0, 0.0, 0.0, coeff_rhs);

    //inverse inertia tensors
    let inv_tensor_lhs = na::inv(&inertia_tensor_lhs).unwrap();
    let inv_tensor_rhs = na::inv(&inertia_tensor_rhs).unwrap();

    //resolve penetration
    lhs.position = lhs.position + res.normal * (lhs.radius / (lhs.position - res.contact_point).norm() - 1.0f32);
    rhs.position = rhs.position - res.normal * (rhs.radius / (rhs.position - res.contact_point).norm() - 1.0f32);

    //get radius from center of object to contact point
    let radius_lhs = res.contact_point - lhs.position;
    let radius_rhs = res.contact_point - rhs.position;

    let numerator = -(1.0f32 + res.restitution) * na::dot(&res.relative_velocity, &res.normal);
    let denominator = combined_mass + na::dot(&na::cross(&(inv_tensor_lhs * na::cross(&radius_lhs, &res.normal)), &radius_lhs), &res.normal) + na::dot(&na::cross(&(inv_tensor_rhs * na::cross(&radius_rhs, &res.normal)), &radius_rhs), &res.normal);
    
    let impulse =  res.normal * (numerator / denominator); 

    let perp_lhs = res.relative_velocity.clone().normalize() * radius_lhs.norm();
    let perp_rhs = res.relative_velocity.clone().normalize() * radius_rhs.norm();

    lhs.angular_velocity = lhs.angular_velocity - na::cross(&perp_lhs, &(impulse));
    rhs.angular_velocity = rhs.angular_velocity - na::cross(&perp_rhs, &(impulse));

    lhs.velocity = lhs.velocity + impulse;
    rhs.velocity = rhs.velocity - impulse;

    

}

fn hit_test(a: & Sphere, b: & Sphere) -> Option<CollisionResult> {

    let dist = (a.position - b.position).norm();
    if dist <= a.radius + b.radius {
        //if na::dot(&a.velocity, &(b.position - a.position)) > 0.0f32 || na::dot(&b.velocity, &(a.position - b.position)) > 0.0f32 {
        
        let contact_normal = (a.position - b.position).normalize();
        let point_of_contact = b.position + (a.position - b.position) / 2.0f32;
        let rel_a = contact_normal * na::dot(&a.velocity, &contact_normal);
        let rel_b = -contact_normal * na::dot(&b.velocity, &(-contact_normal));
        let rel_vel = rel_b - rel_a;

        let result = CollisionResult {
            normal: contact_normal,
            contact_point: point_of_contact,
            relative_velocity: rel_vel,
            relative_perp_velocity: (b.velocity - rel_b) - (a.velocity - rel_a),
            restitution: 0.5f32,
        };
        Some(result)

    } else {
        None
    }
}



