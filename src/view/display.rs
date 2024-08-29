use crate::model::Model;
use std::io::Write;

pub struct Display<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> Display<'a> {
    pub fn initialize(&mut self, model: &Model) {
        writeln!(self.writer, "Dew It!").unwrap();
    }

    pub fn new(console: &'a mut dyn Write) -> Display {
        Display {
            writer: console,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_captures_writes() {
        let model = Model {
            items: vec!(),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let mut display = Display::new(&mut buffer);

        display.initialize(&model);

        let expected = "Dew It!\n";
        assert_eq!(expected.to_string(), String::from_utf8_lossy(&buffer))
    }
}