use crate::model::{Model, ViewItem};
use std::io;
use std::io::Error;

pub struct Display<'a> {
    writer: &'a mut dyn io::Write,
}

impl<'a> Display<'a> {
    pub fn new(console: &'a mut dyn io::Write) -> Display {
        Display {
            writer: console,
        }
    }

    pub fn initialize(&mut self, model: &Model) -> Result<(), io::Error> {
        writeln!(self.writer, "Dew It!")?;

        self.write_items_to_display(model)?;

        Ok(())
    }

    fn write_items_to_display(&mut self, model: &Model) -> Result<(), Error> {
        let loop_size = model.items.len();

        for (index, item) in model.items.iter().enumerate() {
            write!(self.writer, "{}", item.name)?;
            self.append_item_ending(loop_size, index)?;
        }
        Ok(())
    }

    fn append_item_ending(&mut self, loop_size: usize, index: usize) -> Result<(), Error> {
        if index < loop_size - 1 {
            write!(self.writer, "{}", Display::PADDING)?;
        } else {
            writeln!(self.writer)?;
        }
        Ok(())
    }

    pub fn find_width(item: &ViewItem) -> usize {
        if item.name.len() > Display::MAX_COLUMN_WIDTH {
            Display::MAX_COLUMN_WIDTH
        } else if item.name.len() < Display::MIN_COLUMN_WIDTH {
            Display::MIN_COLUMN_WIDTH
        } else {
            item.name.len() + Display::PADDING.len()
        }
    }

    pub const MIN_COLUMN_WIDTH: usize = 10;
    pub const MAX_COLUMN_WIDTH: usize = 30;
    pub const PADDING: &'static str = " | ";
}

#[cfg(test)]
mod tests {
    use crate::model::ViewItem;
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


    #[test]
    fn display_determines_width_of_tiny_column_to_be_min() {
        let item1 = ViewItem::new("I1".to_string());
        let model = Model::new(vec!(item1.clone()));

        let column_width = Display::find_width(&item1);

        assert_eq!(Display::MIN_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn model_determines_width_of_huge_column_to_be_max() {
        let item1 = ViewItem::new("::ITEM WITH A REALLY LONG NAME THAT IS LONGER THAN THE MAX COLUMN WIDTH::".to_string());
        let model = Model::new(vec!(item1.clone()));

        let column_width = Display::find_width(&item1);

        assert_eq!(Display::MAX_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn model_determines_width_to_be_length_plus_padding() {
        let item1 = ViewItem::new("::ITEM WITH MEDIUM LENGTH::".to_string());
        let model = Model::new(vec!(item1.clone()));

        let column_width = Display::find_width(&item1);

        assert_eq!(item1.name.len()+Display::PADDING.len(), column_width);
    }
}