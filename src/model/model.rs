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
        self.items.iter().find(|item| item.id == *id_to_find)
    }
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
    fn model_finds_item_by_id() {
        let mut item1 = ViewItem::new("Item 1".to_string());
        let item2 = ViewItem::new("Item 2".to_string());
        let item3 = ViewItem::new("Item 3".to_string());
        item1.add_sub_item(item3.clone());
        let items = vec!(item1.clone(), item2.clone());
        let model = Model::new(items);

        assert_eq!(Some(&item3), model.find_item_by_id(&item3.id));
    }
}