use spair::prelude::*;

mod renders;

pub struct Profile {
    comp: spair::Comp<Self>,
    logged_in_user: Option<types::UserInfo>,
    profile_username: String,
    profile: Option<types::ProfileInfo>,
    favorited: bool,
    article_list_comp: spair::ChildComp<crate::article_list::ArticleList>,
    error: Option<services::error::Error>,
}

pub struct Props {
    pub logged_in_user: Option<types::UserInfo>,
    pub profile_username: String,
}

impl Profile {
    fn new(comp: spair::Comp<Self>, props: Props) -> Self {
        let filter = crate::article_list::ArticleFilter::Author(props.profile_username.clone());
        let article_list_comp = spair::ChildComp::with_props(filter);
        Self {
            comp,
            logged_in_user: props.logged_in_user,
            profile_username: props.profile_username,
            profile: None,
            favorited: false,
            article_list_comp,
            error: None,
        }
    }

    fn is_logged_in_username(&self, username: &str) -> Option<bool> {
        self.logged_in_user
            .as_ref()
            .map(|u| u.username.as_str() == username)
    }

    pub fn set_username_and_favorited(
        &mut self,
        (username, favorited): (String, bool),
    ) -> spair::ShouldRender {
        let new_user = username != self.profile_username;
        let new_tab = favorited != self.favorited;

        self.profile_username = username;
        self.favorited = favorited;

        if new_user || new_tab {
            let filter = match favorited {
                false => crate::article_list::ArticleFilter::Author(self.profile_username.clone()),
                true => crate::article_list::ArticleFilter::FavoritedByUser(
                    self.profile_username.clone(),
                ),
            };
            self.article_list_comp
                .comp()
                .callback_arg_mut(crate::article_list::ArticleList::set_filter)
                .queue(filter);
        }

        self.request_profile_info();
        spair::ShouldRender::No
    }

    fn request_profile_info(&mut self) {
        let profile_username = self.profile_username.clone();
        let cb = self.comp.callback_arg_mut(|state: &mut Self, p| match p {
            Ok(p) => state.set_profile(p),
            Err(e) => state.error = Some(e),
        });
        services::profiles::get(profile_username).spawn_local_with(cb);
    }

    fn set_profile(&mut self, p: types::ProfileInfoWrapper) {
        self.profile = Some(p.profile);
    }

    fn toggle_follow(&self) {
        let Some(p) = self.profile.as_ref() else {
            return;
        };
        let following = p.following;
        let username = p.username.clone();
        let cb = self.comp.callback_arg_mut(|state: &mut Self, p| match p {
            Ok(p) => state.set_profile(p),
            Err(e) => state.error = Some(e),
        });
        async move {
            match following {
                true => services::profiles::unfollow(username).await,
                false => services::profiles::follow(username).await,
            }
        }
        .spawn_local_with(cb);
    }
}
