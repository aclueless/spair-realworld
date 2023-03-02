use spair::prelude::*;

impl spair::Component for super::Profile {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback_once_mut(Self::request_profile_info).queue();
    }

    fn register_routing_callback(
        router: &mut <Self::Routes as spair::Routes>::Router,
        comp: &spair::Comp<Self>,
    ) {
        router.profile_comp = Some(comp.clone());
    }

    fn remove_routing_callback(router: &mut <Self::Routes as spair::Routes>::Router) {
        router.profile_comp = None;
    }

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("profile-page")
                .match_if(|mi| match self.profile.as_ref() {
                    None => spair::set_arm!(mi).done(),
                    Some(profile) => spair::set_arm!(mi)
                        .rfn(|nodes| render_profile_view(profile, nodes))
                        .done(),
                })
                .div(|d| {
                    d.class("container").div(|d| {
                        d.class("row").div(|d| {
                            d.class("col-xs-12")
                                .class("col-md-10")
                                .class("offset-md-1")
                                .rfn(|nodes| render_tab_list(&self.profile_username, nodes))
                                .component_ref(self.article_list_comp.component_ref());
                        });
                    });
                });
        });
    }
}

impl spair::AsChildComp for super::Profile {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = super::Props;
    fn init(comp: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(comp.clone(), props)
    }
}

fn render_profile_view(profile: &types::ProfileInfo, nodes: spair::Nodes<super::Profile>) {
    nodes.div(|d| {
        d.class("user-info").div(|d| {
            d.class("container").div(|d| {
                d.class("row").div(|d| {
                    d.class("col-xs-12")
                        .class("col-md-10")
                        .class("offset-md-1")
                        .rfn(|nodes| render_profile(profile, nodes));
                });
            });
        });
    });
}

fn render_profile(profile: &types::ProfileInfo, nodes: spair::Nodes<super::Profile>) {
    let state = nodes.state();
    let comp = nodes.comp();
    nodes
        .img(|i| {
            i.class("user-image").src(&profile.image);
        })
        .h4(|h| h.update_text(&profile.username).done())
        .match_if(|mi| match profile.bio.as_ref() {
            None => spair::set_arm!(mi).done(),
            Some(bio) => spair::set_arm!(mi).p(|p| p.update_text(bio).done()).done(),
        })
        .match_if(|mi| match state.is_logged_in_username(&profile.username) {
            None => spair::set_arm!(mi).done(),
            Some(true) => spair::set_arm!(mi)
                .a(|a| {
                    a.class("btn")
                        .class("btn-sm")
                        .class("btn-outline-secondary")
                        .class("action-btn")
                        .href(&crate::routes::Route::Settings)
                        .static_text("Edit Profile Settings");
                })
                .done(),
            Some(false) => spair::set_arm!(mi)
                .button(|b| {
                    b.class("btn")
                        .class("btn-sm")
                        .class_or(profile.following, "btn-secondary", "btn-outline-secondary")
                        .class("action-btn")
                        .on_click(comp.handler(super::Profile::toggle_follow))
                        .i(|i| i.class("ion-plus-round").done())
                        .update_text(if profile.following {
                            "Unfollow"
                        } else {
                            " Follow "
                        })
                        .update_text(&profile.username);
                })
                .done(),
        });
}

fn render_tab_list(name: &str, nodes: spair::Nodes<super::Profile>) {
    let state = nodes.state();
    nodes.div(|d| {
        d.class("articles-toggle").ul(|u| {
            u.class("nav")
                .class("nav-pills")
                .class("outline-active")
                .rfn(|nodes| {
                    render_tab(
                        "Articles",
                        !state.favorited,
                        crate::routes::Route::Profile(name.to_string()),
                        nodes,
                    )
                })
                .rfn(|nodes| {
                    render_tab(
                        "Favorite Articles",
                        state.favorited,
                        crate::routes::Route::ProfileFavorites(name.to_string()),
                        nodes,
                    )
                });
        });
    });
}

fn render_tab(
    title: &'static str,
    active: bool,
    route: crate::routes::Route,
    nodes: spair::Nodes<super::Profile>,
) {
    nodes.li(|i| {
        i.class("nav-item").a(|a| {
            a.class("nav-link")
                .class_if(active, "active")
                .href(&route)
                .static_text(title);
        });
    });
}
