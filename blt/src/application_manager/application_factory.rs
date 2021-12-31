use crate::application_manager::GattApplication;

pub trait ApplicationFactory {
    fn create(&self) -> GattApplication;
}
