use spair::prelude::*;

use realworld_shared::types::*;

mod renders;

pub struct ArticleList {
    filter: ArticleFilter,
    article_list: Option<ArticleListInfo>,
    page_number: u32,
    error: Option<crate::error::Error>,
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
    fn new(filter: ArticleFilter) -> Self {
        Self {
            filter,
            article_list: None,
            page_number: 0,
            error: None,
        }
    }

    pub fn set_filter(&mut self, filter: ArticleFilter) {
        self.filter = filter;
        self.page_number = 0;
    }

    fn request_article_list(&mut self) -> spair::Command<Self> {
        let filter = self.filter.clone();
        spair::Future::new(async move {
            use realworld_shared::services::articles::*;
            match filter {
                ArticleFilter::Global => all(self.page_number).await,
                ArticleFilter::Feed => feed().await,
                ArticleFilter::Tag(tag) => by_tag(tag, self.page_number).await,
                ArticleFilter::Author(author) => by_author(author, self.page_number).await,
                ArticleFilter::FavoritedByUser(author) => {
                    favorited_by(author, self.page_number).await
                }
            }
        })
        .with_fn(|state: &mut Self, list| match list {
            Ok(list) => state.article_list = Some(list),
            Err(e) => state.error = Some(e.to_string()),
        })
    }

    fn set_page_number(&mut self, page_number: u32) -> spair::Command<Self> {
        self.page_number = page_number;
        self.request_article_list()
    }

    fn toggle_favorite(
        &mut self,
        current_favorited_value: bool,
        slug: &str,
    ) -> spair::Command<Self> {
        spair::Future::new(async move {
            use realworld_shared::services::articles::*;
            match current_favorited_value {
                false => favorite(slug).await,
                true => unfavorite(slug).await,
            }
        })
        .with_fn(|state: &mut Self, a| match a {
            Ok(a) => self.update_article(a),
            Err(e) => self.error = Some(e.to_string()),
        })
    }

    fn update_article(&mut self, article: ArticleInfoWrapper) {
        self.article_list
            .as_mut()
            .and_then(|list| {
                list.articles
                    .iter_mut()
                    .find(|a| a.slug == article.article.slug)
            })
            .map(|a| *a = article.article);
    }
}
