mod renders;

pub struct ArticleViewer {
    //app_comp: spair::Comp<crate::app::App>,
    user: Option<types::UserInfo>,
    slug: types::Slug,
    article: Option<types::ArticleInfo>,
    comments: Option<Vec::Comment>,
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
            error: None,
        }
    }

    fn user_own_article(&self, article_author_username: &str) -> Option<bool> {
        self.user.as_ref().map(|u| u.username.as_ref() == article_author_username)
    }
}
