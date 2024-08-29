
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

fn display_and_capture_output(model: Model) -> Vec<String> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut display = Display::new(&mut buffer);
    display.initialize(&model);

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