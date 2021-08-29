use spair::prelude::*;

mod renders;

pub struct ArticleViewer {
    //app_comp: spair::Comp<crate::app::App>,
    user: Option<types::UserInfo>,
    slug: types::Slug,
    article: Option<types::ArticleInfo>,
    comments: Option<Vec<types::CommentInfo>>,
    new_comment: String,
    error: Option<crate::error::Error>,
}

pub enum ArticleToView {
    Slug(types::Slug),
    Article(types::ArticleInfo),
}

impl ArticleViewer {
    fn new(props: (Option<types::UserInfo>, ArticleToView)) -> Self {
        let (slug, article) = match props.1 {
            ArticleToView::Slug(slug) => (slug, None),
            ArticleToView::Article(article) => (article.slug.clone(), Some(article)),
        };
        Self {
            user: props.0,
            slug,
            article,
            comments: None,
            new_comment: String::new(),
            error: None,
        }
    }

    fn is_logged_in_username(&self, username: &str) -> Option<bool> {
        self.user.as_ref().map(|u| u.username.as_str() == username)
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(error.into());
    }

    fn toggle_follow(&self) -> spair::OptionCommand<Self> {
        self
            .article
            .as_ref()
            .map(|a| {
                match a.author.following {
                    false => {
                        let url = crate::urls::UrlBuilder::new().profile(&a.author.username).follow();
                        spair::http::Request::post(&url)
                    }
                    true => {
                        let url = crate::urls::UrlBuilder::new().profile(&a.author.username).unfollow();
                        spair::http::Request::delete(&url)

                    }
                }
                .text_mode()
                .response()
                .json(Self::update_article_author_profile, Self::responsed_error)
            }).into()
    }

    fn update_article_author_profile(&mut self, new_article_author_profile: types::ProfileInfo) {
        self
            .article
            .as_mut()
            .map(|a| a.author = new_article_author_profile);
    }
}
