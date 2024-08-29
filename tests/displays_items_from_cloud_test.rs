
use dew_it_console::model::{Model, ModelUpdater, ViewItem};
use dew_it_console::view::Display;

#[test]
fn test_displays_empty_items_from_cloud_gives_title() {
    let items: Vec<ViewItem> = vec!();
    let model_updater: Box<dyn ModelUpdater> = Box::new(FakeModelUpdater::new(items));
    let model = model_updater.get_model();

    let display_lines = display_and_capture_output(model);

    assert_eq!(1, display_lines.len());
    assert_eq!("Dew It!", display_lines[0]);
}

#[test]
fn test_displays_list_with_pipe_separator() {
    let item1_text = "Item 1";
    let item2_text = "Item 2";
    let item_names = vec!(item1_text, item2_text);

    let model = make_model(item_names);

    let display_lines = display_and_capture_output(model);

    assert_eq!(2, display_lines.len());

    let index = display_lines[1].find("|");
    assert!(index.is_some());
    let first = &display_lines[1][..index.unwrap()];
    let second = &display_lines[1][index.unwrap() + 1..];
    assert_eq!(item1_text, first.trim());
    assert_eq!(item2_text, second.trim());
}

fn make_model(item_names: Vec<&str>) -> Model {
    let items = item_names.iter().map(|name| ViewItem::new(name.to_string())).collect();

    let model_updater: Box<dyn ModelUpdater> = Box::new(FakeModelUpdater::new(items));
    let model = model_updater.get_model();
    model
}

fn display_and_capture_output(model: Model) -> Vec<String> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut display = Display::new(&mut buffer);

    display.initialize(&model).unwrap();

    String::from_utf8_lossy(&buffer)
        .lines()
        .map(|line| line.to_string())
        .collect()
}

struct FakeModelUpdater {
    items: Vec<ViewItem>,
}

impl FakeModelUpdater {
    fn new(items: Vec<ViewItem>) -> FakeModelUpdater {
        FakeModelUpdater {
            items: items,
        }
    }
}

impl ModelUpdater for FakeModelUpdater {
    fn get_model<'a>(&'a self) -> Model {
        Model {
            items: self.items.clone(),
        }
    }
}