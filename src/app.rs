use spair::prelude::*;

pub struct App {
    pub comp: spair::Comp<Self>,
    pub route: crate::routes::Route,
    pub user: Option<types::UserInfo>,
    pub page: crate::pages::Page,
}

impl App {
    fn new(comp: spair::Comp<Self>) -> Self {
        let route = crate::routes::Route::Home(crate::pages::Feed::Global);
        let page = crate::pages::Page::new(&route, &comp);
        Self {
            comp,
            route,
            user: None,
            page,
        }
    }

    pub fn set_route(&mut self, route: crate::routes::Route) {
        log::debug!("set_route");
        self.route = route;
    }
}

impl spair::Component for App {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(crate::renders::header::Header)
            .render(&self.route.url())
            .div(|d| match &self.page {
                crate::pages::Page::Home(child) => d.component(child),
            })
            .render(crate::renders::footer::Footer);
    }
}

impl spair::Application for App {
    fn init(comp: &spair::Comp<Self>) -> Self {
        Self::new(comp.clone())
    }

    fn init_router(comp: &spair::Comp<Self>) -> Option<crate::routes::Router> {
        Some(crate::routes::Router {
            app: comp.clone(),
            home: None,
        })
    }
}
