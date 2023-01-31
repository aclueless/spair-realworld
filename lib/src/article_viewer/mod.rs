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
        let cb = self.comp.callback_arg_mut(
            |state: &mut Self, a: Result<types::ArticleInfoWrapper, services::error::Error>| match a
            {
                Ok(a) => {
                    state.get_comments(&a.article.slug);
                    state.set_article(a);
                }
                Err(e) => state.error = Some(e),
            },
        );
        services::articles::get(&self.slug)
            .send()
            .spawn_local_with(cb);
    }

    fn get_comments(&self, slug: &str) {
        let cb = self.comp.callback_arg_mut(
            |state: &mut Self, comments: Result<types::CommentListInfo, services::error::Error>| {
                match comments {
                    Ok(comments) => state.comments = Some(comments.comments),
                    Err(e) => state.error = Some(e),
                }
            },
        );
        services::comments::for_article(slug)
            .send()
            .spawn_local_with(cb);
    }

    fn set_article(&mut self, article: types::ArticleInfoWrapper) {
        self.article = Some(article.article);
    }

    fn toggle_follow(&self) {
        let Some(author) = self.article.as_ref().map(|a| &a.author) else {
            return;
        };
        let cb = self.comp.callback_arg_mut(|state: &mut Self, p| match p {
            Ok(p) => state.update_article_author_profile(p),
            Err(e) => state.error = Some(e),
        });
        match author.following {
            false => services::profiles::follow(&author.username),
            true => services::profiles::unfollow(&author.username),
        }
        .send()
        .spawn_local_with(cb);
    }

    fn update_article_author_profile(
        &mut self,
        new_article_author_profile: types::ProfileInfoWrapper,
    ) {
        if let Some(a) = self.article.as_mut() {
            a.author = new_article_author_profile.profile;
        }
    }

    fn delete_article(&self) {
        let cb = self.comp.callback_arg_mut(|state: &mut Self, d| match d {
            Ok(d) => state.delete_article_completed(d),
            Err(e) => state.error = Some(e),
        });
        services::articles::del(&self.slug)
            .send()
            .spawn_local_with(cb);
    }

    fn delete_article_completed(&mut self, _: types::DeleteWrapper) {
        crate::routes::Route::Home.execute_routing();
    }

    fn toggle_favorite(&self) {
        let Some(favorited) = self.article.as_ref().map(|a| a.favorited) else {
            return;
        };
        let cb = self.comp.callback_arg_mut(|state: &mut Self, a| match a {
            Ok(a) => state.set_article(a),
            Err(e) => state.error = Some(e),
        });
        match favorited {
            true => services::articles::unfavorite(&self.slug),
            false => services::articles::favorite(&self.slug),
        }
        .send()
        .spawn_local_with(cb);
    }

    fn set_new_comment(&mut self, new_comment: String) {
        self.new_comment = new_comment;
    }

    fn post_comment(&mut self) {
        if self.article.is_none() {
            return;
        }
        let mut new_comment = String::new();
        std::mem::swap(&mut self.new_comment, &mut new_comment);
        let cb = self.comp.callback_arg_mut(|state: &mut Self, c| match c {
            Ok(c) => state.add_comment(c),
            Err(e) => state.error = Some(e),
        });
        services::comments::create(
            &self.slug,
            &types::CommentCreateInfoWrapper {
                comment: types::CommentCreateInfo { body: new_comment },
            },
        )
        .send()
        .spawn_local_with(cb);
    }

    fn add_comment(&mut self, comment: types::CommentInfoWrapper) {
        if let Some(comments) = self.comments.as_mut() {
            comments.insert(0, comment.comment);
        }
    }

    fn delete_comment(&mut self, comment_id: u32) {
        if self.article.is_none() {
            return;
        }
        let cb = self.comp.callback_arg_mut(
            move |state: &mut Self, d: Result<types::DeleteWrapper, _>| match d {
                Ok(_) => state.remove_comment(comment_id),
                Err(e) => state.error = Some(e),
            },
        );
        services::comments::delete(&self.slug, comment_id)
            .send()
            .spawn_local_with(cb);
    }

    fn remove_comment(&mut self, comment_id: u32) {
        if let Some(comments) = self.comments.as_mut() {
            comments.retain(|c| c.id != comment_id);
        }
    }
}
