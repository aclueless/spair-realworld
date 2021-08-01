pub struct HomePage {
    pub feed: Feed,
    pub page_number: u32,
    pub article_list: types::ArticleListInfo,
    pub tag_list: types::TagListInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feed {
    Global,
    Your,
    Tag(String),
}

impl Feed {
    pub fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    pub fn is_your(&self) -> bool {
        matches!(self, Self::Your)
    }

    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }
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

    pub fn set_feed(&mut self, feed: Feed) {
        self.feed = feed;
    }

    pub fn toggle_favorite(&mut self, slug: &types::Slug) {
        todo!()
    }

    pub fn set_selected_tag(&mut self, tag: &str) {
        self.feed = Feed::Tag(tag.to_string());
        self.page_number = 0;
    }
}
