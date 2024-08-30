use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ViewItem {
    pub name: String,
    pub sub_items: Vec<ViewItem>,
    pub id: String,
}

impl ViewItem {
    pub fn new(name: String) -> ViewItem {
        ViewItem {
            name: name.to_string(),
            sub_items: vec!(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn with_id(name: String, id: String) -> ViewItem {
        ViewItem {
            name: name.to_string(),
            sub_items: vec!(),
            id,
        }
    }

    pub fn add_sub_item(&mut self, item_to_add: ViewItem) {
        self.sub_items.push(item_to_add)
    }

    pub fn find_by_id(&self, id_to_find: &String) -> Option<&ViewItem> {
        if(self.id == *id_to_find) {
            Some(self)
        } else {
            None
        }
    }
}

impl PartialEq for ViewItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ViewItem {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_item_has_name() {
        let name = "::ITEM_NAME::";
        let item = ViewItem::new(name.to_string());
        assert_eq!(item.name, name);
    }

    #[test]
    fn view_item_can_have_sub_items() {
        let mut item1 = ViewItem::new("Item 1".to_string());
        let item2 = ViewItem::new("Item 2".to_string());
        item1.add_sub_item(item2.clone());

        assert_eq!(vec!(item2), item1.sub_items)
    }

    #[test]
    fn view_item_equal_if_cloned() {
        let item1 = ViewItem::new("Item 1".to_string());
        let item2 = item1.clone();

        assert_eq!(item1, item2)
    }

    #[test]
    fn view_item_equal_if_id_equal() {
        let id = String::from("::ITEM_2_ID::");
        let text = "Item 1".to_string();
        let item1 = ViewItem::new(text.clone());
        let item2 = ViewItem::with_id(text.clone(), id.clone());
        let item3 = ViewItem::with_id("Other Text".to_string(), id.clone());
        assert_ne!(item1, item2);
        assert_eq!(item2, item3);
    }
    
    #[test]
    fn view_item_finds_self_by_id() {
        let item = ViewItem::new("::ANY OLD ITEM::".to_string());
        let result = item.find_by_id(&item.id);
        
        assert_eq!(item, *result.unwrap())
    }
}