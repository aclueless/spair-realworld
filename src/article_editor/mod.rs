use spair::prelude::*;
use crate::SetAuthorizationToken;

mod renders;

pub struct ArticleEditor {
    app_comp: spair::Comp<crate::app::App>,
    slug: Option<types::Slug>,
    article: types::ArticleCreateUpdateInfo,
    //tag_string: String,
    error: Option<crate::error::Error>,
}

impl ArticleEditor {
    fn new(app_comp: spair::Comp<crate::app::App>, slug: Option<types::Slug>) -> Self {
        Self {
            app_comp,
            slug,
            article: Default::default(),
            //tag_string: String::new(),
            error: None,
        }
    }

    fn get_article(&mut self) ->  spair::OptionCommand<Self> {
        self
            .slug
            .as_ref()
            .map(|slug| crate::urls::UrlBuilder::new().articles().slug(slug).done())
            .map(|url| {
                spair::http::Request::get(&url)
                    .set_token()
                    .text_mode()
                    .response()
                    .json(Self::set_article_for_editting, Self::responsed_error)
            }).into()
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        self.error = Some(error.into());
    }

    fn set_article_for_editting(&mut self, article_info: types::ArticleInfoWrapper) {
        self.article = types::ArticleCreateUpdateInfo {
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
        let data = types::ArticleCreateUpdateInfoWrapper {
            article: self.article.clone(),
        };
        let url = crate::urls::UrlBuilder::new().articles();
        let builder = match self.slug.as_ref() {
            Some(slug) => {
                let url = url.slug(slug).done();
                spair::http::Request::put(&url)
            }
            None => {
                let url = url.create_article();
                spair::http::Request::post(&url)
            }
        };
        builder
            .set_token()
            .text_mode()
            .body()
            .json(&data)
            .response()
            .json(Self::responsed_article, Self::responsed_error)
    }

    fn responsed_article(&mut self, article_info: types::ArticleInfoWrapper) {
        spair::update_component(self.app_comp.callback_once_mut(move |state| state.view_article(article_info.article)));
    }
}
