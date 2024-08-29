

#[derive(Debug, Clone)]
pub struct ViewItem {
    pub name: String,
}

impl ViewItem {
    pub fn new(name: String) -> ViewItem {
        ViewItem {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_item_has_name() {
        let name = "::ITEM_NAME::";
        let item = ViewItem::new(name.to_string());
        assert_eq!(item.name, name);
    }
}