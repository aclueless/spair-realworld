use spair::prelude::*;

mod footer;
mod header;

impl spair::Component for super::App {
    type Routes = crate::routes::Route;

    fn init(comp: &spair::Comp<Self>) {
        if realworld_shared::services::get_token().is_some() {
            comp.callback_once_mut(super::App::get_logged_in_user_info)
                .queue();
        }
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .rupdate(header::Header)
            .rupdate(&self.route.url())
            .div(|d| match &self.page {
                super::Page::Home(child) => d.component(child),
                super::Page::Register(child) => d.component(child),
                super::Page::Login(child) => d.component(child),
                super::Page::Editor(child) => d.component(child),
                super::Page::Viewer(child) => d.component(child),
                super::Page::Profile(child) => d.component(child),
                super::Page::Settings(child) => d.component(child),
            })
            .rupdate(footer::Footer);
    }
}

impl spair::Application for super::App {
    fn init(comp: &spair::Comp<Self>) -> Self {
        Self::new(comp.clone())
    }

    fn init_router(comp: &spair::Comp<Self>) -> Option<crate::routes::Router> {
        Some(crate::routes::Router {
            app_comp: comp.clone(),
            profile_comp: None,
        })
    }
}
