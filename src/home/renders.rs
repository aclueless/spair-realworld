use spair::prelude::*;

impl spair::Component for super::HomePage {
    type Routes = crate::routes::Route;
    fn initialize(comp: &spair::Comp<Self>) {
        spair::update_component(comp.callback(Self::request_data_for_home_page));
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
        element.class("home-page").render(Banner).render(&format!("{:?}", self.feed)).render(Feeds);
    }
}

impl spair::WithParentComp for super::HomePage {
    type Parent = crate::app::App;
    type Properties = ();
    fn init(
        _parent: &spair::Comp<Self::Parent>,
        _comp: &spair::Comp<Self>,
        _props: Self::Properties,
    ) -> Self {
        Self::new()
    }
}

struct Banner;
impl spair::Render<super::HomePage> for Banner {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
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
impl spair::Render<super::HomePage> for Feeds {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.static_attributes().class("container").class("page").div(|d| {
                d.static_attributes().class("row")
                    .div(|d| {
                        d.static_attributes().class("col-md-9")
                            .render(FeedTabs)
                            .match_if(|mi| match state.article_list.as_ref() {
                                None => spair::set_arm!(mi).r#static("Loading articles...").done(),
                                Some(article_list) => spair::set_arm!(mi)
                                    .list(
                                        article_list.articles.iter(),
                                        spair::ListElementCreation::Clone,
                                    ).done(),
                            })
                            .render(Pagenation);
                    })
                    .render(PopularTags);
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
                u.static_attributes().class("nav")
                    .class("nav-pills")
                    .class("outline-active")
                    .render(FeedTab {
                        title: "Your Feed",
                        active: state.feed.is_your(),
                        handler: comp.handler_mut(|state| state.set_feed(super::Feed::Your)),
                    })
                    .render(FeedTab {
                        title: "Global Feed",
                        active: state.feed.is_global(),
                        handler: comp.handler_mut(|state| state.set_feed(super::Feed::Global)),
                    })
                    .match_if(|mi| match &state.feed {
                        super::Feed::Tag(tag) => {
                            let tag = tag.to_string();
                            spair::set_arm!(mi).render(FeedTab {
                                title: &format!("#{}", tag),
                                active: state.feed.is_tag(),
                                handler: comp.handler_mut(move |state| state.set_feed(super::Feed::Tag(tag.clone()))),
                            });
                        }
                        _ => spair::set_arm!(mi).done(),
                    })
                    ;
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
                a.class_if("active", self.active)
                    //.href_str("")
                    .on_click(self.handler)
                    .static_attributes()
                    .class("nav-link")
                    .r#static(self.title)
                    .done()
            });
        });
    }
}

impl spair::ListItemRender<super::HomePage> for &types::ArticleInfo {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::HomePage>) {
        let comp = element.comp();
        let profile = crate::routes::Route::Profile(self.author.username.clone());
        let article_slug = self.slug.clone();
        element
            .static_attributes().class("article-preview")
            .div(|d| {
                d.static_attributes().class("article-meta")
                    .a(|a| {
                        a.href(&profile)
                        .img(|i| i.src(&self.author.image).done());
                    })
                    .div(|d| {
                        d.static_attributes().class("info")
                            .a(|a| {
                                a.href(&profile)
                                    .static_attributes()
                                    .class("author")
                                    .render(&self.author.username);
                            })
                            .span(|s| {
                                s.static_attributes().class("date").render(&self.created_at.to_string());
                            });
                    })
                    .button(|b| {
                        b
                            .on_click(comp.handler_mut(move |state| state.toggle_favorite(&article_slug)))
                            .static_attributes()
                            .class("btn")
                            .class_or(self.favorited, "btn-primary", "btn-outline-primary")
                            .class("btn-sm")
                            .class("pull-xs-right")
                            .i(|i| i.static_attributes().class("ion-heart").done())
                            .r#static(" ")
                            .render(self.favorites_count);
                    });
            })
            .a(|a| {
                let route = crate::routes::Route::Article(From::from(self.slug.clone()));
                a
                    .href(&route)
                    .static_attributes().class("preview-link")
                    .h1(|h| h.render(&self.title).done())
                    .p(|p| p.render(&self.description).done())
                    .static_nodes()
                    .span(|s| s.r#static("Read more...").done());
            })
            .ul(|u| {
                u.static_attributes()
                    .class("tag-list")
                    .list_with_render(
                        self.tag_list.iter(),
                        spair::ListElementCreation::Clone,
                        "li",
                        |tag, li| {
                            li.class("tag-default")
                            .class("tag-pill")
                            .class("tag-outlinepill")
                            .render(tag);
                        }
                    );
            })
            ;
    }
}

struct Pagenation;
impl spair::Render<super::HomePage> for Pagenation {
    fn render(self, nodes: spair::Nodes<super::HomePage>) {
        nodes.render("Pagenation");
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
                    .p(|p| p.render("Popular Tags").done())
                    .nodes()
                    .match_if(|mi| match state.tag_list.as_ref() {
                        None => spair::set_arm!(mi).r#static("Loading tags...").done(),
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
                                            .render(tag);
                                        }
                                    );
                            }).done(),
                    })
;
            });
        });
    }
}
