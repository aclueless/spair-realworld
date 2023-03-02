use spair::prelude::*;

pub fn render_error<C: spair::Component>(
    error: Option<&services::error::Error>,
    nodes: spair::Nodes<C>,
) {
    log::info!("render error: {}", error.is_some());
    nodes.ul(|u| {
        u.class("error-messages").match_if(|mi| match error {
            None => spair::set_arm!(mi).done(),
            Some(error) => match error {
                services::error::Error::UnprocessableEntity(error_info) => {
                    log::info!("error: {}", error_info.errors.len());
                    spair::set_arm!(mi)
                        .list_clone(error_info.errors.iter(), "li", |(key, values), li| {
                            li.update_text(key)
                                .list_clone(values.iter(), "span", |value, s| {
                                    s.static_text(" ").update_text(value).done()
                                });
                        })
                        .done();
                }
                _ => spair::set_arm!(mi)
                    .li(|li| li.update_text(&error.to_string()).done())
                    .done(),
            },
        });
    });
}
