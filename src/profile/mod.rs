use spair::prelude::*;
mod renders;

pub struct Profile {
    logged_in_user: Option<types::UserInfo>,
    profile_username: String,
    profile: Option<types::ProfileInfo>,
    tab: ProfileTab,
    page_number: u32,
    article_list: Option<types::ArticleListInfo>,
    error: Option<crate::error::Error>,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    Articles,
    FavoritedArticles,
}

impl Profile {
    fn new(logged_in_user: Option<types::UserInfo>, profile_username: String) -> Self {
        Self {
            logged_in_user,
            profile_username,
            profile: None,
            tab: ProfileTab::Articles,
            page_number: 0,
            article_list: None,
            error: None,
        }
    }

    fn is_logged_in_username(&self, username: &str) -> Option<bool> {
        self.logged_in_user.as_ref().map(|u| u.username.as_str() == username)
    }


    pub fn set_username_and_favorited(&mut self, (username, favorited): (String, bool)) {
        //
    }

    fn toggle_follow(&self) -> spair::Command<Self> {
        todo!()
    }

    fn toggle_favorite(&mut self, slug: &types::Slug) -> spair::Command<Self> {
        todo!()
    }

}
