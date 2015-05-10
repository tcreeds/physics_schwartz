extern crate glutin;
#[macro_use]
extern crate glium;

extern crate nalgebra as na;

use na::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);


#[derive(Clone, Copy)]
struct Sphere {
    radius: f32,
    position: Vec3<f32>,
    velocity: Vec3<f32>,
    rotation: Rot3<f32>,
    angular_velocity: Vec3<f32>,
}

impl Sphere {
    fn new(radius: f32) -> Sphere {
        Sphere {
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
            .map(|a| a as f32 / 180.0f32 * std::f32::consts::PI );
        let next_y_angle_iter =(1..y_subdivs + 1)
            .map(|a| a * (90 / y_subdivs) )
            .map(|a| a as f32 / 180.0f32 * std::f32::consts::PI );

        for (y_angle, next_y_angle) in y_angle_iter.zip(next_y_angle_iter)
        {
            let y_dist = y_angle.cos() * self.radius;
            let xz_dist = y_angle.sin() * self.radius;

            let next_y_dist = next_y_angle.cos() * self.radius;
            let next_xz_dist = next_y_angle.sin() * self.radius;

            let xz_angle_iter = (0..x_subdivs)
                .map(|a| a * 180 / x_subdivs )
                .map(|a| a as f32 / 180.0 * std::f32::consts::PI );
            let next_xz_angle_iter = (1..x_subdivs + 1)
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
    fn into_buffer<F>(&self, display: &F, subdivs: u32) -> glium::VertexBuffer<Vertex> where F: glium::backend::Facade {
        glium::VertexBuffer::new(display, self.into_vertex_list(subdivs))
    }
    // until here

    fn update(&mut self) {
        self.rotation = Rot3::new(self.angular_velocity) * self.rotation;
        self.position = self.position + self.velocity;
    }
    fn get_homogeneous(&self) -> na::Mat4<f32> {
        na::Iso3::new_with_rotmat(self.position, self.rotation).to_homogeneous()
    }
}


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
    sphere1.position = Vec3::new(3.0, 1.0, 10.0);
    sphere1.velocity = Vec3::new(-0.02, 0.0, 0.0);
    sphere1.angular_velocity = Vec3::new(0.01, 0.0, 0.0);

    let mut sphere2 = Sphere::new(1.0);
    sphere2.position = Vec3::new(-3.0, 0.0, 10.0);
    sphere2.angular_velocity = Vec3::new(0.0, 0.0, 0.1);

   
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
            let mut test_iter = pair_list.iter().enumerate();

            'iter_loop: loop {
                let (l_index, & (ref lhs, _, _)) = match test_iter.next() {
                    Some(x) => x,
                    None => break 'iter_loop,
                };
                for (r_index, & (ref rhs, _, _)) in test_iter.clone()
                {
                    match hit_test(lhs, rhs){
                        Some(x) => {
                            update_index_list.push((l_index, r_index, x));

                        },
                        _ => ()
                    }
                }
            }
            update_index_list
        };

        for (li, ri, result) in color_update {

            let & mut (ref mut s1, _, ref mut c1) = pair_list.get_mut(li).unwrap();
            *c1 = Vec3::new(0.0, 1.0, 0.0);
            let & mut (ref mut s2, _, ref mut c2) = pair_list.get_mut(ri).unwrap();
            *c2 = Vec3::new(0.0, 1.0, 0.0);
            resolve_collision(s1, s2, result);
            
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


struct CollisionResult {
    normal: Vec3<f32>,
    contact_point: Vec3<f32>,
    relative_velocity: Vec3<f32>,
    relative_perp_velocity: Vec3<f32>,
    restitution: f32,
}

//doesn't deal with rotation yet
fn resolve_collision(a: & mut Sphere, b: & mut Sphere, res: CollisionResult) -> () {
    let impulse = (-(res.restitution + 1.0f32) * na::dot(&res.relative_velocity, &res.normal));

    a.velocity = a.velocity + res.normal * impulse;
    b.velocity = b.velocity - res.normal * impulse;

    a.angular_velocity = a.angular_velocity + res.relative_perp_velocity;
    b.angular_velocity = b.angular_velocity - res.relative_perp_velocity;
}

fn hit_test(a: & Sphere, b: & Sphere) -> Option<CollisionResult> {

    let dist = (a.position - b.position).norm();
    if dist <= a.radius + b.radius {
        let point_of_contact  = (a.position - b.position) / 2.0f32;
        let a_along_normal = point_of_contact - a.position;
        let b_along_normal = point_of_contact - b.position;
        let rel_a = (a_along_normal - a.position).normalize();
        let rel_b = (b_along_normal - b.position).normalize();
        let rel_v = a.velocity * na::dot(&a.velocity, &rel_a) - b.velocity * na::dot(&b.velocity, &rel_b);
        let contact_normal = (a.position - b.position).normalize();
        let result = CollisionResult {
            normal: contact_normal,
            contact_point: point_of_contact,
            relative_velocity: rel_v,
            relative_perp_velocity: na::cross(&rel_v, &contact_normal),
            restitution: 0.9f32,
        };
        Some(result)

        //let angular = ;
        //let rel_angular_velocity_a = na::cross(&a.velocity, &(-(contact_normal.clone().normalize())));
        //let rel_angular_velocity_b = na::cross(&b.velocity, &(contact_normal.clone().normalize()));


    } else {
        None
    }
}



