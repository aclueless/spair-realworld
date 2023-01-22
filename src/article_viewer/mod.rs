use spair::prelude::*;

use realworld_shared::types::*;

mod renders;

pub struct ArticleViewer {
    logged_in_user: Option<UserInfo>,
    slug: String,
    article: Option<ArticleInfo>,
    comments: Option<Vec<CommentInfo>>,
    new_comment: String,
    error: Option<crate::error::Error>,
}

pub enum ArticleToView {
    Slug(String),
    Article(ArticleInfo),
}

pub struct Props {
    pub logged_in_user: Option<UserInfo>,
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

    fn get_article(&self) -> spair::OptionCommand<Self> {
        if self.article.is_some() {
            return None.into();
        }
        spair::Future::new(
            async move { realworld_shared::services::articles::get(&self.slug).await },
        )
        .with_fn(|state: &mut Self, a| match a {
            Ok(a) => state.set_article(a),
            Err(e) => state.error = Some(e.to_string()),
        })
    }

    fn set_article(&mut self, article: ArticleInfoWrapper) {
        self.article = Some(article.article);
    }

    fn toggle_follow(&self) -> spair::OptionCommand<Self> {
        self.article
            .as_ref()
            .map(|a| {
                spair::Future::new(async move {
                    use realworld_shared::services::profiles::*;
                    match a {
                        false => follow(&a.author.username).await,
                        true => unfollow(&a.author.username).await,
                    }
                })
            })
            .into()
    }

    fn update_article_author_profile(&mut self, new_article_author_profile: ProfileInfo) {
        self.article
            .as_mut()
            .map(|a| a.author = new_article_author_profile);
    }

    fn delete_article(&self) -> spair::Command<Self> {
        spair::Future::new(
            async move { realworld_shared::services::articles::del(&self.slug).await },
        )
        .with_fn(|state, d| match d {
            Ok(d) => state.delete_article_completed(d),
            Err(e) => state.error = Some(e.to_string()),
        })
    }

    fn delete_article_completed(&mut self, _: DeleteWrapper) {
        crate::routes::Route::Home.execute_routing();
    }

    fn toggle_favorite(&self) -> spair::OptionCommand<Self> {
        self.article
            .as_ref()
            .map(|a| {
                spair::Future::new(async move {
                    use realworld_shared::services::articles::*;
                    match a.favorited {
                        true => unfavorite(&self.slug),
                        false => favorite(&self.slug),
                    }
                })
                .with_fn(|state, a| match a {
                    Ok(a) => state.set_article(a),
                    Err(e) => state.error = Some(e.to_string()),
                })
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
        spair::Future::new(async move {
            realworld_shared::services::comments::create(
                &self.slug,
                CommentCreateInfoWrapper {
                    comment: CommentCreateInfo {
                        body: self.new_comment.clone(),
                    },
                },
            )
            .await
        })
        .with_fn(|state, c| match c {
            Ok(c) => state.add_comment(c),
            Err(e) => state.error = Some(e.to_string()),
        })
        .into()
    }

    fn add_comment(&mut self, comment: CommentInfoWrapper) {
        self.comments
            .as_mut()
            .map(|comments| comments.insert(0, comment.comment));
    }

    fn delete_comment(&mut self, comment_id: u32) -> spair::OptionCommand<Self> {
        if self.article.is_none() {
            return None.into();
        }
        spair::Future::new(async move {
            realworld_shared::services::comments::delete(&self.slug, comment_id).await
        })
        .with_fn(|state, d| match d {
            Ok(_) => state.remove_comment(comment_id),
            Err(e) => state.error = Some(e.to_string()),
        })
    }

    fn remove_comment(&mut self, comment_id: u32) {
        self.comments
            .as_mut()
            .map(|comments| comments.retain(|c| c.id != comment_id));
    }
}
