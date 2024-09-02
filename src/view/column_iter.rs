use crate::model::ViewItem;
use crate::view::Column;

pub struct ColumnIter<'a> {
    collection: Vec<&'a ViewItem>,
    index: usize,
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = &'a ViewItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.len() {
            let item = &self.collection[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> Column<'a> {
    pub fn iter(&self) -> ColumnIter {
        ColumnIter {
            collection: self.top_item.flatten(),
            index: 0,
        }
    }
}
