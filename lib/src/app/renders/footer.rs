use spair::prelude::*;

pub fn render_footer(nodes: spair::Nodes<crate::app::App>) {
    nodes.static_nodes().footer(|f| {
        f.div(|d| {
            d.class("container")
                .a(|a| {
                    a.class("logo-font")
                        .href(&crate::routes::Route::Home)
                        .static_text("conduit");
                })
                .span(|s| {
                    s.class("attribution")
                        .static_text("An interactive learning project from ")
                        .a(|a| {
                            a.href_str("https://thinkster.io").static_text("Thinkster");
                        })
                        .static_text(". Code &amp; design licensed under MIT.");
                });
        });
    });
}
