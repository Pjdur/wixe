// Import required crates
extern crate winit;
extern crate glium;
extern crate glium_glyph;

use std::time::Instant;
use glium::{Display, Surface};
use glium_glyph::{GlyphBrush, GlyphBrushBuilder, SectionGeometry};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Core types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default)]
pub struct LayoutConstraint {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

// Custom event type
#[derive(Debug, Clone)]
pub enum CustomEvent {
    Click(Point),
    Hover(Point),
    KeyPress(KeyCode),
    FocusChange(bool),
    MouseWheel(f64),
}

#[derive(Debug, Copy, Clone)]
pub enum KeyCode {
    Space,
    Enter,
    Backspace,
    Char(char),
}

// Widget trait
pub trait Widget {
    fn handle_event(&mut self, event: &CustomEvent) -> bool;
    fn update(&mut self, ctx: &mut UpdateContext) -> bool;
    fn layout(&mut self, constraints: &LayoutConstraint) -> Size;
    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError>;
}

// Update context
#[derive(Debug, Default)]
pub struct UpdateContext {}

// Renderer trait
#[derive(Debug)]
pub enum RenderError {
    GraphicsError(String),
    WindowError(String),
}

pub trait Renderer {
    fn begin_frame(&mut self) -> Result<(), RenderError>;
    fn end_frame(&mut self) -> Result<(), RenderError>;
    fn clear(&mut self, color: Color);
    fn draw_text(&mut self, text: &str, position: Point, font_size: f32, color: Color, alignment: TextAlignment) -> Result<(), RenderError>;
}

// Text widget
pub struct TextWidget {
    pub text: String,
    pub font_size: f32,
    pub alignment: TextAlignment,
    pub color: Color,
}

impl TextWidget {
    pub fn new(text: String) -> Self {
        Self {
            text,
            font_size: 14.0,
            alignment: TextAlignment::Left,
            color: Color::default(),
        }
    }

    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl Widget for TextWidget {
    fn handle_event(&mut self, _: &CustomEvent) -> bool {
        false
    }

    fn update(&mut self, _: &mut UpdateContext) -> bool {
        false
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        Size {
            width: self.font_size * 10.0,
            height: self.font_size * 1.2,
        }
    }

    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError> {
        renderer.draw_text(
            &self.text,
            position,
            self.font_size,
            self.color,
            self.alignment,
        )
    }
}

// Heading widget
pub struct Heading {
    base: TextWidget,
    level: u8,
}

impl Heading {
    pub fn new(text: String, level: u8) -> Self {
        assert!(level >= 1 && level <= 6, "Heading level must be between 1 and 6");
        
        let mut base = TextWidget::new(text);
        base.font_size = match level {
            1 => 36.0,
            2 => 30.0,
            3 => 24.0,
            4 => 18.0,
            5 => 16.0,
            6 => 14.0,
            _ => unreachable!(),
        };
        
        Self { base, level }
    }
}

impl Widget for Heading {
    fn handle_event(&mut self, event: &CustomEvent) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
    }

    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError> {
        self.base.render(renderer, position)
    }
}

// Subheading widget
pub struct Subheading {
    base: TextWidget,
}

impl Subheading {
    pub fn new(text: String) -> Self {
        let mut base = TextWidget::new(text);
        base.font_size = 16.0;
        base.alignment = TextAlignment::Left;
        
        Self { base }
    }
}

impl Widget for Subheading {
    fn handle_event(&mut self, event: &CustomEvent) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
    }

    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError> {
        self.base.render(renderer, position)
    }
}

// Paragraph widget
pub struct Paragraph {
    base: TextWidget,
}

impl Paragraph {
    pub fn new(text: String) -> Self {
        let mut base = TextWidget::new(text);
        base.alignment = TextAlignment::Justify;
        
        Self { base }
    }
}

impl Widget for Paragraph {
    fn handle_event(&mut self, event: &CustomEvent) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
    }

    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError> {
        self.base.render(renderer, position)
    }
}

