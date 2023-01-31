use spair::prelude::*;

mod renders;

pub struct ArticleList {
    comp: spair::Comp<Self>,
    filter: ArticleFilter,
    article_list: Option<types::ArticleListInfo>,
    current_page: u32,
    error: Option<services::error::Error>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArticleFilter {
    Global,
    Feed,
    Tag(String),
    Author(String),
    FavoritedByUser(String),
}

impl ArticleList {
    fn new(comp: spair::Comp<Self>, filter: ArticleFilter) -> Self {
        Self {
            comp,
            filter,
            article_list: None,
            current_page: 0,
            error: None,
        }
    }

    pub fn set_filter(&mut self, filter: ArticleFilter) {
        self.filter = filter;
        self.current_page = 0;
        self.request_article_list()
    }

    fn request_article_list(&mut self) {
        let cb = self
            .comp
            .callback_arg_mut(|state: &mut Self, list| match list {
                Ok(list) => state.article_list = Some(list),
                Err(e) => state.error = Some(e),
            });
        match &self.filter {
            ArticleFilter::Global => services::articles::all(self.current_page),
            ArticleFilter::Feed => services::articles::feed(),
            ArticleFilter::Tag(tag) => services::articles::by_tag(tag, self.current_page),
            ArticleFilter::Author(author) => {
                services::articles::by_author(author, self.current_page)
            }
            ArticleFilter::FavoritedByUser(author) => {
                services::articles::favorited_by(author, self.current_page)
            }
        }
        .send()
        .spawn_local_with(cb);
    }

    fn set_current_page(&mut self, current_page: u32) {
        log::info!("current page {}", current_page);
        self.current_page = current_page;
        self.request_article_list()
    }

    fn toggle_favorite(&mut self, current_favorited_value: bool, slug: &str) {
        let cb = self.comp.callback_arg_mut(|state: &mut Self, a| match a {
            Ok(a) => state.update_article(a),
            Err(e) => state.error = Some(e),
        });
        match current_favorited_value {
            false => services::articles::favorite(slug),
            true => services::articles::unfavorite(slug),
        }
        .send()
        .spawn_local_with(cb);
    }

    fn update_article(&mut self, article: types::ArticleInfoWrapper) {
        if let Some(a) = self.article_list.as_mut().and_then(|list| {
            list.articles
                .iter_mut()
                .find(|a| a.slug == article.article.slug)
        }) {
            *a = article.article;
        }
    }
}
