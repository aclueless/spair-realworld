#[derive(Debug, PartialEq)]
pub enum Route {
    Home,
    Login,
    Register,
    Settings,
    Editor(Option<String>),
    Article(String),
    Profile(String),
    ProfileFavorites(String),
}

pub struct Router {
    pub app_comp: spair::Comp<crate::app::App>,
    pub profile_comp: Option<spair::Comp<crate::profile::Profile>>,
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
            Self::Editor(Some(slug)) => format!("/#/editor/{}", &slug),
            Self::Article(slug) => format!("/#/article/{}", &slug),
            Self::Profile(username) => format!("/#/profile/{}", username),
            Self::ProfileFavorites(username) => format!("/#/profile/{}/favorites", username),
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
                    Route::Article(hash.replace("#/article/", ""))
                } else if hash.starts_with("#/editor/") {
                    Route::Editor(Some(hash.replace("#/editor/", "")))
                } else if hash.starts_with("#/profile/") {
                    log::info!("hash: {}", hash);
                    let tail = hash.replace("#/profile/", "");
                    log::info!("tail: {}", tail);
                    if tail.ends_with("/favorites") {
                        let pf = Route::ProfileFavorites(tail.replace("/favorites", ""));
                        log::info!("pf: {:?}", &pf);
                        pf
                    } else if tail == "/" {
                        Route::Home
                    } else {
                        Route::Profile(tail)
                    }
                } else {
                    Route::Home
                }
            }
        };
        if let Some(profile_comp) = self.profile_comp.as_ref() {
            log::info!("On profile page");
            if let Some(uf) = match &route {
                Route::Profile(username) => Some((username.to_string(), false)),
                Route::ProfileFavorites(username) => Some((username.to_string(), true)),
                _ => None,
            } {
                profile_comp
                    .callback_once_arg_mut(crate::profile::Profile::set_username_and_favorited)
                    .queue(uf);
                return;
            }
        }
        self.app_comp
            .callback_once_arg_mut(crate::app::App::set_route)
            .queue(route);
    }
}
