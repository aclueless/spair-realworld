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
                    Some(profile) => spair::set_arm!(mi).rupdate(profile).done(),
                })
                .div(|d| {
                    d.class("container").div(|d| {
                        d.class("row").div(|d| {
                            d.class("col-xs-12")
                                .class("col-md-10")
                                .class("offset-md-1")
                                .rupdate(ProfileTabListView(&self.profile_username))
                                .div(|d| d.component(&self.article_list_comp));
                        });
                    });
                });
        });
    }
}

impl spair::AsChildComp for super::Profile {
    type Properties = super::Props;
    fn init(_comp: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(props)
    }
}

impl spair::Render<super::Profile> for &ProfileInfo {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.class("user-info").div(|d| {
                d.class("container").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-xs-12")
                            .class("col-md-10")
                            .class("offset-md-1")
                            .rupdate(ProfileView(self));
                    });
                });
            });
        });
    }
}

struct ProfileView<'a>(&'a ProfileInfo);
impl<'a> spair::Render<super::Profile> for ProfileView<'a> {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes
            .img(|i| {
                i.class("user-image").src(&self.0.image);
            })
            .h4(|h| h.rupdate(&self.0.username).done())
            .match_if(|mi| match self.0.bio.as_ref() {
                None => spair::set_arm!(mi).done(),
                Some(bio) => spair::set_arm!(mi).p(|p| p.rupdate(bio).done()).done(),
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
                            .rstatic("Edit Profile Settings");
                    })
                    .done(),
                Some(true) => spair::set_arm!(mi)
                    .button(|b| {
                        let username = self.0.username.clone();
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(self.0.following, "btn-secondary", "btn-outline-secondary")
                            .class("action-btn")
                            .on_click(comp.handler(super::Profile::toggle_follow))
                            .i(|i| i.class("ion-plus-round").done())
                            .rstatic(" Follow ")
                            .rupdate(&self.0.username);
                    })
                    .done(),
            });
    }
}

struct ProfileTabListView<'a>(&'a str);
impl<'a> spair::Render<super::Profile> for ProfileTabListView<'a> {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        let state = nodes.state();
        nodes.div(|d| {
            d.class("articles-toggle").ul(|u| {
                u.class("nav")
                    .class("nav-pills")
                    .class("outline-active")
                    .rupdate(ProfileTabView {
                        title: "Articles",
                        active: !state.favorited,
                        route: crate::routes::Route::Profile(self.0.to_string()),
                    })
                    .rupdate(ProfileTabView {
                        title: "Favorite Articles",
                        active: state.favorited,
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
        nodes.li(|i| {
            i.class("nav-item").a(|a| {
                a.class("nav-link")
                    .class_if(self.active, "active")
                    .href(&self.route)
                    .rstatic(self.title);
            });
        });
    }
}
