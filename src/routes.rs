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

impl spair::Routes<crate::app::App> for Route {
    fn url(&self) -> String {
        match self {
            Self::Home => "/#/".to_string(),
            Self::Login => "/#/login".to_string(),
            Self::Register => "/#/register".to_string(),
            Self::Settings => "/#/settings".to_string(),
            Self::Editor(None) => "/#/editor".to_string(),
            Self::Editor(Some(slug)) => {
                let mut url = "/#/editor/".to_string();
                url.push_str(slug);
                url
            }
            Self::Article(slug) => {
                let mut url = "/#/article/".to_string();
                url.push_str(slug);
                url
            }
            Self::Profile(user_name) => {
                let mut url = "/#/profile/".to_string();
                url.push_str(user_name);
                url
            }
            Self::ProfileFavorites(user_name) => {
                let mut url = "/#/profile/".to_string();
                url.push_str(user_name);
                url.push_str("/favorites");
                url
            }
        }
    }

    fn routing(_location: spair::web_sys::Location, _comp: &spair::Comp<crate::app::App>) {
        //
    }
}
