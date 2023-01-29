use spair::prelude::*;

impl spair::Component for super::HomePage {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback(Self::request_data_for_home_page).queue();
    }

    fn register_routing_callback(_router: &mut crate::routes::Router, _comp: &spair::Comp<Self>) {
        //log::debug!("register_routing_callback for home page");
        //router.home = Some(comp.clone());
    }

    fn remove_routing_callback(_router: &mut crate::routes::Router) {
        //log::debug!("remove_routing_callback  for home page");
        //router.home = None;
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .class("home-page")
            .rupdate(Banner)
            .rupdate(Feeds);
    }
}

impl spair::AsChildComp for super::HomePage {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = Option<realworld_shared::types::UserInfo>;
    fn init(_comp: &spair::Comp<Self>, logged_in_user: Self::Properties) -> Self {
        Self::new(logged_in_user)
    }
}

struct Banner;
impl spair::Render<super::HomePage> for Banner {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        let state = nodes.state();
        nodes.match_if(|mi| match state.logged_in_user.as_ref() {
            None => spair::set_arm!(mi)
                .static_nodes()
                .div(|d| {
                    d.class("banner").div(|d| {
                        d.class("container")
                            .h1(|h| h.class("logo-font").rstatic("conduit").done())
                            .p(|p| p.rstatic("A place to share your knowledge.").done());
                    });
                })
                .done(),
            Some(_) => spair::set_arm!(mi).done(),
        });
    }
}

struct Feeds;
impl spair::Render<super::HomePage> for Feeds {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.static_attributes()
                .class("container")
                .class("page")
                .div(|d| {
                    d.static_attributes()
                        .class("row")
                        .div(|d| {
                            d.static_attributes()
                                .class("col-md-9")
                                .rupdate(FeedTabs)
                                .component_ref(state.article_list_comp.component_ref());
                        })
                        .rupdate(PopularTags);
                });
        });
    }
}

struct FeedTabs;
impl spair::Render<super::HomePage> for FeedTabs {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.div(|d| {
            d.static_attributes().class("feed-toggle").ul(|ul| {
                ul.static_attributes()
                    .class("nav")
                    .class("nav-pills")
                    .class("outline-active")
                    .match_if(|mi| match state.logged_in_user.as_ref() {
                        None => spair::set_arm!(mi).done(),
                        Some(_) => spair::set_arm!(mi)
                            .rupdate(FeedTab {
                                title: "Your Feed",
                                active: state.filter == crate::article_list::ArticleFilter::Feed,
                                handler: comp.handler_mut(|state| {
                                    state.set_filter(crate::article_list::ArticleFilter::Feed)
                                }),
                            })
                            .done(),
                    })
                    .rupdate(FeedTab {
                        title: "Global Feed",
                        active: state.filter == crate::article_list::ArticleFilter::Global,
                        handler: comp.handler_mut(|state| {
                            state.set_filter(crate::article_list::ArticleFilter::Global)
                        }),
                    })
                    .match_if(|mi| match &state.filter {
                        crate::article_list::ArticleFilter::Tag(tag) => {
                            let tag = tag.to_string();
                            spair::set_arm!(mi).rupdate(FeedTab {
                                title: &format!("#{}", tag),
                                active: true,
                                handler: comp.handler_mut(move |state| {
                                    state.set_selected_tag(&tag);
                                }),
                            });
                        }
                        _ => spair::set_arm!(mi).done(),
                    });
            });
        });
    }
}

struct FeedTab<'a, F> {
    title: &'a str,
    active: bool,
    handler: F,
}
impl<'a, F: spair::Click> spair::Render<super::HomePage> for FeedTab<'a, F> {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        nodes.li(|li| {
            li.static_attributes().class("nav-item").a(|a| {
                a.class_if(self.active, "active")
                    //.href_str("")
                    .static_attributes()
                    .on_click(self.handler)
                    .class("nav-link")
                    .rstatic(self.title)
                    .done()
            });
        });
    }
}

struct PopularTags;
impl spair::Render<super::HomePage> for PopularTags {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.div(|d| {
            d.static_attributes().class("col-md-3")
            .div(|d| {
                d.static_attributes().class("sidebar")
                    .static_nodes()
                    .p(|p| p.rupdate("Popular Tags").done())
                    .update_nodes()
                    .match_if(|mi| match state.tag_list.as_ref() {
                        None => spair::set_arm!(mi).rstatic("Loading tags...").done(),
                        Some(tag_list) => spair::set_arm!(mi)
                            .div(|d| {
                                d.static_attributes().class("tag-list")
                                    .lwr_clone(
                                        tag_list.tags.iter(),
                                        "a",
                                        |tag, a| {
                                            let route = crate::routes::Route::Home;
                                            let cloned_tag = tag.to_string();
                                            a
                                            .href(&route) // This is a hack?
                                            .on_click(comp.handler_mut(move |state| state.set_selected_tag(&cloned_tag)))
                                            .static_attributes()
                                            .class("tag-pill")
                                            .class("tag-default")
                                            .rupdate(tag);
                                        }
                                    );
                            }).done(),
                    })
;
            });
        });
    }
}
