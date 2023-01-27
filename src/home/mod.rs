use spair::prelude::*;

mod renders;

pub struct HomePage {
    logged_in_user: Option<realworld_shared::types::UserInfo>,
    filter: crate::article_list::ArticleFilter,
    article_list_comp: spair::ChildComp<crate::article_list::ArticleList>,
    tag_list: Option<realworld_shared::types::TagListInfo>,
    error: Option<realworld_shared::error::Error>,
}

impl HomePage {
    pub fn new(logged_in_user: Option<realworld_shared::types::UserInfo>) -> Self {
        let filter = crate::article_list::ArticleFilter::Global;
        Self {
            logged_in_user,
            filter: filter.clone(),
            article_list_comp: spair::ChildComp::with_props(filter),
            tag_list: Some(realworld_shared::types::TagListInfo {
                tags: vec!["TagToTest".to_string()],
            }),
            error: None,
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

    pub fn request_data_for_home_page(&self) -> spair::Checklist<Self> {
        let mut cl = Self::default_checklist();
        cl.set_skip_render();
        //cl.add_command(self.request_feeds());
        cl.add_command(self.request_tags());
        cl
    }

    fn request_tags(&self) -> spair::Command<Self> {
        spair::Future::new(async move { realworld_shared::services::tags::get_all().await })
            .with_fn(|state: &mut Self, tag_list| match tag_list {
                Ok(tag_list) => state.tag_list = Some(tag_list),
                Err(e) => state.error = Some(e),
            })
    }
}
