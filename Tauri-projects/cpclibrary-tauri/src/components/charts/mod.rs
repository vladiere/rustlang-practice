mod bar_chart;
mod line_chart;
mod pie_chart;

pub use bar_chart::*;
pub use line_chart::*;
pub use pie_chart::*;

pub struct MyData {
    x: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    y4: f64,
    y5: f64,
}

impl MyData {
    pub fn new(x: f64, y1: f64, y2: f64, y3: f64, y4: f64, y5: f64) -> Self {
        Self {
            x,
            y1,
            y2,
            y3,
            y4,
            y5,
        }
    }
}
