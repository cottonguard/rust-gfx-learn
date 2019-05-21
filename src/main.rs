use gfx_learn::frame_count::*;

#[macro_use]
extern crate gfx;

use gfx::traits::*;
use gfx_window_glutin as gfx_glutin;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0., 0., 0., 1.];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title("First Gfx")
        .with_dimensions(glutin::dpi::LogicalSize::new(640., 480.));
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(wb, ctx, &events_loop)
        .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/first_150.glslv")),
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/first_150.glslf")),
        pipe::new()
    ).unwrap();

    const CYAN: [f32; 3] = [0., 1., 1.];
    const GREEN: [f32; 3] = [0., 1., 0.];
    const BLUE: [f32; 3] = [0., 0., 1.];
    const PINK: [f32; 3] = [1., 0., 1.];
    let vertices = vec![
        Vertex { pos: [0.5, -0.5], color: CYAN },
        Vertex { pos: [-0.5, -0.5], color: GREEN },
        Vertex { pos: [-0.5, 0.5], color: BLUE },
        Vertex { pos: [0.5, 0.5], color: PINK },
    ];

    let indices = vec![0u32, 1, 2, 2, 3, 0];

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
        &vertices, indices.as_slice()
    );

    let mut mat = Transform {
        transform: [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ]
    };

    let mat2 = Transform {
        transform: [
            [0., 1., 0., 0.],
            [1., 0., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]
        ]
    };

    let transform_buffer = factory.create_constant_buffer(1);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        transform: transform_buffer,
        out: main_color
    };

    let _ = encoder.update_buffer(&data.transform, &[mat], 0);

    let mut running = true;
    let mut frame_count = FrameCount::new();
    frame_count.toggle_log(true);
    while running {
        use glutin::WindowEvent::*;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { window_id: _, event } => match event {
                    KeyboardInput { device_id: _, input } => {
                        if let glutin::KeyboardInput { virtual_keycode: Some(code), .. } = input {
                            use glutin::VirtualKeyCode::*;
                            let (vx, vy) = match code {
                                Left => (0.01, 0.),
                                Right => (-0.01, 0.),
                                Up => (0., -0.01),
                                Down => (0., 0.01),
                                _ => (0., 0.)
                            };

                            mat.transform[0][3] += vx;
                            mat.transform[1][3] += vy;
                            let _ = encoder.update_buffer(&data.transform, &[mat], 0);

                            match code {
                                Z => { let _ = encoder.update_buffer(&data.transform, &[mat2], 0); }
                                _ => {}
                            }
                       }
                    },
                    MouseInput {..} => {},
                    CursorMoved {
                        device_id: _,
                        position: glutin::dpi::LogicalPosition{ x, y },
                        modifiers: _ 
                    } => {
                        mat.transform[0][3] = (x / 1000.) as f32;
                        mat.transform[1][3] = (-y / 1000.) as f32;
                        println!("x: {}, y: {}", x, y);
                    },
                    CloseRequested => { 
                        println!("close requested");
                        running = false;
                    },
                    _ => {}
                },
                _ => {
                    // dbg!(event);
                }
            }
        });

        encoder.clear(&data.out, BLACK);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        frame_count.update();
    }
}