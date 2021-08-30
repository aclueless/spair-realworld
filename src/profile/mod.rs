use spair::prelude::*;
mod renders;

pub struct Profile {
    username: String,
    profile: Option<types::ProfileInfo>,
    tab: ProfileTab,
    page_number: u32,
    error: Option<crate::error::Error>,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    Articles,
    FavoritedArticles,
}

impl Profile {
    fn new(username: String) -> Self {
        Self {
            username,
            profile: None,
            tab: ProfileTab::Articles,
            page_number: 0,
            error: None,
        }
    }

    pub fn set_username_and_favorited(&mut self, (username, favorited): (String, bool)) {
        //
    }
}
