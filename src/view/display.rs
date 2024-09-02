use crate::model::Model;
use crate::view::column::Column;
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

    pub fn write_column(&self, column: &Column) -> Result<(), io::Error> {
        todo!()
    }

    fn append_item_ending(&mut self, loop_size: usize, index: usize) -> Result<(), Error> {
        if index < loop_size - 1 {
            write!(self.writer, "{}", Column::PADDING)?;
        } else {
            writeln!(self.writer)?;
        }
        Ok(())
    }
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
    fn writes_a_column() {
        let mut item1 = ViewItem::new("Item 1".to_string());
        let mut item2 = ViewItem::new("Item 2".to_string());
        let mut child1 = ViewItem::new("Child 1".to_string());
        let child2 = ViewItem::new("Child 2".to_string());
        let grandchild = ViewItem::new("Grandchild".to_string());
        child1.add_sub_item(grandchild.clone());
        item1.add_sub_item(child1.clone());
        item2.add_sub_item(child2.clone());
        let model = Model {
            items: vec!(item1, item2),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let mut display = Display::new(&mut buffer);
        let col1 = &model.columns()[0];
        let width = col1.width();
        let height = col1.height();

        display.write_column(col1).unwrap();

        let expected = "Item 1           | \n - Child 1       | \n    - Grandchild | \n";
        let result = String::from_utf8_lossy(&buffer);
        assert_eq!(expected.to_string(), result);


    }
}