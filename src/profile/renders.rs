use spair::prelude::*;

impl spair::Component for super::Profile {
    type Routes = crate::routes::Route;
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
        element
            .match_if(|mi| match self.profile.as_ref() {
                None => spair::set_arm!(mi).done(),
                Some(profile) => spair::set_arm!(mi).render(profile).done(),
            });
    }
}

impl spair::WithParentComp for super::Profile {
    type Parent = crate::app::App;
    type Properties = (Option<types::UserInfo>, String);
    fn init(
        _: &spair::Comp<Self::Parent>,
        _: &spair::Comp<Self>,
        (logged_in_user, username): Self::Properties,
    ) -> Self {
        Self::new(logged_in_user, username)
    }
}

impl spair::Render<super::Profile> for &types::ProfileInfo {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.class("profile-page")
                .div(|d| {
                    d.class("user-info")
                        .div(|d| {
                            d.class("container")
                                .div(|d| {
                                    d.class("row")
                                        .div(|d| {
                                            d.class("col-xs-12")
                                            .class("col-md-10")
                                            .class("offset-md-1")
                                            .render(ProfileView(self));
                                        });
                                });
                        });
                }).div(|d| {
                    d.class("container")
                        .div(|d| {
                            d.class("row")
                                .div(|d| {
                                    d.class("col-xs-12")
                                        .class("col-md-10")
                                        .class("offset-md-1")
                                        .render(ProfileTabListView(&self.username))
                                        .list_with_render(
                                            state
                                                .article_list
                                                .iter()
                                                .flat_map(|al| {
                                                    al
                                                        .articles
                                                        .iter()
                                                }),
                                            spair::ListElementCreation::Clone,
                                            "div",
                                            |article, d| {
                                                let ap =  crate::renders::ArticlePreview {
                                                    article,
                                                    toggle_favorite_fn: super::Profile::toggle_favorite,
                                                };
                                                d
                                                    .class("article-preview")
                                                    .render_fn(|nodes| ap.render(nodes));
                                            }
                                        );
                                });
                        });
                });
        });
    }
}

struct ProfileView<'a>(&'a types::ProfileInfo);
impl<'a> spair::Render<super::Profile> for ProfileView<'a> {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes
            .img(|i| {
                i.class("user-image")
                    .src(&self.0.image);
            })
            .h4(|h| h.render(&self.0.username).done())
            .match_if(|mi| match self.0.bio.as_ref() {
                None => spair::set_arm!(mi).done(),
                Some(bio) => spair::set_arm!(mi).p(|p| p.render(bio).done()).done(),
            })
            .match_if(|mi| match state.is_logged_in_username(&self.0.username) {
                None => spair::set_arm!(mi).done(),
                Some(false) => spair::set_arm!(mi)
                    .a(|a| {
                        a.class("btn")
                        .class("btn-sm")
                        .class("btn-outline-secondary")
                        .class("action-btn")
                        .href(&crate::routes::Route::Settings)
                        .r#static("Edit Profile Settings");
                    })
                    .done(),
                Some(true) => spair::set_arm!(mi)
                    .button(|b| {
                        let username = self.0.username.clone();
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(
                                self.0.following,
                                "btn-secondary",
                                "btn-outline-secondary",
                            )
                            .class("action-btn")
                            .on_click(comp.handler(super::Profile::toggle_follow))
                            .i(|i| i.class("ion-plus-round").done())
                            .r#static(" Follow ")
                            .render(&self.0.username);
                    })
                    .done(),
            });
    }
}

struct ProfileTabListView<'a>(&'a str);
impl<'a> spair::Render<super::Profile> for ProfileTabListView<'a> {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        nodes
            .div(|d| {
                d.class("articles-toggle")
                    .ul(|u| {
                        u.class("nav")
                            .class("nav-pills")
                            .class("outline-active")
                            .render(ProfileTabView{
                                title: "Articles",
                                active: state.tab == super::ProfileTab::Articles,
                                route: crate::routes::Route::Profile(self.0.to_string()),
                            })
                            .render(ProfileTabView{
                                title: "Favorite Articles",
                                active: state.tab == super::ProfileTab::FavoritedArticles,
                                route: crate::routes::Route::ProfileFavorites(self.0.to_string()),
                            });
                    });
            });
    }
}

struct ProfileTabView {
    title: &'static str,
    active: bool,
    route: crate::routes::Route,
}
impl spair::Render<super::Profile> for ProfileTabView {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        nodes
            .li(|i| {
                i.class("nav-item")
                    .a(|a| {
                        a.class("nav-link")
                            .class_if(self.active, "active")
                            .href(&self.route)
                            .r#static(self.title);
                    });
            });
    }
}

