pub struct HomePage {
    pub feed: Feed,
    pub page_number: u32,
    pub article_list: types::ArticleListInfo,
    pub tag_list: types::TagListInfo,
}

pub enum Feed {
    Global,
    Your(String),
    Tag(String),
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            feed: Feed::Global,
            page_number: 0,
            article_list: types::ArticleListInfo {
                articles: Vec::new(),
                articles_count: 0,
            },
            tag_list: types::TagListInfo { tags: Vec::new() },
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
