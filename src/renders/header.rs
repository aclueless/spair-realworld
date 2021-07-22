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
                                        .r#static("&nbsp;New Post");
                                });
                            })
                            .li(|i| {
                                i.class("nav-item").a(|a| {
                                    let route = crate::routes::Route::Settings;
                                    a.class("nav-link")
                                        .class_if("active", state.route == route)
                                        .href(&route)
                                        .i(|i| i.class("ion-gear-a").done())
                                        .r#static("&nbsp;Settings");
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
            });
        });
    }
}
