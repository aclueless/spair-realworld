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
        log::debug!("{:?}", location);
        log::debug!("{:?}", location.hash());
        log::debug!("{:?}", location.href());
        let route = match location.hash().unwrap_or_else(|_| String::new()).as_str() {
            "" | "#" | "#/" | "#/global-feed" => Route::Home(crate::pages::Feed::Global),
            "#/your-feed" => Route::Home(crate::pages::Feed::Your),
            "#/login" => Route::Login,
            "#/register" => Route::Register,
            "#/settings" => Route::Settings,
            "#/editor" => Route::Editor(None),
            hash if hash.starts_with("#/#") => {
                log::debug!("{}", hash);
                let tag = hash.split_at("#/#".len()).1.to_string();
                log::debug!("{}", tag);
                Route::Home(crate::pages::Feed::Tag(tag))
            }
            _ => {
                Route::Home(crate::pages::Feed::Global)
            }
        };
        match &route {
            Route::Home(feed) => {
                if let Some(home) = self.home.as_ref() {
                    home.callback_arg_mut(crate::pages::HomePage::set_feed)(feed.clone());
                    return;
                }
            }
            _ => {}
        }
        self.app.callback_arg_mut(crate::app::App::set_route)(route);
    }
}
