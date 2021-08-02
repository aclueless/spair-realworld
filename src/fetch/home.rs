use spair::prelude::*;

impl crate::pages::HomePage {
    fn fetch_error(&mut self, error: spair::FetchError) {
        //
    }

    pub fn request_data_for_home_page(&self) -> spair::Checklist<Self> {
        let mut cl = Self::default_checklist();
        cl.set_skip_render();
        cl.add_command(self.request_feeds());
        cl.add_command(self.request_tags());
        cl
    }

    fn request_feeds(&self) -> spair::Command<Self> {
        let url = super::UrlBuilder::new().articles().page(self.page_number).done();
        spair::Request::get(&url)
            .text_mode()
            .response()
            .json(|state, article_list| state.article_list = Some(article_list), Self::fetch_error)
    }

    fn request_tags(&self) -> spair::Command<Self> {
        let url = super::UrlBuilder::new().tags();
        spair::Request::get(&url)
            .text_mode()
            .response()
            .json(|state, tag_list| state.tag_list = Some(tag_list), Self::fetch_error)
    }

    fn favorite(&self) {}
}
