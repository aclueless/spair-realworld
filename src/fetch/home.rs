use spair::prelude::*;

impl crate::pages::HomePage {
    pub fn request_data_for_home(&self) -> spair::Checklist<Self> {
        let mut cl = Self::default_checklist();
        cl.set_skip_render();
        cl.add_command(self.request_feeds());
        cl
    }

    fn request_feeds(&self) -> spair::Command<Self> {
        let url = super::UrlBuilder::new().article().page(self.page_number).into_inner();
        spair::Request::get(&url)
            .text_mode()
            .response()
            .json(Self::feeds_arrived, Self::fetch_error)
    }

    fn fetch_error(&mut self, error: spair::FetchError) {
        //
    }

    fn feeds_arrived(&mut self, al: types::ArticleListInfo) {
        self.article_list = al;
    }
}
