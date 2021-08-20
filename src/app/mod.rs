use spair::prelude::*;

mod footer;
mod header;

pub struct App {
    pub comp: spair::Comp<Self>,
    pub route: crate::routes::Route,
    pub user: Option<types::UserInfo>,
    pub page: Page,
}

pub enum Page {
    Home(spair::ChildComp<crate::home::HomePage>),
    Register(spair::ChildComp<crate::register::Register>),
}

impl Page {
    pub fn new(route: &crate::routes::Route, comp: &spair::Comp<crate::app::App>) -> Self {
        use crate::routes::Route;
        match route {
            Route::Register => Self::Register(spair::ChildComp::init(comp, ())),
            _ => Self::Home(spair::ChildComp::init(comp, ())),
        }
    }
}

impl App {
    fn new(comp: spair::Comp<Self>) -> Self {
        let route = crate::routes::Route::Home;
        let page = Page::new(&route, &comp);
        Self {
            comp,
            route,
            user: None,
            page,
        }
    }

    pub fn set_route(&mut self, route: crate::routes::Route) -> spair::ShouldRender {
        if self.route == route {
            return spair::ShouldRender::No;
        }
        self.route = route;
        self.page = Page::new(&self.route, &self.comp);
        spair::ShouldRender::Yes
    }

    pub fn set_user(&mut self, user: types::UserInfo) {
        self.user = Some(user);
        self.set_route(crate::routes::Route::Home);
    }
}

impl spair::Component for App {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(header::Header)
            .render(&self.route.url())
            .div(|d| match &self.page {
                Page::Home(child) => d.component(child),
                Page::Register(child) => d.component(child),
            })
            .render(footer::Footer);
    }
}

impl spair::Application for App {
    fn init(comp: &spair::Comp<Self>) -> Self {
        Self::new(comp.clone())
    }

    fn init_router(comp: &spair::Comp<Self>) -> Option<crate::routes::Router> {
        Some(crate::routes::Router {
            app: comp.clone(),
            //home: None,
        })
    }
}
