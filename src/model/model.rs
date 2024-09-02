use super::view_item::ViewItem;
use crate::view::Column;

pub struct Model {
    pub items: Vec<ViewItem>,
}

impl Model {
    pub fn new(items: Vec<ViewItem>) -> Model {
        Model {
            items,
        }
    }

    pub fn columns(&self) -> Vec<Column> {
        self.items.iter()
            .enumerate()
            .map(|(index, item)| Column::new(item, index))
            .collect()
    }

    pub fn find_item_by_id(&self, id_to_find: &String) -> Option<&ViewItem> {
        self.items.iter()
            .map(|item| item.find_by_id(id_to_find))
            .find(|item| item.is_some())
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_items() {
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
    fn has_all_items_in_columns() {
        let mut item1 = ViewItem::new("Item 1".to_string());
        let mut item2 = ViewItem::new("Item 2".to_string());
        let mut child1 = ViewItem::new("Child 1".to_string());
        let child2 = ViewItem::new("Child 2".to_string());
        let grandchild = ViewItem::new("Grandchild".to_string());
        child1.add_sub_item(grandchild.clone());
        item1.add_sub_item(child1.clone());
        item2.add_sub_item(child2.clone());
        let items = vec!(item1.clone(), item2.clone());
        let model = Model::new(items);

        let columns: Vec<Column> = model.columns();

        assert_eq!(2, columns.len());
        assert_eq!(vec!(&item1, &child1, &grandchild), columns[0].iter().collect::<Vec<_>>());
        assert_eq!(vec!(&item2, &child2), columns[1].iter().collect::<Vec<_>>());

        assert_eq!(vec!(item1, item2), model.items);
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
}