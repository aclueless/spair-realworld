use spair::prelude::*;

pub struct Header;
impl spair::Render<crate::app::App> for Header {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes.nav(|n| {
            n.class("navbar").class("navbar-light").div(|d| {
                d.class("container")
                    .a(|a| {
                        a.class("navbar-brand")
                            .href(&crate::routes::Route::Home)
                            .r#static("conduit");
                    })
                    .match_if(|mi| match state.user.as_ref() {
                        None => spair::set_arm!(mi).render(LoggedOutHeader).done(),
                        Some(user) => spair::set_arm!(mi).render(LoggedInHeader(user)).done(),
                    });
            });
        });
    }
}

pub struct LoggedOutHeader;
impl spair::Render<crate::app::App> for LoggedOutHeader {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes.static_nodes().ul(|u| {
            u.class("nav")
                .class("navbar-nav")
                .class("pull-xs-right")
                .render(HeaderLink {
                    title: "Home",
                    route: crate::routes::Route::Home,
                    icon: None,
                })
                .render(HeaderLink {
                    title: "Sign in",
                    route: crate::routes::Route::Login,
                    icon: None,
                })
                .render(HeaderLink {
                    title: "Sign up",
                    route: crate::routes::Route::Register,
                    icon: None,
                });
        });
    }
}

struct HeaderLink<'a> {
    title: &'a str,
    route: crate::routes::Route,
    icon: Option<&'static str>,
}
impl<'a> spair::Render<crate::app::App> for HeaderLink<'a> {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes.li(|i| {
            i.class("nav-item").a(|a| {
                a.class("nav-link")
                    .class_if(state.route == self.route, "active")
                    .href(&self.route)
                    .match_if(|mi| match self.icon {
                        None => spair::set_arm!(mi).done(),
                        Some(c) => spair::set_arm!(mi)
                            .i(|i| i.static_attributes().class(c).done())
                            .done(),
                    })
                    .r#static(self.title);
            });
        });
    }
}

pub struct LoggedInHeader<'a>(&'a types::UserInfo);
impl<'a> spair::Render<crate::app::App> for LoggedInHeader<'a> {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes.static_nodes().ul(|u| {
            u.class("nav")
                .class("navbar-nav")
                .class("pull-xs-right")
                .render(HeaderLink {
                    title: "Home",
                    route: crate::routes::Route::Home,
                    icon: None,
                })
                .render(HeaderLink {
                    title: "New Post",
                    route: crate::routes::Route::Editor(None),
                    icon: Some("ion-compose"),
                })
                .render(HeaderLink {
                    title: "Settings",
                    route: crate::routes::Route::Settings,
                    icon: Some("ion-gear-a"),
                })
                .render(HeaderLink {
                    title: &self.0.username,
                    route: crate::routes::Route::Profile(self.0.username.clone()),
                    icon: None,
                });
        });
    }
}
