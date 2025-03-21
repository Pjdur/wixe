// Add these types to your existing code
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

// Base text widget
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
    fn handle_event(&mut self, _: &Event) -> bool {
        false
    }

    fn update(&mut self, _: &mut UpdateContext) -> bool {
        false
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        // For now, return a fixed size based on font size
        Size {
            width: self.font_size * 10.0, // Rough estimate
            height: self.font_size * 1.2, // Rough estimate
        }
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
    fn handle_event(&mut self, event: &Event) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
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
    fn handle_event(&mut self, event: &Event) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
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
    fn handle_event(&mut self, event: &Event) -> bool {
        self.base.handle_event(event)
    }

    fn update(&mut self, ctx: &mut UpdateContext) -> bool {
        self.base.update(ctx)
    }

    fn layout(&mut self, constraints: &LayoutConstraint) -> Size {
        self.base.layout(constraints)
    }
}

// Example usage
fn main() {
    let heading = Heading::new("Main Title".to_string(), 1);
    let subheading = Subheading::new("Sub Title".to_string());
    let paragraph = Paragraph::new("This is a paragraph of text that will be justified.".to_string());
    
    println!("Created text widgets successfully!");
}