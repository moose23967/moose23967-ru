use crate::application_factory::ApplicationFactory;
use may_minihttp::HttpServiceFactory;

pub mod application;
pub mod application_factory;

fn main() {
    ApplicationFactory
        .start("0.0.0.0:80")
        .unwrap()
        .join()
        .unwrap();
}
