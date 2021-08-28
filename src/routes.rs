#[derive(Debug, PartialEq)]
pub enum Route {
    Home,
    Login,
    Register,
    Settings,
    Editor(Option<types::Slug>),
    Article(types::Slug),
    Profile(String),
    ProfileFavorites(String),
}

pub struct Router {
    pub app: spair::Comp<crate::app::App>,
    //    pub home: Option<spair::Comp<crate::pages::HomePage>>,
}

impl spair::Routes for Route {
    type Router = Router;
    fn url(&self) -> String {
        match self {
            Self::Home => "/#/".to_string(),
            Self::Login => "/#/login".to_string(),
            Self::Register => "/#/register".to_string(),
            Self::Settings => "/#/settings".to_string(),
            Self::Editor(None) => "/#/editor".to_string(),
            Self::Editor(Some(slug)) => format!("/#/editor/{}", slug.as_ref()),
            Self::Article(slug) => format!("/#/article/{}", slug.as_ref()),
            Self::Profile(user_name) => format!("/#/profile/{}", user_name),
            Self::ProfileFavorites(user_name) => format!("/#/profile/{}/favorites", user_name),
        }
    }
}

impl spair::Router for Router {
    fn routing(&self, location: spair::web_sys::Location) {
        let route = match location.hash().unwrap_or_else(|_| String::new()).as_str() {
            "" | "#" | "#/" => Route::Home,
            "#/login" => Route::Login,
            "#/register" => Route::Register,
            "#/settings" => Route::Settings,
            "#/editor" => Route::Editor(None),
            hash => {
                if hash.starts_with("#/article/") {
                    Route::Article(types::Slug::from(hash.replace("#/article/", "")))
                } else {
                    Route::Home
                }
            }
        };
        // match &route {
        //     Route::Home(feed) => {
        //         if let Some(home) = self.home.as_ref() {
        //             home.callback_arg_mut(crate::pages::HomePage::set_feed)(feed.clone());
        //             return;
        //         }
        //     }
        //     _ => {}
        // }
        self.app.callback_arg_mut(crate::app::App::set_route)(route);
    }
}
