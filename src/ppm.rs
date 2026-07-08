use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color(x, y, z)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self {
        Color((self.0 * rhs), (self.1 * rhs), (self.2 * rhs))
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Self {
        Color(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        )
    }
}

pub fn init_ppm(width: u16, height: u16) -> std::io::Result<BufWriter<File>>{
    let mut file = File::create("render.ppm")?;
    let mut writer = BufWriter::new(file);

    // configuration
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    // write
    writer.flush()?;
    Ok(writer)
}

pub fn push_pixel(col: Color, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    writeln!(writer, "{} {} {}", (255.99 * col.0) as u8, (255.99 * col.1) as u8, (255.99 * col.2) as u8)?;
    Ok(())
}