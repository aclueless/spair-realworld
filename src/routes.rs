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

impl spair::Routes<crate::app::App> for Route {
    fn url(&self) -> String {
        match self {
            Self::Home(crate::pages::Feed::Global) => "/#/global-feed".to_string(),
            Self::Home(crate::pages::Feed::Your(_)) => "/#/your-feed".to_string(),
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

    fn routing(_location: spair::web_sys::Location, _comp: &spair::Comp<crate::app::App>) {
        //
    }
}
