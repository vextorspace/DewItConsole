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