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
        self.top_item.find_level(view_item)
    }

    pub fn format_sub_item(view_item: ViewItem, level: usize) -> String {
        let mut formatted = String::from(Self::indent(level));
        formatted.push_str(&view_item.name);
        formatted
    }

    pub fn find_total_width(&self) -> usize {
        let mut max_width = 0;

        for item in self.iter() {
            let level = self.find_level(item).unwrap();
            let width = item.find_width(level);
            if width > max_width {
                max_width = width;
            }
        }
        max_width
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
    fn iterates_over_all_levels() {
        let mut item = ViewItem::new("::ANY::".to_string());
        let mut child1 = ViewItem::new("::ANY::".to_string());
        let grandchild = ViewItem::new("::ANY::".to_string());
        let child2 = ViewItem::new("::ANY::".to_string());
        child1.add_sub_item(grandchild.clone());
        item.add_sub_item(child1.clone());
        item.add_sub_item(child2.clone());
        let column = Column::new(&item, 0);

        let mut items = column.iter();
        assert_eq!(Some(&item), items.next());
        assert_eq!(Some(&child1), items.next());
        assert_eq!(Some(&grandchild), items.next());
        assert_eq!(Some(&child2), items.next());
        assert_eq!(None, items.next());
    }


    #[test]
    fn determine_column_width_as_max_entry_width() {
        let (item1, item2, item3) = setup_3_entry_column();
        let column = Column::new(&item1, 0);

        let column_width = column.find_total_width();
        let expected_width = calculate_max_width_of_items(&item1, &item2, &item3, column);
        assert_eq!(expected_width, column_width);
    }

    fn calculate_max_width_of_items(item1: &ViewItem, item2: &ViewItem, item3: &ViewItem, column: Column) -> usize {
        let widths = vec!(item1.find_width(column.find_level(&item1).unwrap()),
                          item2.find_width(column.find_level(&item2).unwrap()),
                          item3.find_width(column.find_level(&item3).unwrap()));
        let expected_width = *widths.iter().max().unwrap();
        expected_width
    }

    fn setup_3_entry_column() -> (ViewItem, ViewItem, ViewItem) {
        let mut item1 = ViewItem::new("::ITEM 1 WITH MEDIUM LENGTH::".to_string());
        let mut item2 = ViewItem::new("::ITEM 2 WITH MEDIUM LENGTH::".to_string());
        let item3 = ViewItem::new("::ITEM 3 WITH MEDIUM LENGTH::".to_string());

        item2.add_sub_item(item3.clone());
        item1.add_sub_item(item2.clone());

        (item1, item2, item3)
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

    #[test]
    fn finds_sub_level_for_sub_item() {
        let mut item = ViewItem::new("::ANY::".to_string());
        let mut child = ViewItem::new("::ANY::".to_string());
        let grandchild = ViewItem::new("::ANY::".to_string());

        child.add_sub_item(grandchild.clone());
        item.add_sub_item(child.clone());

        let mut column = Column::new(&item, 0);
        let mut level = column.find_level(&child);

        assert_eq!(Some(1), level);

        column = Column::new(&item, 0);
        level = column.find_level(&grandchild);

        assert_eq!(Some(2), level);
    }
}

