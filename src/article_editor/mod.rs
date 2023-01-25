mod renders;

pub struct ArticleEditor {
    view_article_callback: spair::CallbackArg<realworld_shared::types::ArticleInfo>,
    slug: Option<String>,
    article: realworld_shared::types::ArticleCreateUpdateInfo,
    //tag_string: String,
    error: Option<realworld_shared::error::Error>,
}

pub struct Props {
    pub view_article_callback: spair::CallbackArg<realworld_shared::types::ArticleInfo>,
    pub slug: Option<String>,
}

impl ArticleEditor {
    fn new(props: Props) -> Self {
        Self {
            view_article_callback: props.view_article_callback,
            slug: props.slug,
            article: Default::default(),
            //tag_string: String::new(),
            error: None,
        }
    }

    fn get_article(&mut self) -> spair::OptionCommand<Self> {
        let Some(slug) = self.slug.as_ref() else {
            return None.into();
        };
        let slug = slug.to_string();
        spair::Future::new(async move { realworld_shared::services::articles::get(slug).await })
            .with_fn(|state: &mut Self, a| match a {
                Ok(a) => state.set_article_for_editting(a),
                Err(e) => state.error = Some(e),
            })
            .into()
    }

    fn set_article_for_editting(
        &mut self,
        article_info: realworld_shared::types::ArticleInfoWrapper,
    ) {
        self.article = realworld_shared::types::ArticleCreateUpdateInfo {
            title: article_info.article.title,
            description: article_info.article.description,
            body: article_info.article.body,
            tag_list: Some(article_info.article.tag_list),
        };
        self.error = None;
    }

    fn set_title(&mut self, title: String) {
        self.article.title = title;
    }

    fn set_description(&mut self, description: String) {
        self.article.description = description;
    }

    fn set_body(&mut self, body: String) {
        self.article.body = body;
    }

    fn add_tag(&mut self, tag: String) {
        let tags = self.article.tag_list.get_or_insert_with(Vec::new);
        if tags.contains(&tag) == false {
            tags.push(tag);
        }
    }

    fn remove_tag(&mut self, tag: &str) {
        if let Some(tags) = self.article.tag_list.as_mut() {
            tags.retain(|t| t != tag);
        }
    }

    fn publish_article(&self) -> spair::Command<Self> {
        let data = realworld_shared::types::ArticleCreateUpdateInfoWrapper {
            article: self.article.clone(),
        };
        let slug = self.slug.clone();
        spair::Future::new(async move {
            if let Some(slug) = slug {
                realworld_shared::services::articles::update(slug, data).await
            } else {
                realworld_shared::services::articles::create(data).await
            }
        })
        .with_fn(|state: &mut Self, a| match a {
            Ok(a) => state.responsed_article(a),
            Err(e) => state.error = Some(e),
        })
    }

    fn responsed_article(&mut self, article_info: realworld_shared::types::ArticleInfoWrapper) {
        self.view_article_callback.queue(article_info.article);
    }
}
