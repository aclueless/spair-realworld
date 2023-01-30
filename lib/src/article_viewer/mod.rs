use spair::prelude::*;

mod renders;

pub struct ArticleViewer {
    comp: spair::Comp<Self>,
    logged_in_user: Option<types::UserInfo>,
    slug: String,
    article: Option<types::ArticleInfo>,
    comments: Option<Vec<types::CommentInfo>>,
    new_comment: String,
    error: Option<services::error::Error>,
}

pub enum ArticleToView {
    Slug(String),
    Article(types::ArticleInfo),
}

pub struct Props {
    pub logged_in_user: Option<types::UserInfo>,
    pub article: ArticleToView,
}

impl ArticleViewer {
    fn new(comp: spair::Comp<Self>, props: Props) -> Self {
        let (slug, article) = match props.article {
            ArticleToView::Slug(slug) => (slug, None),
            ArticleToView::Article(article) => (article.slug.clone(), Some(article)),
        };
        Self {
            comp,
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

    fn get_article(&self) {
        if self.article.is_some() {
            return;
        }
        let slug = self.slug.clone();
        let cb = self.comp.callback_arg_mut(
            |state: &mut Self,
             a: Result<
                types::ArticleInfoWrapper,
                services::error::Error,
            >| match a {
                Ok(a) => {
                    let slug = a.article.slug.clone();
                    state.set_article(a);
                    state.get_comments(slug);
                }
                Err(e) => state.error = Some(e),
            },
        );
        services::articles::get(slug).spawn_local_with(cb);
    }

    fn get_comments(&self, slug: String) {
        let cb = self.comp.callback_arg_mut(
            |state: &mut Self,
             comments: Result<
                types::CommentListInfo,
                services::error::Error,
            >| match comments {
                Ok(comments) => state.comments = Some(comments.comments),
                Err(e) => state.error = Some(e),
            },
        );
        services::comments::for_article(slug).spawn_local_with(cb);
    }

    fn set_article(&mut self, article: types::ArticleInfoWrapper) {
        self.article = Some(article.article);
    }

    fn toggle_follow(&self) {
        let Some((following, username)) = self.article.as_ref().map(|a| (a.author.following, a.author.username.clone())) else {
            return;
        };
        let cb = self.comp.callback_arg_mut(|state: &mut Self, p| match p {
            Ok(p) => state.update_article_author_profile(p),
            Err(e) => state.error = Some(e),
        });
        async move {
            use services::profiles::*;
            match following {
                false => follow(username).await,
                true => unfollow(username).await,
            }
        }
        .spawn_local_with(cb);
    }

    fn update_article_author_profile(
        &mut self,
        new_article_author_profile: types::ProfileInfoWrapper,
    ) {
        self.article
            .as_mut()
            .map(|a| a.author = new_article_author_profile.profile);
    }

    fn delete_article(&self) {
        let slug = self.slug.clone();
        let cb = self.comp.callback_arg_mut(|state: &mut Self, d| match d {
            Ok(d) => state.delete_article_completed(d),
            Err(e) => state.error = Some(e),
        });
        services::articles::del(slug).spawn_local_with(cb);
    }

    fn delete_article_completed(&mut self, _: types::DeleteWrapper) {
        crate::routes::Route::Home.execute_routing();
    }

    fn toggle_favorite(&self) {
        let Some(favorited) = self.article.as_ref().map(|a| a.favorited) else {
            return;
        };
        let slug = self.slug.clone();
        let cb = self.comp.callback_arg_mut(|state: &mut Self, a| match a {
            Ok(a) => state.set_article(a),
            Err(e) => state.error = Some(e),
        });
        async move {
            use services::articles::*;
            match favorited {
                true => unfavorite(slug).await,
                false => favorite(slug).await,
            }
        }
        .spawn_local_with(cb);
    }

    fn set_new_comment(&mut self, new_comment: String) {
        self.new_comment = new_comment;
    }

    fn post_comment(&mut self) {
        if self.article.is_none() {
            return;
        }
        let slug = self.slug.clone();
        let mut new_comment = String::new();
        std::mem::swap(&mut self.new_comment, &mut new_comment);
        let cb = self.comp.callback_arg_mut(|state: &mut Self, c| match c {
            Ok(c) => state.add_comment(c),
            Err(e) => state.error = Some(e),
        });
        services::comments::create(
            slug,
            types::CommentCreateInfoWrapper {
                comment: types::CommentCreateInfo { body: new_comment },
            },
        )
        .spawn_local_with(cb);
    }

    fn add_comment(&mut self, comment: types::CommentInfoWrapper) {
        self.comments
            .as_mut()
            .map(|comments| comments.insert(0, comment.comment));
    }

    fn delete_comment(&mut self, comment_id: u32) {
        if self.article.is_none() {
            return;
        }
        let slug = self.slug.clone();
        let cb = self
            .comp
            .callback_arg_mut(move |state: &mut Self, d| match d {
                Ok(_) => state.remove_comment(comment_id),
                Err(e) => state.error = Some(e),
            });
        services::comments::delete(slug, comment_id).spawn_local_with(cb);
    }

    fn remove_comment(&mut self, comment_id: u32) {
        self.comments
            .as_mut()
            .map(|comments| comments.retain(|c| c.id != comment_id));
    }
}
