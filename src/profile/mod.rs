use spair::prelude::*;

mod renders;

pub struct Profile {
    logged_in_user: Option<realworld_shared::types::UserInfo>,
    profile_username: String,
    profile: Option<realworld_shared::types::ProfileInfo>,
    favorited: bool,
    article_list_comp: spair::ChildComp<crate::article_list::ArticleList>,
    error: Option<realworld_shared::error::Error>,
}

pub struct Props {
    pub logged_in_user: Option<realworld_shared::types::UserInfo>,
    pub profile_username: String,
}

impl Profile {
    fn new(props: Props) -> Self {
        let filter = crate::article_list::ArticleFilter::Author(props.profile_username.clone());
        let article_list_comp = spair::ChildComp::with_props(filter);
        Self {
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
    ) -> spair::Checklist<Self> {
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

        let mut cl = Self::default_checklist();
        cl.set_skip_render();
        cl.add_command(self.request_profile_info());
        cl
    }

    fn request_profile_info(&mut self) -> spair::Command<Self> {
        let profile_username = self.profile_username.clone();
        spair::Future::new(async move {
            realworld_shared::services::profiles::get(profile_username).await
        })
        .with_fn(|state: &mut Self, p| match p {
            Ok(p) => state.set_profile(p),
            Err(e) => state.error = Some(e),
        })
    }

    fn set_profile(&mut self, p: realworld_shared::types::ProfileInfoWrapper) {
        self.profile = Some(p.profile);
    }

    fn toggle_follow(&self) -> spair::OptionCommand<Self> {
        let Some(p) = self.profile.as_ref() else {
            return None.into();
        };
        let following = p.following;
        let username = p.username.clone();
        spair::Future::new(async move {
            match following {
                true => realworld_shared::services::profiles::unfollow(username).await,
                false => realworld_shared::services::profiles::follow(username).await,
            }
        })
        .with_fn(|state: &mut Self, p| match p {
            Ok(p) => state.set_profile(p),
            Err(e) => state.error = Some(e),
        })
        .into()
    }
}
