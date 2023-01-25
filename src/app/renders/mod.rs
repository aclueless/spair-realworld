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
            .component_ref2(match &self.page {
                super::Page::Home(child) => child.component_ref(),
                super::Page::Register(child) => child.component_ref(),
                super::Page::Login(child) => child.component_ref(),
                super::Page::Editor(child) => child.component_ref(),
                super::Page::Viewer(child) => child.component_ref(),
                super::Page::Profile(child) => child.component_ref(),
                super::Page::Settings(child) => child.component_ref(),
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
