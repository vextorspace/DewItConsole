use crate::model::Model;

pub trait ModelUpdater {
    fn get_model<'a>(&'a self) -> Model;
}