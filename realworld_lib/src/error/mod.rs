use spair::prelude::*;

pub struct ErrorView<'a>(pub Option<&'a realworld_shared::error::Error>);

impl<'a, C: spair::Component> spair::Render<C> for ErrorView<'a> {
    fn render(self, nodes: spair::Nodes<C>) {
        log::info!("render error: {}", self.0.is_some());
        nodes.ul(|u| {
            u.class("error-messages").match_if(|mi| match self.0 {
                None => spair::set_arm!(mi).done(),
                Some(error) => match error {
                    realworld_shared::error::Error::UnprocessableEntity(error_info) => {
                        log::info!("error: {}", error_info.errors.len());
                        spair::set_arm!(mi)
                            .lwr_clone(error_info.errors.iter(), "li", |(key, values), li| {
                                li.rupdate(key)
                                    .lwr_clone(values.iter(), "span", |value, s| {
                                        s.rstatic(" ").rupdate(value).done()
                                    });
                            })
                            .done();
                    }
                    _ => spair::set_arm!(mi)
                        .li(|li| li.rupdate(&error.to_string()).done())
                        .done(),
                },
            });
        });
    }
}
