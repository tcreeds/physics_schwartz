extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;

mod iter;
mod sphere;
mod vec_tools;


use iter::Itertools;
use sphere::*;
use vec_tools::*;

use na::*;

fn main() {

    use glium::DisplayBuild;
    use glium::index;
    use glium::Surface;

    let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Hello world"))
            .build_glium().unwrap();

    let depth_buffer = glium::render_buffer::DepthRenderBuffer::new(&display, glium::texture::DepthFormat::I24, 1024, 768);
    let color_buffer = glium::texture::Texture2d::empty(&display, 1024, 768);
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_buffer, &depth_buffer);


    let mut sphere1 = Sphere::new(1.0, 1.0);
    sphere1.position = Vec3::new(3.0, 0.9, 10.0);
    sphere1.velocity = Vec3::new(-0.06, 0.0, 0.0);
    sphere1.angular_velocity = Vec3::new(0.0, 0.0, -0.1);
    sphere1.mass = 1.0f32;

    let mut sphere2 = Sphere::new(1.0, 1.0);
    sphere2.position = Vec3::new(-3.0, 0.0, 10.0);
    sphere2.velocity = Vec3::new(0.04, 0.0, 0.0);
    sphere2.angular_velocity = Vec3::new(0.0, 0.0, -0.1);
    sphere2.mass = 1.0f32;
   
    let mut pair_list: Vec<_> = {
        let object_list = vec![sphere1, sphere2]; 
        object_list.iter().map(|s| (s.clone(), s.into_buffer(&display, 10), Vec3::new(1.0, 0.0, 0.0))).collect()
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

    
        for & mut (ref mut s, _, ref mut c) in pair_list.iter_mut() {
            s.update();
            *c = Vec3::new(1.0, 0.0, 0.0);
        }


        let color_update = {
            let mut update_index_list = vec![];
            for ((l_index, &(ref lhs, _, _)), (r_index, &(ref rhs, _, _))) in pair_list.iter().enumerate().combinate_pair() {
                let test_result = hit_test(lhs, rhs);
                match test_result {
                    Some(x) => update_index_list.push((l_index, r_index, x)),
                    None => (),
                }
            }

            update_index_list
        };

        for (li, ri, result) in color_update {
            let (& mut (ref mut lhs, _, ref mut c1), & mut (ref mut rhs, _, ref mut c2)) = pair_list.get_pair_mut(li, ri);

            resolve_collision(lhs, rhs, result);
            //let & mut (ref mut s1, _, ref mut c1) = pair_list.get_mut(li).unwrap();
            *c1 = Vec3::new(0.0, 1.0, 0.0);
            //let & mut (ref mut s2, _, ref mut c2) = pair_list.get_mut(ri).unwrap();
            *c2 = Vec3::new(0.0, 1.0, 0.0);
            //resolve_collision(s1, s2, result);
            
        }
    

        frame_buffer.clear_color(0.0, 0.0, 0.0, 0.0);  
        frame_buffer.clear_depth(1.0);
        
        for &(ref s, ref buf, ref color) in pair_list.iter() {
            let uniforms = uniform! {
                vp_matrix: *(persp * s.get_homogeneous()).as_array(),
                color: *color.as_array(),
            };

            frame_buffer.draw(buf, &sphere_indices, &program, &uniforms, &params).unwrap();
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

    let combined_mass = 1.0f32 / lhs.mass + 1.0f32 / rhs.mass;

    let coeff_lhs = 2.0f32 / 5.0 * lhs.mass * lhs.radius * lhs.radius;
    let inertia_tensor_lhs = na::Mat3::new(coeff_lhs, 0.0, 0.0, 0.0, coeff_lhs, 0.0, 0.0, 0.0, coeff_lhs);
    let coeff_rhs = 2.0f32 / 5.0 * rhs.mass * rhs.radius * rhs.radius;
    let inertia_tensor_rhs = na::Mat3::new(coeff_rhs, 0.0, 0.0, 0.0, coeff_rhs, 0.0, 0.0, 0.0, coeff_rhs);

    let inv_tensor_lhs = na::inv(&inertia_tensor_lhs).unwrap();
    let inv_tensor_rhs = na::inv(&inertia_tensor_rhs).unwrap();

    let radius_lhs = res.contact_point - lhs.position;
    let radius_rhs = res.contact_point - rhs.position;

    let tensor_product_lhs = inv_tensor_lhs * na::cross(&(na::cross(&radius_lhs, &res.normal)), &radius_lhs);
    let tensor_product_rhs = inv_tensor_rhs * na::cross(&(na::cross(&radius_rhs, &res.normal)), &radius_rhs);

    let impulse = -(res.restitution) * na::dot(&res.relative_velocity, &res.normal) / (combined_mass + na::dot(&(tensor_product_lhs + tensor_product_rhs), &res.normal));

    lhs.velocity = lhs.velocity + res.normal * impulse;
    rhs.velocity = rhs.velocity - res.normal * impulse;
    println!("\nnew_b: {:?}", rhs.velocity);
    println!("new_a: {:?}", lhs.velocity);
    println!("\nimp:   {:?}", impulse);
    println!("rel v: {:?}\n", res.relative_velocity);
    lhs.position = lhs.position + res.normal * (lhs.radius / (lhs.position - res.contact_point).norm() - 1.0f32);
    rhs.position = rhs.position - res.normal * (rhs.radius / (rhs.position - res.contact_point).norm() - 1.0f32);

    println!("ang_lhs:     {:?}", lhs.angular_velocity);
    println!("ang_rhs:     {:?}", rhs.angular_velocity);

    //lhs.angular_velocity = lhs.angular_velocity - inv_tensor_lhs * na::cross(&(radius_lhs * lhs.angular_velocity), &(res.relative_perp_velocity * impulse));
    //rhs.angular_velocity = rhs.angular_velocity - inv_tensor_rhs * na::cross(&(radius_rhs * rhs.angular_velocity), &(res.relative_perp_velocity * impulse));

    let lhs_ang_vel = lhs.angular_velocity.clone();
    let rhs_ang_vel = rhs.angular_velocity.clone();

    lhs.angular_velocity = lhs.angular_velocity + inv_tensor_lhs * (na::cross(&res.relative_perp_velocity, &radius_rhs) + radius_rhs * rhs_ang_vel * impulse);
    rhs.angular_velocity = rhs.angular_velocity - inv_tensor_rhs * (na::cross(&res.relative_perp_velocity, &radius_lhs) + radius_lhs * lhs_ang_vel * impulse);

    println!("new_ang_lhs: {:?}", lhs.angular_velocity);
    println!("new_ang_rhs: {:?}", rhs.angular_velocity);
    
    println!("lhs_radius: {:?}", radius_lhs);
    println!("rhs_radius: {:?}", radius_rhs);

    println!("impulse: {:?}", res.relative_velocity);

    

}

fn hit_test(a: & Sphere, b: & Sphere) -> Option<CollisionResult> {

    let dist = (a.position - b.position).norm();
    if dist <= a.radius + b.radius {
        //if na::dot(&a.velocity, &(b.position - a.position)) > 0.0f32 || na::dot(&b.velocity, &(a.position - b.position)) > 0.0f32 {
        
            let contact_normal = (a.position - b.position).normalize();
            let point_of_contact  = b.position + (a.position - b.position) / 2.0f32;
            let rel_a = a.velocity * na::dot(&a.velocity.clone().normalize(), &contact_normal);
            let rel_b = b.velocity * na::dot(&b.velocity.clone().normalize(), &(-contact_normal));
            let rel_vel = rel_b - rel_a;

            println!("norm:  {:?}", contact_normal);
            println!("rel_b: {:?}", b.velocity);
            println!("rel_a: {:?}", a.velocity);

            let result = CollisionResult {
                normal: contact_normal,
                contact_point: point_of_contact,
                relative_velocity: rel_vel,
                relative_perp_velocity: (b.velocity - rel_b) - (a.velocity - rel_a),
                restitution: 1.0f32,
            };
            Some(result)
        /*}
        else {
            None
        }*/
        //let angular = ;
        //let rel_angular_velocity_a = na::cross(&lhs.velocity, &(-(contact_normal.clone().normalize())));
        //let rel_angular_velocity_b = na::cross(&rhs.velocity, &(contact_normal.clone().normalize()));


    } else {
        None
    }
}



