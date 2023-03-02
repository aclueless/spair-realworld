use spair::prelude::*;

pub fn render_header(nodes: spair::Nodes<crate::app::App>) {
    let state = nodes.state();
    nodes.nav(|n| {
        n.class("navbar").class("navbar-light").div(|d| {
            d.class("container")
                .a(|a| {
                    a.class("navbar-brand")
                        .href(&crate::routes::Route::Home)
                        .static_text("conduit");
                })
                .match_if(|mi| match state.user.as_ref() {
                    None => spair::set_arm!(mi).rfn(render_logouted_header).done(),
                    Some(user) => spair::set_arm!(mi)
                        .rfn(|nodes| render_logined_header(user, nodes))
                        .done(),
                });
        });
    });
}

fn render_logouted_header(nodes: spair::Nodes<crate::app::App>) {
    nodes.ul(|u| {
        u.class("nav")
            .class("navbar-nav")
            .class("pull-xs-right")
            .rfn(|nodes| render_header_link("Home", crate::routes::Route::Home, None, nodes))
            .rfn(|nodes| render_header_link("Sign in", crate::routes::Route::Login, None, nodes))
            .rfn(|nodes| {
                render_header_link("Sign up", crate::routes::Route::Register, None, nodes)
            });
    });
}

fn render_header_link(
    title: &str,
    route: crate::routes::Route,
    icon: Option<&'static str>,
    nodes: spair::Nodes<crate::app::App>,
) {
    let state = nodes.state();
    nodes.li(|i| {
        i.class("nav-item").a(|a| {
            a.class("nav-link")
                .class_if(state.route == route, "active")
                .href(&route)
                .match_if(|mi| match icon {
                    None => spair::set_arm!(mi).done(),
                    Some(c) => spair::set_arm!(mi)
                        .i(|i| i.static_attributes().class(c).done())
                        .done(),
                })
                .static_text(title);
        });
    });
}

fn render_logined_header(ui: &types::UserInfo, nodes: spair::Nodes<crate::app::App>) {
    nodes.static_nodes().ul(|u| {
        u.class("nav")
            .class("navbar-nav")
            .class("pull-xs-right")
            .rfn(|nodes| render_header_link("Home", crate::routes::Route::Home, None, nodes))
            .rfn(|nodes| {
                render_header_link(
                    "New Post",
                    crate::routes::Route::Editor(None),
                    Some("ion-compose"),
                    nodes,
                )
            })
            .rfn(|nodes| {
                render_header_link(
                    "Settings",
                    crate::routes::Route::Settings,
                    Some("ion-gear-a"),
                    nodes,
                )
            })
            .rfn(|nodes| {
                render_header_link(
                    &ui.username,
                    crate::routes::Route::Profile(ui.username.clone()),
                    None,
                    nodes,
                )
            });
    });
}
