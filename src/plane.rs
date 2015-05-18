extern crate glutin;

extern crate glium;

extern crate nalgebra as na;

use na::*;
use sphere::*;

#[derive(Clone, Copy)]
pub struct Plane {
	point: Vec3<f32>,
	normal: Vec3<f32>,
	restitution: f32,
}

impl Plane {
	pub fn new(point: Vec3<f32>, normal: Vec3<f32>, restitution: f32) -> Plane {
		Plane {
			point: point,
			normal: normal.normalize(),
			restitution: restitution,
		}
	}

	pub fn check_collision(&self, sphere: &mut Sphere) -> bool {

		//let velocity_normal = sphere.velocity.clone().normalize();

		println!("sphere's position{:?}\n", sphere.position);

		let velocity_direction = na::dot(&sphere.velocity, &self.normal);
		if velocity_direction > 0.0f32 {
			return false;
		}

		//dot product of previous frame - for tunneling purposes
		let prev_dot_product = na::dot(&(sphere.position - sphere.velocity - self.point), &self.normal);
		//current dot product
		let dot_product: f32 = na::dot(&(sphere.position - self.point), &self.normal);
		//vector from plane to center of sphere
		let projection = self.normal * dot_product;

		if velocity_direction < 0.0f32 {
			//println!("plane collision possible");
			//println!("plane's normal   : {:?}", self.normal);
			//println!("sphere's velocity: {:?}\n", sphere.velocity);
			
		}

		let distance: f32 = if projection.norm() < 0.0f32 {
			projection.norm() * -1.0f32
		} else {
			projection.norm()
		};

		//check if sphere is currently intersecting the line
		if velocity_direction <= 0.0f32 && distance < sphere.radius {
			
			//sphere is still on the positive side of the plane
			if dot_product > 0.0f32 {
				format!("distance: {}", distance);
				println!("sphere's position{:?}\n", sphere.position);
				sphere.position = sphere.position + self.normal * projection.norm() / sphere.radius;

				println!("sphere's position{:?}\n", sphere.position);
				return true;
			} else{
				sphere.position = sphere.position - projection + self.normal * sphere.radius;
				return true;
			}

		}
		//check if sphere has just tunneled through plane
		else if velocity_direction < 0.0f32 && prev_dot_product > 0.0f32 {
			sphere.position = sphere.position - projection + self.normal * sphere.radius;
			return true;
		}
		false
	}

	pub fn bounce_sphere(&self, sphere: &mut Sphere){

		println!("plane collision happening");
		println!("sphere's velocity: {:?}", sphere.velocity);
		println!("sphere's position: {:?}", sphere.position);
		println!("plane's point:     {:?}", self.point);
		
		
		sphere.velocity = -sphere.velocity * self.restitution; 
		println!("sphere's new velocity: {:?}", sphere.velocity);
		let tensor_value = 2.0f32 / 5.0f32 * sphere.mass * sphere.radius * sphere.radius;
		let inertia_tensor = na::Mat3::new(tensor_value, 0.0, 0.0, 0.0, tensor_value, 0.0, 0.0, 0.0, tensor_value);
		let inv_tensor = na::inv(&inertia_tensor).unwrap();
		let vradius = -self.normal * sphere.radius;
		let numerator = -(1.0f32 + self.restitution) * na::dot(&sphere.velocity, &self.normal);
		let denominator = 1.0f32 / sphere.mass + na::dot(&na::cross(&(inv_tensor * na::cross(&vradius, &self.normal)), &vradius), &self.normal);
		let impulse =  self.normal * (numerator / denominator); 

		let perp = sphere.velocity.clone().normalize() * sphere.radius;

		sphere.angular_velocity = sphere.angular_velocity - na::cross(&perp, &impulse);

	}
}