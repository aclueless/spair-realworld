use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct App {
    comp: spair::Comp<Self>,
    route: crate::routes::Route,
    user: Option<types::UserInfo>,
    page: Page,
}

pub enum Page {
    Home(spair::ChildComp<crate::home::HomePage>),
    Register(spair::ChildComp<crate::register::Register>),
    Login(spair::ChildComp<crate::login::Login>),
    Editor(spair::ChildComp<crate::article_editor::Editor>),
}

impl Page {
    pub fn new(route: &crate::routes::Route, comp: &spair::Comp<crate::app::App>) -> Self {
        use crate::routes::Route;
        match route {
            Route::Register => Self::Register(spair::ChildComp::init(comp, ())),
            Route::Login => Self::Login(spair::ChildComp::init(comp, ())),
            Route::Editor(slug) => Self::Editor(spair::ChildComp::init(comp, slug.clone())),
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

    pub fn set_user(&mut self, user: types::UserInfoWrapper) {
        let user = user.user;
        crate::store_token(&user.token);
        self.user = Some(user);
        //self.set_route(crate::routes::Route::Home);
        crate::routes::Route::Home.execute_routing();
    }

    fn get_logged_in_user_info(&mut self) -> spair::Command<Self> {
        let url = crate::urls::UrlBuilder::new().user();
        spair::http::Request::get(&url)
            .set_token()
            .text_mode()
            .response()
            .json(Self::set_user, |_, _: spair::FetchError| {})
    }

    pub fn view_article(&mut self, article_info: types::ArticleInfoWrapper) {
        //
    }
}
