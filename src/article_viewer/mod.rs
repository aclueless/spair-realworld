use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct ArticleViewer {
    logged_in_user: Option<types::UserInfo>,
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

pub struct Props {
    pub logged_in_user: Option<types::UserInfo>,
    pub article: ArticleToView,
}

impl ArticleViewer {
    fn new(props: Props) -> Self {
        let (slug, article) = match props.article {
            ArticleToView::Slug(slug) => (slug, None),
            ArticleToView::Article(article) => (article.slug.clone(), Some(article)),
        };
        Self {
            logged_in_user: props.logged_in_user,
            slug,
            article,
            comments: None,
            new_comment: String::new(),
            error: None,
        }
    }

    fn is_logged_in_username(&self, username: &str) -> Option<bool> {
        self.logged_in_user
            .as_ref()
            .map(|u| u.username.as_str() == username)
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(error.into());
    }

    fn current_article_url(&self) -> crate::urls::TheSpecifiedArticle {
        crate::urls::UrlBuilder::new().articles().slug(&self.slug)
    }

    fn get_article(&self) -> spair::OptionCommand<Self> {
        if self.article.is_some() {
            return None.into();
        }
        let url = self.current_article_url().get();
        spair::http::Request::get(&url)
            .text_mode()
            .response()
            .json(Self::set_article, Self::responsed_error)
            .into()
    }

    fn set_article(&mut self, article: types::ArticleInfoWrapper) {
        self.article = Some(article.article);
    }

    fn toggle_follow(&self) -> spair::OptionCommand<Self> {
        self.article
            .as_ref()
            .map(|a| {
                let url = crate::urls::UrlBuilder::new()
                    .profile(&a.author.username)
                    .follow();
                match a.author.following {
                    false => spair::http::Request::post(&url),
                    true => spair::http::Request::delete(&url),
                }
                .set_token()
                .text_mode()
                .response()
                .json(Self::update_article_author_profile, Self::responsed_error)
            })
            .into()
    }

    fn update_article_author_profile(&mut self, new_article_author_profile: types::ProfileInfo) {
        self.article
            .as_mut()
            .map(|a| a.author = new_article_author_profile);
    }

    fn delete_article(&self) -> spair::Command<Self> {
        let url = self.current_article_url().delete();
        spair::http::Request::delete(&url)
            .set_token()
            .text_mode()
            .response()
            .json(Self::delete_article_completed, Self::responsed_error)
    }

    fn delete_article_completed(&mut self, _: types::DeleteWrapper) {
        crate::routes::Route::Home.execute_routing();
    }

    fn toggle_favorite(&self) -> spair::OptionCommand<Self> {
        let url = self.current_article_url().favorite();
        self.article
            .as_ref()
            .map(|a| {
                match a.favorited {
                    false => spair::http::Request::post(&url),
                    true => spair::http::Request::delete(&url),
                }
                .set_token()
                .text_mode()
                .response()
                .json(Self::set_article, Self::responsed_error)
            })
            .into()
    }

    fn set_new_comment(&mut self, new_comment: String) {
        self.new_comment = new_comment;
    }

    fn post_comment(&self) -> spair::OptionCommand<Self> {
        if self.article.is_none() {
            return None.into();
        }
        let url = self.current_article_url().comment();
        spair::http::Request::post(&url)
            .set_token()
            .text_mode()
            .body()
            .json(&types::CommentCreateInfoWrapper {
                comment: types::CommentCreateInfo {
                    body: self.new_comment.clone(),
                },
            })
            .response()
            .json(Self::add_comment, Self::responsed_error)
            .into()
    }

    fn add_comment(&mut self, comment: types::CommentInfoWrapper) {
        self.comments
            .as_mut()
            .map(|comments| comments.insert(0, comment.comment));
    }

    fn delete_comment(&mut self, comment_id: u32) -> spair::OptionCommand<Self> {
        if self.article.is_none() {
            return None.into();
        }
        let url = self.current_article_url().delete_comment(comment_id);
        spair::http::Request::delete(&url)
            .set_token()
            .text_mode()
            .response()
            .json(
                move |state, _: types::DeleteWrapper| state.remove_comment(comment_id),
                Self::responsed_error,
            )
            .into()
    }

    fn remove_comment(&mut self, comment_id: u32) {
        self.comments
            .as_mut()
            .map(|comments| comments.retain(|c| c.id != comment_id));
    }
}
