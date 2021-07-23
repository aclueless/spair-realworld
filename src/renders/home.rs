use spair::prelude::*;

impl spair::Component for crate::pages::HomePage {
    type Routes = ();
    fn initialize(comp: &spair::Comp<Self>) {
        spair::update_component(comp.callback(Self::request_data_for_home_page));
    }
    fn render(&self, element: spair::Element<Self>) {
        element.class("home-page").render(Banner).render(Feeds);
    }
}

impl spair::WithParentComp for crate::pages::HomePage {
    type Parent = crate::app::App;
    type Properties = ();
    fn init(
        _parent: &spair::Comp<Self::Parent>,
        _comp: spair::Comp<Self>,
        _props: Self::Properties,
    ) -> Self {
        Self::new()
    }
}

struct Banner;
impl spair::Render<crate::pages::HomePage> for Banner {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes.static_nodes().div(|d| {
            d.class("banner").div(|d| {
                d.class("container")
                    .h1(|h| h.class("logo-font").r#static("conduit").done())
                    .p(|p| p.r#static("A place to share your knowledge.").done());
            });
        });
    }
}

struct Feeds;
impl spair::Render<crate::pages::HomePage> for Feeds {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.static_attributes().class("container").class("page").div(|d| {
                d.static_attributes().class("row")
                    .div(|d| {
                        d.static_attributes().class("col-md-9")
                            .render(FeedTabs)
                            .list(
                                state.article_list.articles.iter(),
                                spair::ListElementCreation::Clone,
                            )
                            .render(Pagenation);
                    })
                    .render(PopularTags);
            });
        });
    }
}

struct FeedTabs;
impl spair::Render<crate::pages::HomePage> for FeedTabs {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.div(|d| {
            d.static_attributes().class("feed-toggle").ul(|u| {
                u.static_attributes().class("nav")
                    .class("nav-pills")
                    .class("outline-active")
                    .render(FeedTab {
                        title: "Your Feed",
                        active: state.feed.is_your(),
                        handler: comp.handler_mut(crate::pages::HomePage::your_feed),
                    })
                    .render(FeedTab {
                        title: "Global Feed",
                        active: state.feed.is_global(),
                        handler: comp.handler_mut(crate::pages::HomePage::global_feed),
                    })
                    .render(FeedTab {
                        title: "Tag Feed",
                        active: state.feed.is_tag(),
                        handler: comp.handler_mut(crate::pages::HomePage::tag_feed),
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
impl<'a, F: spair::Click> spair::Render<crate::pages::HomePage> for FeedTab<'a, F> {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes.li(|i| {
            i.static_attributes().class("nav-item").a(|a| {
                a.class_if("active", self.active)
                    .href_str("")
                    .on_click(self.handler)
                    .static_attributes()
                    .class("nav-link")
                    .r#static(self.title)
                    .done()
            });
        });
    }
}

impl spair::ListItemRender<crate::pages::HomePage> for &types::ArticleInfo {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<crate::pages::HomePage>) {
        element
            .static_attributes().class("article-preview")
            .div(|d| {
                d.static_attributes().class("article-meta")
                    .a(|a| {
                        // Hack on the routes, must be fixed after a redesign of spair's Router
                        a.href_str(
                            &crate::routes::Route::Profile(self.author.username.clone()).url(),
                        )
                        .img(|i| i.src(&self.author.image).done());
                    })
                    .div(|d| {
                        d.static_attributes().class("info")
                            .a(|a| {
                                a.href_str("").static_attributes().class("author").render(&self.author.username);
                            })
                            .span(|s| {
                                s.static_attributes().class("date").render(&self.created_at.to_string());
                            });
                    })
                    .button(|b| {
                        b.static_attributes()
                            .class("btn")
                            .class("btn-outline-primary")
                            .class("btn-sm")
                            .class("pull-xs-right")
                            .i(|i| i.static_attributes().class("ion-heart").done())
                            .r#static(" ")
                            .render(self.favorites_count);
                    });
            })
            .a(|a| {
                a.static_attributes().class("preview-link")
                    .h1(|h| h.render(&self.title).done())
                    .p(|p| p.render(&self.description).done())
                    .static_nodes()
                    .span(|s| s.r#static("Read more...").done());
            });
    }
}

struct Pagenation;
impl spair::Render<crate::pages::HomePage> for Pagenation {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes.render("Pagenation");
    }
}

struct PopularTags;
impl spair::Render<crate::pages::HomePage> for PopularTags {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.static_attributes().class("col-md-3")
            .div(|d| {
                d.static_attributes().class("sidebar")
                    .static_nodes()
                    .p(|p| p.render("Popular Tags").done())
                    .nodes()
                    .div(|d| {
                        d.static_attributes().class("tag-list")
                            .list_with_render(
                                state.tag_list.tags.iter(),
                                spair::ListElementCreation::Clone,
                                "a",
                                |tag, a| {
                                    a
                                    .href_str("")
                                    .static_attributes()
                                    .class("tag-pill")
                                    .class("tag-default")
                                    .render(tag);
                                }
                            );
                    });
            });
        });
    }
}

