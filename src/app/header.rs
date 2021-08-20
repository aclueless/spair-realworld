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
                    })
                    ;
            });
        });
    }
}

pub struct LoggedOutHeader;
impl spair::Render<crate::app::App> for LoggedOutHeader {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes
            .static_nodes()
            .ul(|u| {
                u.class("nav")
                    .class("navbar-nav")
                    .class("pull-xs-right")
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Home;
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .r#static("Home");
                        });
                    })
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Login;
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .r#static("Sign in");
                        });
                    })
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Register;
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .r#static("Sign up");
                        });
                    });
            });
    }
}

pub struct LoggedInHeader<'a>(&'a types::UserInfo);
impl<'a> spair::Render<crate::app::App> for LoggedInHeader<'a> {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        let state = nodes.state();
        nodes
            .static_nodes()
            .ul(|u| {
                u.class("nav")
                    .class("navbar-nav")
                    .class("pull-xs-right")
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Home;
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .r#static("Home");
                        });
                    })
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Editor(None);
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .i(|i| i.class("ion-compose").done())
                                .r#static("New Post");
                        });
                    })
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Settings;
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .i(|i| i.class("ion-gear-a").done())
                                .r#static("Settings");
                        });
                    })
                    .li(|i| {
                        i.class("nav-item").a(|a| {
                            let route = crate::routes::Route::Profile(self.0.username.clone());
                            a.class("nav-link")
                                .class_if("active", state.route == route)
                                .href(&route)
                                .render(&self.0.username);
                        });
                    });
            });
    }
}
