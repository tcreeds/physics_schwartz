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
            Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]);
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        
        let program = glium::Program::from_source(&display,
                // vertex shader
                "   #version 110

                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec3 color;

                varying vec3 v_color;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    v_color = color;
                }
                ",

                // fragment shader
                "   #version 110
                varying vec3 v_color;

                void main() {
                    gl_FragColor = vec4(v_color, 1.0);
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

