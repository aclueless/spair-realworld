use spair::prelude::*;

impl spair::Component for super::HomePage {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback(Self::request_data_for_home_page).queue();
    }

    fn register_routing_callback(router: &mut crate::routes::Router, comp: &spair::Comp<Self>) {
        //log::debug!("register_routing_callback for home page");
        //router.home = Some(comp.clone());
    }

    fn remove_routing_callback(router: &mut crate::routes::Router) {
        //log::debug!("remove_routing_callback  for home page");
        //router.home = None;
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .class("home-page")
            .rupdate(Banner)
            .rupdate(&format!("{:?}", self.filter))
            .rupdate(Feeds);
    }
}

impl spair::AsChildComp for super::HomePage {
    type Properties = ();
    fn init(_comp: &spair::Comp<Self>, _props: Self::Properties) -> Self {
        Self::new()
    }
}

struct Banner;
impl spair::Render<super::HomePage> for Banner {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        nodes.static_nodes().div(|d| {
            d.class("banner").div(|d| {
                d.class("container")
                    .h1(|h| h.class("logo-font").rstatic("conduit").done())
                    .p(|p| p.rstatic("A place to share your knowledge.").done());
            });
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
                                .div(|d| d.component(&state.article_list_comp));
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
            d.static_attributes().class("feed-toggle").ul(|u| {
                u.static_attributes()
                    .class("nav")
                    .class("nav-pills")
                    .class("outline-active")
                    .rupdate(FeedTab {
                        title: "Your Feed",
                        active: state.filter == crate::article_list::ArticleFilter::Feed,
                        handler: comp.handler_mut(|state| {
                            state.set_filter(crate::article_list::ArticleFilter::Feed)
                        }),
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
        nodes.li(|i| {
            i.static_attributes().class("nav-item").a(|a| {
                a.class_if(self.active, "active")
                    //.href_str("")
                    .on_click(self.handler)
                    .static_attributes()
                    .class("nav-link")
                    .rstatic(self.title)
                    .done()
            });
        });
    }
}

struct Pagenation;
impl spair::Render<super::HomePage> for Pagenation {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        nodes.rupdate("Pagenation");
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
                    .nodes()
                    .match_if(|mi| match state.tag_list.as_ref() {
                        None => spair::set_arm!(mi).rstatic("Loading tags...").done(),
                        Some(tag_list) => spair::set_arm!(mi)
                            .div(|d| {
                                d.static_attributes().class("tag-list")
                                    .list_with_render(
                                        tag_list.tags.iter(),
                                        spair::ListElementCreation::Clone,
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
