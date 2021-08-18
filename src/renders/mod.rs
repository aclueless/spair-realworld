impl<C: spair::Component> spair::Render<C> for Option<&crate::error::Error> {
    fn render(self, nodes: spair::Nodes<C>) {
        nodes.ul(|u| {
            u.class("error-messages").match_if(|mi| match self {
                None => spair::set_arm!(mi).done(),
                Some(error) => match error {
                    crate::error::Error::UnprocessableEntity(error_info) => {
                        spair::set_arm!(mi)
                            .list_with_render(
                                error_info.errors.iter(),
                                spair::ListElementCreation::Clone,
                                "li",
                                |error, li| {
                                    li.render(&error);
                                }
                            )
                            .done();
                    }
                    _ => spair::set_arm!(mi).render(&error.to_string()).done(),
                },
            });
        });
    }
}
