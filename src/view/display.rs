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

    pub fn format_sub_item(view_item: ViewItem, level: i32) -> String {
        let mut formatted = String::from(Self::indent(level));
        formatted.push_str(&view_item.name);
        formatted
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

    pub fn indent(level: i32) -> String {
        if level == 0 {
            String::from("")
        } else if level == 1 {
            String::from(" - ")
        } else {
            let mut indent = String::new();
            for _ in 0 .. (level-1) {
                indent.push_str("   ");
            }
            indent.push_str(" - ");
            indent
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

        let column_width = Display::find_width(&item1);

        assert_eq!(Display::MIN_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn display_determines_width_of_huge_column_to_be_max() {
        let item1 = ViewItem::new("::ITEM WITH A REALLY LONG NAME THAT IS LONGER THAN THE MAX COLUMN WIDTH::".to_string());

        let column_width = Display::find_width(&item1);

        assert_eq!(Display::MAX_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn display_determines_width_to_be_length_plus_padding() {
        let item1 = ViewItem::new("::ITEM WITH MEDIUM LENGTH::".to_string());

        let column_width = Display::find_width(&item1);

        let expected_width = item1.name.len() + Display::PADDING.len();
        assert_eq!(expected_width, column_width);
    }

    #[test]
    fn determines_width_to_increase_by_indent_per_level() {
        let mut item1 = ViewItem::new("::ITEM 1 WITH MEDIUM LENGTH::".to_string());
        let mut item2 = ViewItem::new("::ITEM 2 WITH MEDIUM LENGTH::".to_string());
        let item3 = ViewItem::new("::ITEM 3 WITH MEDIUM LENGTH::".to_string());

        item2.add_sub_item(item3);
        item1.add_sub_item(item2);


        let column_width = Display::find_width(&item1);

        let expected_width =
            Display::indent(2).len()
            + item1.name.len()
            + Display::PADDING.len();
        assert_eq!(expected_width, column_width);
    }

    #[test]
    fn indent_empty_for_zero_level() {
        let indent = Display::indent(0);

        assert_eq!("".to_string(), indent);
    }

    #[test]
    fn indent_level_1() {
        let indent = Display::indent(1);

        assert_eq!(" - ".to_string(), indent);
    }

    #[test]
    fn indent_level_3() {
        let indent = Display::indent(3);
        assert_eq!("       - ".to_string(), indent)
    }

    #[test]
    fn sub_item_displayed_with_indent() {
        let item = ViewItem::new("::SUB_ITEM OF MEDIUM LENGTH::".to_string());

        let sub_item_formatted = Display::format_sub_item(item.clone(), 3);

        assert_eq!(format!("       - {}", item.name), sub_item_formatted);
    }
}