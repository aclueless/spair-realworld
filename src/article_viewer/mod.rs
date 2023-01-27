use spair::prelude::*;

mod renders;

pub struct ArticleViewer {
    logged_in_user: Option<realworld_shared::types::UserInfo>,
    slug: String,
    article: Option<realworld_shared::types::ArticleInfo>,
    comments: Option<Vec<realworld_shared::types::CommentInfo>>,
    new_comment: String,
    error: Option<realworld_shared::error::Error>,
}

pub enum ArticleToView {
    Slug(String),
    Article(realworld_shared::types::ArticleInfo),
}

pub struct Props {
    pub logged_in_user: Option<realworld_shared::types::UserInfo>,
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
        let slug = self.slug.clone();
        spair::Future::new(async move { realworld_shared::services::articles::get(slug).await })
            .with_fn(|state: &mut Self, a| -> spair::OptionCommand<Self> {
                match a {
                    Ok(a) => {
                        let slug = a.article.slug.clone();
                        state.set_article(a);
                        state.get_comments(slug).into()
                    }
                    Err(e) => {
                        state.error = Some(e);
                        None.into()
                    }
                }
            })
            .into()
    }

    fn get_comments(&self, slug: String) -> spair::Command<Self> {
        spair::Future::new(
            async move { realworld_shared::services::comments::for_article(slug).await },
        )
        .with_fn(|state: &mut Self, comments| match comments {
            Ok(comments) => state.comments = Some(comments.comments),
            Err(e) => state.error = Some(e),
        })
    }

    fn set_article(&mut self, article: realworld_shared::types::ArticleInfoWrapper) {
        self.article = Some(article.article);
    }

    fn toggle_follow(&self) -> spair::OptionCommand<Self> {
        let Some((following, username)) = self.article.as_ref().map(|a| (a.author.following, a.author.username.clone())) else {
            return None.into();
        };
        spair::Future::new(async move {
            use realworld_shared::services::profiles::*;
            match following {
                false => follow(username).await,
                true => unfollow(username).await,
            }
        })
        .with_fn(|state: &mut Self, p| match p {
            Ok(p) => state.update_article_author_profile(p.profile),
            Err(e) => state.error = Some(e),
        })
        .into()
    }

    fn update_article_author_profile(
        &mut self,
        new_article_author_profile: realworld_shared::types::ProfileInfo,
    ) {
        self.article
            .as_mut()
            .map(|a| a.author = new_article_author_profile);
    }

    fn delete_article(&self) -> spair::Command<Self> {
        let slug = self.slug.clone();
        spair::Future::new(async move { realworld_shared::services::articles::del(slug).await })
            .with_fn(|state: &mut Self, d| match d {
                Ok(d) => state.delete_article_completed(d),
                Err(e) => state.error = Some(e),
            })
    }

    fn delete_article_completed(&mut self, _: realworld_shared::types::DeleteWrapper) {
        crate::routes::Route::Home.execute_routing();
    }

    fn toggle_favorite(&self) -> spair::OptionCommand<Self> {
        let Some(favorited) = self.article.as_ref().map(|a| a.favorited) else {
            return None.into();
        };
        let slug = self.slug.clone();
        spair::Future::new(async move {
            use realworld_shared::services::articles::*;
            match favorited {
                true => unfavorite(slug).await,
                false => favorite(slug).await,
            }
        })
        .with_fn(|state: &mut Self, a| match a {
            Ok(a) => state.set_article(a),
            Err(e) => state.error = Some(e),
        })
        .into()
    }

    fn set_new_comment(&mut self, new_comment: String) {
        self.new_comment = new_comment;
    }

    fn post_comment(&mut self) -> spair::OptionCommand<Self> {
        if self.article.is_none() {
            return None.into();
        }
        let slug = self.slug.clone();
        let mut new_comment = String::new();
        std::mem::swap(&mut self.new_comment, &mut new_comment);
        spair::Future::new(async move {
            realworld_shared::services::comments::create(
                slug,
                realworld_shared::types::CommentCreateInfoWrapper {
                    comment: realworld_shared::types::CommentCreateInfo { body: new_comment },
                },
            )
            .await
        })
        .with_fn(|state: &mut Self, c| match c {
            Ok(c) => state.add_comment(c),
            Err(e) => state.error = Some(e),
        })
        .into()
    }

    fn add_comment(&mut self, comment: realworld_shared::types::CommentInfoWrapper) {
        self.comments
            .as_mut()
            .map(|comments| comments.insert(0, comment.comment));
    }

    fn delete_comment(&mut self, comment_id: u32) -> spair::OptionCommand<Self> {
        if self.article.is_none() {
            return None.into();
        }
        let slug = self.slug.clone();
        spair::Future::new(async move {
            realworld_shared::services::comments::delete(slug, comment_id).await
        })
        .with_fn(move |state: &mut Self, d| match d {
            Ok(_) => state.remove_comment(comment_id),
            Err(e) => state.error = Some(e),
        })
        .into()
    }

    fn remove_comment(&mut self, comment_id: u32) {
        self.comments
            .as_mut()
            .map(|comments| comments.retain(|c| c.id != comment_id));
    }
}
