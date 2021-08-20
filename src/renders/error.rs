use spair::prelude::*;

pub struct Error<'a>(pub Option<&'a crate::error::Error>);

impl<'a, C: spair::Component> spair::Render<C> for Error<'a> {
    fn render(self, nodes: spair::Nodes<C>) {
        log::info!("render error: {}", self.0.is_some());
        nodes.ul(|u| {
            u.class("error-messages").match_if(|mi| match self.0 {
                None => spair::set_arm!(mi).done(),
                Some(error) => match error {
                    crate::error::Error::UnprocessableEntity(error_info) => {
                        log::info!("error: {}", error_info.errors.len());
                        spair::set_arm!(mi)
                            .list_with_render(
                                error_info.errors.iter(),
                                spair::ListElementCreation::Clone,
                                "li",
                                |(key, values), li| {
                                    li.render(key).list_with_render(
                                        values.iter(),
                                        spair::ListElementCreation::Clone,
                                        "span",
                                        |value, s| s.r#static(" ").render(value).done(),
                                    );
                                },
                            )
                            .done();
                    }
                    _ => spair::set_arm!(mi).render(&error.to_string()).done(),
                },
            });
        });
    }
}
