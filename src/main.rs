pub trait Widget {
    fn handle_event(&mut self, event: &Event) -> bool;
    fn update(&mut self, ctx: &mut UpdateContext) -> bool;
    fn layout(&mut self, constraints: &LayoutConstraint) -> Size;
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyCode {
    Space,
    Enter,
    Backspace,
    Char(char),
    // Add more key codes as needed
}

#[derive(Debug, Default)]
pub struct UpdateContext {}

#[derive(Debug, Default)]
pub struct LayoutConstraint {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

#[derive(Debug, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub enum Event {
    Click(Point),
    Hover(Point),
    KeyPress(KeyCode),
    FocusChange(bool),
    MouseWheel(f32),
}

pub struct Button {
    pub label: String,
    pub on_click: Option<Box<dyn Fn()>>,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl Button {
    pub fn new(label: String) -> Self {
        Self {
            label,
            on_click: None,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self 
    where F: Fn() + 'static {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl Widget for Button {
    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Click(_) => {
                if let Some(on_click) = &self.on_click {
                    on_click();
                }
                true
            },
            Event::Hover(_) => {
                self.is_hovered = true;
                true
            },
            _ => false,
        }
    }

    fn update(&mut self, _: &mut UpdateContext) -> bool {
        false
    }

    fn layout(&mut self, _: &LayoutConstraint) -> Size {
        Size {
            width: 100.0,
            height: 30.0,
        }
    }
}

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
    fn handle_event(&mut self, event: &Event) -> bool {
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
        let mut max_width = 0.0_f32;  // Specify f32 type explicitly
        
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
}

fn main() {
    let button = Button::new("Click me".to_string())
        .with_on_click(|| println!("Button clicked!"));
    
    let container = Container::new()
        .add_child(button);
}