// module for all shape code, refactored to be a bit simpler than the original implementation.

pub enum Shape {
    Circle{radius: f32},
    Rectangle{width: f32, height: f32},
}

impl Shape {
    pub fn circle(radius: f32) -> Self {
        Self::Circle{radius} // note use of more compact less formal notation here
    }

    pub fn rectangle(width: f32, height: f32) -> Self {
        Self::Rectangle{width, height} // note use of more compact less formal notation here
    }

    pub fn iam(&self) {
        match self {
            Shape::Circle{..} => println!("I am a circle!"),
            Shape::Rectangle{..} => println!("I am a rectangle!!"),
        }
    }
}
