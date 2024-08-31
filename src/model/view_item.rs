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
        if self.id == *id_to_find {
            Some(self)
        } else {
            self.find_in_sub_items(id_to_find)
        }
    }

    fn find_in_sub_items(&self, id_to_find: &String) -> Option<&ViewItem> {
        self.sub_items.iter()
            .map(|item| item.find_by_id(id_to_find))
            .find(|item| item.is_some())
            .flatten()
    }

    pub fn find_level(&self, item_to_find: &ViewItem) -> Option<usize> {
        if self == item_to_find {
            Some(0)
        } else if self.sub_items.is_empty() {
            None
        } else {
            match self.sub_items.iter()
                .map(|child| child.find_level(item_to_find))
                .find(|result| result.is_some())
                .flatten() {
                Some(sub_level) => Some(sub_level+1),
                None => None,
            }
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

    #[test]
    fn view_item_finds_second_child_by_id() {
        let mut item1 = ViewItem::new("::ANY OLD ITEM::".to_string());
        let child1 = ViewItem::new("::CHILD 1::".to_string());
        let child2 = ViewItem::new("::CHILD 2::".to_string());
        item1.add_sub_item(child1.clone());
        item1.add_sub_item(child2.clone());

        let result = item1.find_by_id(&child2.id);
        assert_eq!(child2, *result.unwrap())
    }



    #[test]
    fn finds_sub_level_for_self() {
        let item = ViewItem::new("::ANY::".to_string());

        let level = item.find_level(&item);

        assert_eq!(Some(0), level);
    }

    #[test]
    fn finds_sub_level_none_if_not_there() {
        let item = ViewItem::new("::ANY::".to_string());
        let missing_item = ViewItem::new("::OTHER::".to_string());


        let level = item.find_level(&missing_item);

        assert_eq!(None, level);
    }

    #[test]
    fn finds_sub_level_for_sub_item() {
        let mut item = ViewItem::new("::ANY::".to_string());
        let mut child = ViewItem::new("::ANY::".to_string());
        let grandchild = ViewItem::new("::ANY::".to_string());

        child.add_sub_item(grandchild.clone());
        item.add_sub_item(child.clone());

        let mut level = item.find_level(&child);

        assert_eq!(Some(1), level);

        level = item.find_level(&grandchild);

        assert_eq!(Some(2), level);

        level = child.find_level(&grandchild);

        assert_eq!(Some(1), level);
    }
}