use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct ArticleList<P> {
    phantom: std::marker::PhantomData<P>,
    filter: ArticleFilter,
    article_list: Option<types::ArticleListInfo>,
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

impl<C: spair::Component> ArticleList<C> {
    fn new(filter: ArticleFilter) -> Self {
        Self {
            phantom: std::marker::PhantomData as std::marker::PhantomData<C>,
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
        let url = crate::urls::UrlBuilder::new();
        let url = match &self.filter {
            ArticleFilter::Global => url.articles().page(self.page_number).done(),
            ArticleFilter::Feed => url.articles().feed_in_page(self.page_number),
            ArticleFilter::Tag(tag) => url.articles().page(self.page_number).tag(tag),
            ArticleFilter::Author(author_name) => {
                url.articles().page(self.page_number).author(author_name)
            }
            ArticleFilter::FavoritedByUser(username) => {
                url.articles().page(self.page_number).favorited_by(username)
            }
        };
        spair::http::Request::get(&url)
            .set_token()
            .text_mode()
            .response()
            .json(
                |state, article_list| state.article_list = Some(article_list),
                Self::responsed_error,
            )
    }

    fn toggle_favorite(
        &mut self,
        current_favorited_value: bool,
        slug: &types::Slug,
    ) -> spair::Command<Self> {
        let url = crate::urls::UrlBuilder::new()
            .articles()
            .slug(slug)
            .favorite();
        match current_favorited_value {
            true => spair::http::Request::delete(&url),
            false => spair::http::Request::post(&url),
        }
        .set_token()
        .text_mode()
        .response()
        .json(Self::update_article, Self::responsed_error)
    }

    fn update_article(&mut self, article: types::ArticleInfoWrapper) {
        self.article_list
            .as_mut()
            .and_then(|list| {
                list.articles
                    .iter_mut()
                    .find(|a| a.slug == article.article.slug)
            })
            .map(|a| *a = article.article);
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(error.into());
    }
}
