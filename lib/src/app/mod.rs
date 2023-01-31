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
    Editor(spair::ChildComp<crate::article_editor::ArticleEditor>),
    Viewer(spair::ChildComp<crate::article_viewer::ArticleViewer>),
    Profile(spair::ChildComp<crate::profile::Profile>),
    Settings(spair::ChildComp<crate::settings::Settings>),
}

impl Page {
    pub fn new(
        route: &crate::routes::Route,
        user: Option<&types::UserInfo>,
        comp: &spair::Comp<crate::app::App>,
    ) -> Self {
        use crate::routes::Route;
        match route {
            Route::Register => Self::Register(spair::ChildComp::with_props(
                comp.callback_arg_mut(App::set_user_and_navigate_to_home),
            )),
            Route::Login => Self::Login(spair::ChildComp::with_props(
                comp.callback_arg_mut(App::set_user_and_navigate_to_home),
            )),
            Route::Editor(slug) => {
                Self::Editor(spair::ChildComp::with_props(crate::article_editor::Props {
                    view_article_callback: comp.callback_arg_mut(App::view_article),
                    slug: slug.clone(),
                }))
            }
            Route::Article(slug) => {
                Self::Viewer(spair::ChildComp::with_props(crate::article_viewer::Props {
                    logged_in_user: user.cloned(),
                    article: crate::article_viewer::ArticleToView::Slug(slug.clone()),
                }))
            }
            Route::Profile(username) => {
                Self::Profile(spair::ChildComp::with_props(crate::profile::Props {
                    logged_in_user: user.cloned(),
                    profile_username: username.to_string(),
                }))
            }
            Route::Settings => {
                Self::Settings(spair::ChildComp::with_props(crate::settings::Props {
                    logout_callback: comp.callback_mut(App::logout),
                    set_user_callback: comp.callback_arg_mut(App::set_user_info),
                    user_info: user.cloned(),
                }))
            }
            _ => Self::Home(spair::ChildComp::with_props(user.cloned())),
        }
    }
}

impl App {
    fn new(comp: spair::Comp<Self>) -> Self {
        let route = crate::routes::Route::Home;
        let page = Page::new(&route, None, &comp);
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
        self.page = Page::new(&self.route, self.user.as_ref(), &self.comp);
        spair::ShouldRender::Yes
    }

    pub fn set_user_info(&mut self, user: types::UserInfoWrapper) {
        let user = user.user;
        services::set_token(crate::LOCAL_STORAGE_TOKEN_KEY, Some(user.token.as_str()));
        self.user = Some(user);
    }

    pub fn set_user_and_navigate_to_home(&mut self, user: types::UserInfoWrapper) {
        self.set_user_info(user);
        crate::routes::Route::Home.execute_routing();
    }

    fn get_logged_in_user_info(&mut self) {
        let cb = self.comp.callback_arg_mut(|state: &mut Self, rs| match rs {
            Ok(rs) => state.set_user_info(rs),
            Err(_) => services::set_token(crate::LOCAL_STORAGE_TOKEN_KEY, None),
        });
        services::auth::current().send().spawn_local_with(cb);
    }

    pub fn view_article(&mut self, article_info: types::ArticleInfo) {
        crate::routes::Route::Article(article_info.slug.clone()).update_address_bar();
        self.page = Page::Viewer(spair::ChildComp::with_props(crate::article_viewer::Props {
            logged_in_user: self.user.clone(),
            article: crate::article_viewer::ArticleToView::Article(article_info),
        }));
    }

    pub fn logout(&mut self) {
        self.user = None;
        services::set_token(crate::LOCAL_STORAGE_TOKEN_KEY, None);
        crate::routes::Route::Home.execute_routing();
    }
}
