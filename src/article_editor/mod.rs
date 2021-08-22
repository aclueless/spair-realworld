mod renders;

pub struct Editor {
    app_comp: spair::Comp<crate::app::App>,
    slug: Option<types::Slug>,
    article: types::ArticleCreateUpdateInfo,
    tag_string: String,
}

impl Editor {
    fn new(app_comp: spair::Comp<crate::app::App>, slug: Option<types::Slug>) -> Self {
        Self {
            app_comp,
            slug,
            article: Default::default(),
            tag_string: String::new(),
        }
    }
}
