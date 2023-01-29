mod renders;

pub struct ArticleList {
    filter: ArticleFilter,
    article_list: Option<realworld_shared::types::ArticleListInfo>,
    current_page: u32,
    error: Option<realworld_shared::error::Error>,
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
            current_page: 0,
            error: None,
        }
    }

    pub fn set_filter(&mut self, filter: ArticleFilter) -> spair::Command<Self> {
        self.filter = filter;
        self.current_page = 0;
        self.request_article_list()
    }

    fn request_article_list(&mut self) -> spair::Command<Self> {
        let filter = self.filter.clone();
        let current_page = self.current_page;
        spair::Future::new(async move {
            use realworld_shared::services::articles::*;
            match filter {
                ArticleFilter::Global => all(current_page).await,
                ArticleFilter::Feed => feed().await,
                ArticleFilter::Tag(tag) => by_tag(tag, current_page).await,
                ArticleFilter::Author(author) => by_author(author, current_page).await,
                ArticleFilter::FavoritedByUser(author) => favorited_by(author, current_page).await,
            }
        })
        .with_fn(|state: &mut Self, list| match list {
            Ok(list) => state.article_list = Some(list),
            Err(e) => state.error = Some(e),
        })
    }

    fn set_current_page(&mut self, current_page: u32) -> spair::Command<Self> {
        log::info!("current page {}", current_page);
        self.current_page = current_page;
        self.request_article_list()
    }

    fn toggle_favorite(
        &mut self,
        current_favorited_value: bool,
        slug: &str,
    ) -> spair::Command<Self> {
        let slug = slug.to_string();
        spair::Future::new(async move {
            use realworld_shared::services::articles::*;
            match current_favorited_value {
                false => favorite(slug).await,
                true => unfavorite(slug).await,
            }
        })
        .with_fn(|state: &mut Self, a| match a {
            Ok(a) => state.update_article(a),
            Err(e) => state.error = Some(e),
        })
    }

    fn update_article(&mut self, article: realworld_shared::types::ArticleInfoWrapper) {
        if let Some(a) = self.article_list
            .as_mut()
            .and_then(|list| {
                list.articles
                    .iter_mut()
                    .find(|a| a.slug == article.article.slug)
            }) {
            *a = article.article;
        }
    }
}
