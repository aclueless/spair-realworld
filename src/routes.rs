#[derive(Debug, PartialEq)]
pub enum Route {
    Home(crate::pages::Feed),
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
    pub home: Option<spair::Comp<crate::pages::HomePage>>,
}

impl spair::Routes for Route {
    type Router = Router;
    fn url(&self) -> String {
        match self {
            Self::Home(crate::pages::Feed::Global) => "/#/global-feed".to_string(),
            Self::Home(crate::pages::Feed::Your) => "/#/your-feed".to_string(),
            Self::Home(crate::pages::Feed::Tag(tag)) => format!("/#/#{}", tag),
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
            "" | "#" | "#/" | "#/global-feed" => Route::Home(crate::pages::Feed::Global),
            "#/your-feed" => Route::Home(crate::pages::Feed::Your),
            "#/login" => Route::Login,
            "#/register" => Route::Register,
            "#/settings" => Route::Settings,
            "#/editor" => Route::Editor(None),
            hash if hash.starts_with("#/#") => Route::Home(crate::pages::Feed::Tag(hash[3..].to_string())),
            hash => {
                log::debug!("{}", hash);
                Route::Home(crate::pages::Feed::Global)
            }
        };
        log::debug!("{}", spair::Routes::url(&route));
        match &route {
            Route::Home(feed) => {
                if let Some(home) = self.home.as_ref() {
                    log::debug!("home page feed");
                    home.callback_arg_mut(crate::pages::HomePage::set_feed)(feed.clone());
                    return;
                }
            }
            _ => {}
        }
        log::debug!("app route");
        self.app.callback_arg_mut(crate::app::App::set_route)(route);
    }
}
