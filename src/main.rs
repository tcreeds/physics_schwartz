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


    let mut sphere1 = Sphere::new(1.0);
    sphere1.position = Vec3::new(3.0, 0.9, 10.0);
    sphere1.velocity = Vec3::new(-0.02, 0.0, 0.0);
    sphere1.angular_velocity = Vec3::new(0.0, 0.0, 0.0);

    let mut sphere2 = Sphere::new(1.0);
    sphere2.position = Vec3::new(-3.0, 0.0, 10.0);
    sphere2.velocity = Vec3::new(0.01, 0.0, 0.0);
    sphere2.angular_velocity = Vec3::new(0.0, 0.0, 0.0);

   
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
    //rotation_axis: Vec3<f32>,
    restitution: f32,
}

//doesn't deal with rotation yet
fn resolve_collision(lhs: & mut Sphere, rhs: & mut Sphere, res: CollisionResult) -> () {
    let impulse = -(res.restitution) * na::dot(&res.relative_velocity, &res.normal);
 
    lhs.velocity = lhs.velocity + res.normal * impulse;
    rhs.velocity = rhs.velocity - res.normal * impulse;
    println!("\nnew_b: {:?}", rhs.velocity);
    println!("new_a: {:?}", lhs.velocity);
    println!("\nimp:   {:?}", impulse);
    println!("rel v: {:?}\n", res.relative_velocity);
    lhs.position = lhs.position + res.normal * (lhs.radius / (lhs.position - res.contact_point).norm() - 1.0f32);
    rhs.position = rhs.position - res.normal * (rhs.radius / (rhs.position - res.contact_point).norm() - 1.0f32);

    /*let lhs_inertia_value = 2.0f32/5.0f32 * 1.0f32/*mass*/ * lhs.radius * lhs.radius;
    let inertia_tensor_lhs = na::Mat3::new(lhs_inertia_value, 0.0, 0.0, 0.0, lhs_inertia_value, 0.0, 0.0, 0.0, lhs_inertia_value);
    let rhs_inertia_value = 2.0f32/5.0f32 * 1.0f32/*mass*/ * lhs.radius * lhs.radius;
    let inertia_tensor_rhs = na::Mat3::new(rhs_inertia_value, 0.0, 0.0, 0.0, rhs_inertia_value, 0.0, 0.0, 0.0, rhs_inertia_value);

    lhs.angular_velocity = lhs.angular_velocity + na::cross(&res.rotation_axis, &(res.normal * impulse)) * inertia_tensor_lhs;
    rhs.angular_velocity = rhs.angular_velocity - na::cross(&res.rotation_axis, &(res.normal * impulse)) * inertia_tensor_rhs;*/

}

fn hit_test(lhs: & Sphere, rhs: & Sphere) -> Option<CollisionResult> {

    let dist = (lhs.position - rhs.position).norm();
    if dist < lhs.radius + rhs.radius {
        //if na::dot(&a.velocity, &(b.position - a.position)) > 0.0f32 || na::dot(&b.velocity, &(a.position - b.position)) > 0.0f32 {
        
            let contact_normal = (lhs.position - rhs.position).normalize();
            let point_of_contact  = rhs.position + (lhs.position - rhs.position) / 2.0f32;
            let dot_lhs = na::dot(&lhs.velocity.clone().normalize(), &contact_normal);
            let dot_rhs = na::dot(&rhs.velocity.clone().normalize(), &contact_normal);
            let rel_lhs = if dot_lhs.is_nan(){ lhs.velocity * 0.0f32} else { lhs.velocity * dot_lhs };
            let rel_rhs = if dot_rhs.is_nan(){ rhs.velocity * 0.0f32} else { rhs.velocity * dot_rhs };
            let rel_vel = rel_rhs - rel_lhs;

            println!("norm:  {:?}", contact_normal);
            println!("rel_b: {:?}", rel_rhs);
            println!("rel_a: {:?}", rel_lhs);
            println!("rel_v: {:?}", rel_vel);

            let result = CollisionResult {
                normal: contact_normal,
                contact_point: point_of_contact,
                relative_velocity: rel_vel,
                //rotation_axis: na::cross(&rel_vel, &contact_normal),
                restitution: 1.0f32,
            };
            Some(result)

    } else {
        None
    }
}



