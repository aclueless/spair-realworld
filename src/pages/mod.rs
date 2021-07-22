mod home;

pub use home::*;

pub enum Page {
    Home(spair::ChildComp<home::HomePage>),
}

impl Page {
    pub fn new(_route: &crate::routes::Route, comp: &spair::Comp<crate::app::App>) -> Self {
        Self::Home(spair::ChildComp::init(comp, ()))
    }
}
