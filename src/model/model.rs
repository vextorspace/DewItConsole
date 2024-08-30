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
}