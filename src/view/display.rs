use crate::model::Model;
use std::io;

pub struct Display<'a> {
    writer: &'a mut dyn io::Write,
}

impl<'a> Display<'a> {
    pub fn initialize(&mut self, model: &Model) -> Result<(), io::Error> {
        writeln!(self.writer, "Dew It!")?;

        let loop_size = model.items.len();

        for (index, item) in model.items.iter().enumerate() {
            write!(self.writer, "{}", item.name)?;
            if index < loop_size - 1 {
                write!(self.writer, " | ")?;
            } else {
                writeln!(self.writer)?;
            }
        }

        Ok(())
    }

    pub fn new(console: &'a mut dyn io::Write) -> Display {
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

        display.initialize(&model).unwrap();

        let expected = "Dew It!\n";
        assert_eq!(expected.to_string(), String::from_utf8_lossy(&buffer))
    }
}