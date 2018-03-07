#[macro_use]
extern crate glium;//import glium and its macros

#[path = "teapot.rs"]
mod teapot;//load teapot model file as a module

#[path = "shaders.rs"]
mod shaders;

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    use glium::{glutin, Surface};//bring glutin and surface into local scope

    //inits
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Glium Test");
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24);//24 bit depth buffer
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    //load teapot into a useable form
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                            &teapot::INDICES).unwrap();

//shaders were here

    //load shaders into glium
    let program = glium::Program::from_source(&display, &shaders::VSH_BLINN_PHONG, &shaders::FSH_BLINN_PHONG,
                                            None).unwrap();

   /* let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 2.0, 1.0f32],
    ];*/

    let mut t: f32 = 1.0;
    //draw and react to window events
    let mut closed = false;
    while !closed {
        t = t + 0.001;
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


/*
        let model = [//still object
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0 , 0.0, 2.0, 1.0f32],
            ];
*/
/*
        let model = [//rotate z axis
            [ t.cos()/100.0, t.sin()/100.0, 0.0, 0.0],
            [-t.sin()/100.0, t.cos()/100.0, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];
*/
/*
        let model = [//rotate x axis
            [ 0.01, 0.0, 0.0, 0.0],
            [0.0, t.cos()/100.0, -t.sin()/100.0, 0.0],
            [0.0, t.sin()/100.0, t.cos()/100.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];
*/

        let model = [//rotate y axis
            [ t.cos()/100.0, 0.0, t.sin()/100.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [-t.sin()/100.0, 0.0, t.cos()/100.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let model2 = [
            [ t.cos()/100.0, t.sin()/100.0, 0.0, 0.0],
            [-t.sin()/100.0, t.cos()/100.0, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [2.0, 0.0, 2.0, 1.0f32],
        ];

        let model3 = [//rotate x axis
            [ 0.01, 0.0, 0.0, 0.0],
            [0.0, t.cos()/100.0, -t.sin()/100.0, 0.0],
            [0.0, t.sin()/100.0, t.cos()/100.0, 0.0],
            [-2.0, 0.0, 2.0, 1.0f32],
        ];

        //view (camera) matrix. massive pain to edit
        let view = view_matrix(&[0.0, 1.0, 0.0], &[0.0, -0.5, 1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
                let (width, height) = target.get_dimensions();
                let aspect_ratio = height as f32 / width as f32;

                let fov: f32 = 3.141592 / 3.0;
                let zfar = 1024.0;
                let znear = 0.1;

                let f = 1.0 / (fov / 2.0).tan();

                [
                    [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                    [         0.0         ,     f ,              0.0              ,   0.0],
                    [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                    [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                ]
        };

        let light = [t.cos()/100.0, t.sin()/100.0, 0.0f32];//light position
        let light2 = [t.cos()/100.0, -t.sin()/100.0, t.cos()/100f32];//light position
        let stilllight = [0.0, 0.5, 0.0f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: view, perspective: perspective, u_light: stilllight, gloss: 128.0f32 },
                    &params).unwrap();
        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model2, view: view, perspective: perspective, u_light: light2, gloss: 16.0f32 },
                    &params).unwrap();
        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model3, view: view, perspective: perspective, u_light: light, gloss: 8.0f32 },
                    &params).unwrap();
        target.finish().unwrap();

        //react to window events
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
