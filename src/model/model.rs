use super::view_item::ViewItem;

pub struct Model {
    pub items: Vec<ViewItem>,
}

impl Model {
    pub fn new(items: Vec<ViewItem>) -> Model {
        Model {
            items,
        }
    }

    pub fn find_item_by_id(&self, id_to_find: &String) -> Option<&ViewItem> {
        self.items.iter()
            .map(|item| item.find_by_id(id_to_find))
            .find(|item| item.is_some())
            .flatten()
    }
    pub fn find_width(&self, item: &ViewItem) -> u32 {
        if(item.name.len() as u32) > Model::MAX_COLUMN_WIDTH {
            Model::MAX_COLUMN_WIDTH
        } else {
            Model::MIN_COLUMN_WIDTH
        }
    }

    pub const MIN_COLUMN_WIDTH: u32 = 10;
    pub const MAX_COLUMN_WIDTH: u32 = 30;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_has_items() {
        let item1 = ViewItem::new("Item 1".to_string());
        let item2 = ViewItem::new("Item 2".to_string());
        let items = vec!(item1.clone(), item2.clone());
        let model = Model::new(items);

        assert_eq!(vec!(item1, item2), model.items);
    }

    #[test]
    fn model_can_have_no_items() {
        let items = vec!();
        let model = Model::new(items);

        assert_eq!(0, model.items.len());
    }

    #[test]
    fn model_finds_top_level_item_by_id() {
        let item1 = ViewItem::new("Item 1".to_string());
        let item2 = ViewItem::new("Item 2".to_string());
        let items = vec!(item1.clone(), item2.clone());
        let model = Model::new(items);

        assert_eq!(item1, *model.find_item_by_id(&item1.id).unwrap());
    }

    #[test]
    fn model_finds_item_by_id() {
        let mut item1 = ViewItem::new("Item 1".to_string());
        let item2 = ViewItem::new("Item 2".to_string());
        let item3 = ViewItem::new("Item 3".to_string());
        item1.add_sub_item(item3.clone());
        let items = vec!(item1.clone(), item2.clone());
        let model = Model::new(items);

        assert_eq!(item3, *model.find_item_by_id(&item3.id).unwrap());
    }

    #[test]
    fn model_determines_width_of_tiny_column_to_be_min() {
        let item1 = ViewItem::new("I1".to_string());
        let model = Model::new(vec!(item1.clone()));

        let column_width = model.find_width(&item1);

        assert_eq!(Model::MIN_COLUMN_WIDTH, column_width);
    }

    #[test]
    fn model_determines_width_of_huge_column_to_be_max() {
        let item1 = ViewItem::new("::ITEM WITH A REALLY LONG NAME THAT IS LONGER THAN THE MAX COLUMN WIDTH::".to_string());
        let model = Model::new(vec!(item1.clone()));

        let column_width = model.find_width(&item1);

        assert_eq!(Model::MAX_COLUMN_WIDTH, column_width);
    }
}