// Container widget
pub struct Container {
    pub children: Vec<Box<dyn Widget>>,
}

impl Container {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub fn add_child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Widget for Container {
    fn handle_event(&mut self, event: &CustomEvent) -> bool {
        for child in self.children.iter_mut() {
            if child.handle_event(event) {
                return true;
            }
        }
        false
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        let mut updated = false;
        for child in self.children.iter_mut() {
            if child.update(ctx) {
                updated = true;
            }
        }
        updated
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        let mut total_height = 0.0;
        let mut max_width = 0.0;
        
        for child in self.children.iter_mut() {
            let child_size = child.layout(constraints);
            total_height += child_size.height;
            max_width = max_width.max(child_size.width);
        }
        
        Size {
            width: max_width,
            height: total_height,
        }
    }

    fn render(&mut self, renderer: &mut dyn Renderer, position: Point) -> Result<(), RenderError> {
        let mut current_pos = position;
        for child in self.children.iter_mut() {
            child.render(renderer, current_pos)?;
            current_pos.y += child.layout(&LayoutConstraint::default()).height;
        }
        Ok(())
    }
}

// Glium Renderer implementation
pub struct GliumRenderer<'a, T: glium::SurfaceType + glium::ResizeableSurface> {
    display: glium::Display,
    glyph_brush: glium_glyph::GlyphBrush<'a, T>,
}

impl<'a, T: glium::SurfaceType + glium::ResizeableSurface> GliumRenderer<'a, T> {
    pub fn new(display: &glium::Display) -> Result<Self, RenderError> {
        let glyph_brush = glium_glyph::GlyphBrushBuilder::using_font(
            include_str!("../assets/Roboto-Regular.ttf"),
            display,
        )?;
        
        Ok(Self {
            display: display.clone(),
            glyph_brush,
        })
    }
}

impl<'a, T: glium::SurfaceType + glium::ResizeableSurface> Renderer for GliumRenderer<'a, T> {
    fn begin_frame(&mut self) -> Result<(), RenderError> {
        self.display.draw(
            Default::default(),
            |target| {
                target.clear(Some([
                    240.0 / 255.0,
                    240.0 / 255.0,
                    240.0 / 255.0,
                    1.0,
                ]));
                self.glyph_brush.draw_queued(
                    target,
                    &self.display,
                )?;
                Ok(())
            },
        )?;
        Ok(())
    }

    fn end_frame(&mut self) -> Result<(), RenderError> {
        self.display.swap_buffers()?;
        Ok(())
    }

    fn clear(&mut self, color: Color) {
        self.display.draw(Default::default(), |target| {
            target.clear(Some([
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            ]));
        }).unwrap();
    }

    fn draw_text(&mut self, text: &str, position: Point, font_size: f32, color: Color, alignment: TextAlignment) -> Result<(), RenderError> {
        self.glyph_brush.queue_text(
            text,
            Point {
                x: position.x,
                y: position.y,
            },
            [color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0],
            font_size,
            alignment,
        )?;
        Ok(())
    }
}

// Main application
pub fn run_app() -> Result<(), RenderError> {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Wixe GUI Framework")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
    
    let windowed_context = glium::glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();
    
    let display = unsafe { windowed_context.make_current() };
    let mut renderer = GliumRenderer::new(&display)?;
    
    let mut root = Container::new()
        .add_child(Heading::new("Welcome to Wixe".to_string(), 1))
        .add_child(Subheading::new("A modern GUI framework".to_string()))
        .add_child(Paragraph::new("This is a paragraph of text rendered by Wixe.".to_string()));
    
    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            },
            winit::event::Event::MainEventsCleared => {
                let now = Instant::now();
                if now.duration_since(last_frame).as_millis() >= 16 {
                    renderer.begin_frame().unwrap();
                    
                    root.render(&mut renderer, Point { x: 20.0, y: 20.0 }).unwrap();
                    
                    renderer.end_frame().unwrap();
                    last_frame = now;
                }
            }
            _ => (),
        }
    });
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error running app: {:?}", e);
    }
}