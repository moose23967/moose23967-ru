use crate::application::Application;
use may_minihttp::HttpServiceFactory;

pub struct ApplicationFactory;

impl HttpServiceFactory for ApplicationFactory {
    type Service = Application;

    fn new_service(&self, _: usize) -> Self::Service {
        Application
    }
}
