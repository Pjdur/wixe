use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rusttype::{Font, Scale, point};
use std::fs::File;
use std::io::Read;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    env_logger::init();

    // Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wixe GUI Framework")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Load a font (ensure this file exists in the path)
    let mut font_data = Vec::new();
    File::open("assets/Roboto-Regular.ttf")
        .expect("Font file not found")
        .read_to_end(&mut font_data)
        .unwrap();
    let font = Font::try_from_vec(font_data).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                // Clear screen
                for pixel in pixels.get_frame().chunks_exact_mut(4) {
                    pixel[0] = 240;
                    pixel[1] = 240;
                    pixel[2] = 240;
                    pixel[3] = 255;
                }

                // Render text
                draw_text(
                    pixels.get_frame(),
                    WIDTH,
                    HEIGHT,
                    "Welcome to Wixe",
                    &font,
                    48.0,
                    (WIDTH / 2, HEIGHT / 2),
                );

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => pixels.resize_surface(size.width, size.height).unwrap(),
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

/// Draw text centered at (cx, cy)
fn draw_text(
    frame: &mut [u8],
    width: u32,
    height: u32,
    text: &str,
    font: &Font,
    font_size: f32,
    (cx, cy): (u32, u32),
) {
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0 + v_metrics.ascent)).collect();

    let width_text: i32 = glyphs
        .last()
        .map(|g| g.position().x as i32 + g.unpositioned().h_metrics().advance_width as i32)
        .unwrap_or(0);

    let x_offset = cx as i32 - width_text / 2;
    let y_offset = cy as i32 + (font_size / 2.0) as i32;

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let x = gx as i32 + bb.min.x + x_offset;
                let y = gy as i32 + bb.min.y + y_offset;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let idx = ((y as u32) * width + (x as u32)) as usize * 4;
                    frame[idx] = (0.0 * (1.0 - gv) + 0.0 * gv) as u8;
                    frame[idx + 1] = (0.0 * (1.0 - gv) + 0.0 * gv) as u8;
                    frame[idx + 2] = (0.0 * (1.0 - gv) + 0.0 * gv) as u8;
                    frame[idx + 3] = (255.0 * gv) as u8;
                }
            });
        }
    }
}
