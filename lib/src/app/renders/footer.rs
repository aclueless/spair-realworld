use spair::prelude::*;

pub struct Footer;
impl spair::Render<crate::app::App> for Footer {
    fn render(self, nodes: spair::Nodes<crate::app::App>) {
        nodes.static_nodes().footer(|f| {
            f.div(|d| {
                d.class("container")
                    .a(|a| {
                        a.class("logo-font")
                            .href(&crate::routes::Route::Home)
                            .rstatic("conduit");
                    })
                    .span(|s| {
                        s.class("attribution")
                            .rstatic("An interactive learning project from ")
                            .a(|a| {
                                a.href_str("https://thinkster.io").rstatic("Thinkster");
                            })
                            .rstatic(". Code &amp; design licensed under MIT.");
                    });
            });
        });
    }
}
