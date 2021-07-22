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
                            .href_str("index.html")
                            .r#static("conduit");
                    })
                    .ul(|u| {
                        u.class("nav")
                            .class("navbar-nav")
                            .class("pull-xs-right")
                            .li(|i| {
                                i.class("nav-item").a(|a| {
                                    a.class("nav-link")
                                        .class_if("active", state.is_at_home())
                                        .href_str("")
                                        .r#static("Home");
                                });
                            })
                            .li(|i| {
                                i.class("nav-item").a(|a| {
                                    a.class("nav-link")
                                        .class_if("active", state.is_at_new_post())
                                        .href_str("")
                                        .i(|i| i.class("ion-compose").done())
                                        .r#static("&nbsp;New Post");
                                });
                            })
                            .li(|i| {
                                i.class("nav-item").a(|a| {
                                    a.class("nav-link")
                                        .class_if("active", state.is_at_settings())
                                        .href_str("")
                                        .i(|i| i.class("ion-gear-a").done())
                                        .r#static("&nbsp;Settings");
                                });
                            })
                            .li(|i| {
                                i.class("nav-item").a(|a| {
                                    a.class("nav-link")
                                        .class_if("active", state.is_at_sign_up())
                                        .href_str("")
                                        .r#static("Sign up");
                                });
                            });
                    });
            });
        });
    }
}
