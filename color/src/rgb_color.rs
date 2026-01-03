#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn to_css_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use crate::RGBColor;

    #[test]
    fn css_string() {
        let color = RGBColor::black();
        assert_eq!(color.to_css_string(), "#000000");

        let color = RGBColor::white();
        assert_eq!(color.to_css_string(), "#ffffff");

        let color = RGBColor::new(64, 128, 32);
        assert_eq!(color.to_css_string(), "#408020");
    }
}
