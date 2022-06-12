use crate::SetAuthorizationToken;
use spair::prelude::*;

mod renders;

pub struct HomePage {
    filter: crate::article_list::ArticleFilter,
    article_list_comp: spair::ChildComp<crate::article_list::ArticleList>,
    tag_list: Option<types::TagListInfo>,
}

impl HomePage {
    pub fn new(comp: &spair::Comp<Self>) -> Self {
        let filter = crate::article_list::ArticleFilter::Global;
        Self {
            filter: filter.clone(),
            article_list_comp: spair::ChildComp::with_props(filter),
            tag_list: Some(types::TagListInfo {
                tags: vec!["TagToTest".to_string()],
            }),
        }
    }

    pub fn set_filter(&mut self, filter: crate::article_list::ArticleFilter) {
        if self.filter != filter {
            self.filter = filter.clone();
            self.article_list_comp
                .comp()
                .callback_once_mut(move |state| state.set_filter(filter))
                .queue();
        }
    }

    pub fn set_selected_tag(&mut self, tag: &str) {
        self.set_filter(crate::article_list::ArticleFilter::Tag(tag.to_string()));
    }

    fn responsed_error(&mut self, error: spair::ResponsedError<types::ErrorInfo>) {
        //self.error = Some(error.into());
    }

    pub fn request_data_for_home_page(&self) -> spair::Checklist<Self> {
        let mut cl = Self::default_checklist();
        cl.set_skip_render();
        //cl.add_command(self.request_feeds());
        cl.add_command(self.request_tags());
        cl
    }

    fn request_tags(&self) -> spair::Command<Self> {
        let url = crate::urls::UrlBuilder::new().tags();
        spair::http::Request::get(&url).text_mode().response().json(
            |state, tag_list| state.tag_list = Some(tag_list),
            Self::responsed_error,
        )
    }
}
