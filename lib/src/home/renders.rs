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
            .rfn(render_banner)
            .rfn(render_feeds);
    }
}

impl spair::AsChildComp for super::HomePage {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = Option<types::UserInfo>;
    fn init(comp: &spair::Comp<Self>, logged_in_user: Self::Properties) -> Self {
        Self::new(comp.clone(), logged_in_user)
    }
}

fn render_banner(nodes: spair::Nodes<super::HomePage>) {
    let state = nodes.state();
    nodes.match_if(|mi| match state.logged_in_user.as_ref() {
        None => spair::set_arm!(mi)
            .static_nodes()
            .div(|d| {
                d.class("banner").div(|d| {
                    d.class("container")
                        .h1(|h| h.class("logo-font").static_text("conduit").done())
                        .p(|p| p.static_text("A place to share your knowledge.").done());
                });
            })
            .done(),
        Some(_) => spair::set_arm!(mi).done(),
    });
}

fn render_feeds(nodes: spair::Nodes<super::HomePage>) {
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
                            .rfn(render_feed_tabs)
                            .component_ref(state.article_list_comp.component_ref());
                    })
                    .rfn(render_popular_tags);
            });
    });
}

fn render_feed_tabs(nodes: spair::Nodes<super::HomePage>) {
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
                        .rfn(|nodes| {
                            render_feed_tab(
                                "Your Feed",
                                state.filter == crate::article_list::ArticleFilter::Feed,
                                comp.handler_mut(|state| {
                                    state.set_filter(crate::article_list::ArticleFilter::Feed)
                                }),
                                nodes,
                            )
                        })
                        .done(),
                })
                .rfn(|nodes| {
                    render_feed_tab(
                        "Global Feed",
                        state.filter == crate::article_list::ArticleFilter::Global,
                        comp.handler_mut(|state| {
                            state.set_filter(crate::article_list::ArticleFilter::Global)
                        }),
                        nodes,
                    )
                })
                .match_if(|mi| match &state.filter {
                    crate::article_list::ArticleFilter::Tag(tag) => {
                        let tag = tag.to_string();
                        spair::set_arm!(mi).rfn(|nodes| {
                            render_feed_tab(
                                &format!("#{}", tag),
                                true,
                                comp.handler_mut(move |state| {
                                    state.set_selected_tag(&tag);
                                }),
                                nodes,
                            )
                        });
                    }
                    _ => spair::set_arm!(mi).done(),
                });
        });
    });
}

fn render_feed_tab<F: spair::Click>(
    title: &str,
    active: bool,
    handler: F,
    nodes: spair::Nodes<super::HomePage>,
) {
    nodes.li(|li| {
        li.static_attributes().class("nav-item").a(|a| {
            a.class_if(active, "active")
                //.href_str("")
                .static_attributes()
                .on_click(handler)
                .class("nav-link")
                .static_text(title)
                .done()
        });
    });
}

fn render_popular_tags(nodes: spair::Nodes<super::HomePage>) {
    let state = nodes.state();
    let comp = nodes.comp();
    nodes.div(|d| {
        d.static_attributes().class("col-md-3")
        .div(|d| {
            d.static_attributes().class("sidebar")
                .static_nodes()
                .p(|p| p.update_text("Popular Tags").done())
                .update_nodes()
                .match_if(|mi| match state.tag_list.as_ref() {
                    None => spair::set_arm!(mi).static_text("Loading tags...").done(),
                    Some(tag_list) => spair::set_arm!(mi)
                        .div(|d| {
                            d.static_attributes().class("tag-list")
                                .list_clone(
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
                                        .update_text(tag);
                                    }
                                );
                        }).done(),
                });
        });
    });
}
