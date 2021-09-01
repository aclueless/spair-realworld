mod fetch;
mod renders;

pub struct HomePage {
    pub feed: Feed,
    pub page_number: u32,
    pub article_list: Option<types::ArticleListInfo>,
    pub tag_list: Option<types::TagListInfo>,
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
            article_list: None,
            tag_list: Some(types::TagListInfo {
                tags: vec!["TagToTest".to_string()],
            }),
        }
    }

    pub fn set_feed(&mut self, feed: Feed) {
        self.feed = feed;
    }

    pub fn toggle_favorite(&mut self, slug: &types::Slug) -> spair::Command<Self> {
        todo!()
    }

    pub fn set_selected_tag(&mut self, tag: &str) {
        log::info!("{}", tag);
        self.feed = Feed::Tag(tag.to_string());
        self.page_number = 0;
    }
}
