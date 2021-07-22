pub struct HomePage {
    pub article_list: types::ArticleListInfo,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            article_list: types::ArticleListInfo {
                articles: Vec::new(),
                articles_count: 0,
            }
        }
    }

    pub fn your_feed(&mut self) {}

    pub fn global_feed(&mut self) {}

    pub fn tag_feed(&mut self) {}

    pub fn is_your_feed(&self) -> bool {
        false
    }

    pub fn is_global_feed(&self) -> bool {
        false
    }

    pub fn is_tag_feed(&self) -> bool {
        false
    }
}
