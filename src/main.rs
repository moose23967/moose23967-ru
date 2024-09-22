use crate::application_factory::ApplicationFactory;
use may_minihttp::HttpServiceFactory;

pub mod application;
pub mod application_factory;

fn main() {
    ApplicationFactory
        .start("127.0.0.1:8080")
        .unwrap()
        .join()
        .unwrap();
}
