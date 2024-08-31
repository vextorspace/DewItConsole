use crate::model::ViewItem;

pub struct Column<'a> {
    pub top_item: &'a ViewItem,
    pub index: usize,
}



impl<'a> Column<'a> {
    pub fn new(top_item: &ViewItem, index: usize) -> Column {
        Column {
            top_item,
            index,
        }
    }

    pub fn find_level(&self, view_item: &ViewItem) -> Option<usize> {
        if self.top_item == view_item {
            Some(0)
        } else {
            None
        }
    }

    pub fn format_sub_item(view_item: ViewItem, level: usize) -> String {
        let mut formatted = String::from(Self::indent(level));
        formatted.push_str(&view_item.name);
        formatted
    }

    pub fn find_width(item: &ViewItem, level: usize) -> usize {
        if item.name.len() > Column::MAX_COLUMN_WIDTH {
            Column::MAX_COLUMN_WIDTH
        } else if item.name.len() < Column::MIN_COLUMN_WIDTH {
            Column::MIN_COLUMN_WIDTH
        } else {
            Column::indent(level).len() + item.name.len() + Column::PADDING.len()
        }
    }

    pub fn indent(level: usize) -> String {
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
    use super::*;

    #[test]
    fn determines_width_of_tiny_column_to_be_min() {
        let item1 = ViewItem::new("I1".to_string());

        let column_width = Column::find_width(&item1, 0);

        assert_eq!(Column::MIN_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn determines_width_of_huge_column_to_be_max() {
        let item1 = ViewItem::new("::ITEM WITH A REALLY LONG NAME THAT IS LONGER THAN THE MAX COLUMN WIDTH::".to_string());

        let column_width = Column::find_width(&item1, 0);

        assert_eq!(Column::MAX_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn determines_width_to_be_length_plus_padding() {
        let item1 = ViewItem::new("::ITEM WITH MEDIUM LENGTH::".to_string());

        let column_width = Column::find_width(&item1, 0);

        let expected_width = item1.name.len() + Column::PADDING.len();
        assert_eq!(expected_width, column_width);
    }

    #[test]
    fn determines_width_to_increase_by_indent_per_level() {
        let mut item1 = ViewItem::new("::ITEM 1 WITH MEDIUM LENGTH::".to_string());
        let mut item2 = ViewItem::new("::ITEM 2 WITH MEDIUM LENGTH::".to_string());
        let item3 = ViewItem::new("::ITEM 3 WITH MEDIUM LENGTH::".to_string());

        item2.add_sub_item(item3.clone());
        item1.add_sub_item(item2);

        let column = Column::new(&item1, 0);
        let level = column.find_level(&item3).unwrap();
        let column_width = Column::find_width(&item3, level);

        let expected_width =
            Column::indent(2).len()
                + item1.name.len()
                + Column::PADDING.len();
        assert_eq!(expected_width, column_width);
    }



    #[test]
    fn indent_empty_for_zero_level() {
        let indent = Column::indent(0);

        assert_eq!("".to_string(), indent);
    }

    #[test]
    fn indent_level_1() {
        let indent = Column::indent(1);

        assert_eq!(" - ".to_string(), indent);
    }

    #[test]
    fn indent_level_3() {
        let indent = Column::indent(3);
        assert_eq!("       - ".to_string(), indent)
    }

    #[test]
    fn sub_item_displayed_with_indent() {
        let item = ViewItem::new("::SUB_ITEM OF MEDIUM LENGTH::".to_string());

        let sub_item_formatted = Column::format_sub_item(item.clone(), 3);

        assert_eq!(format!("       - {}", item.name), sub_item_formatted);
    }

    #[test]
    fn finds_sub_level_for_self() {
        let item = ViewItem::new("::ANY::".to_string());
        let column = Column::new(&item, 0);

        let level = column.find_level(&item);

        assert_eq!(Some(0), level);
    }

    #[test]
    fn finds_sub_level_none_if_not_there() {
        let item1 = ViewItem::new("::ANY::".to_string());
        let missing_item = ViewItem::new("::OTHER::".to_string());

        let column = Column::new(&item1, 0);

        let level = column.find_level(&missing_item);

        assert_eq!(None, level);
    }
}

