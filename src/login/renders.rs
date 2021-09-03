use spair::prelude::*;

impl spair::Component for super::Login {
    type Routes = crate::routes::Route;
    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("auth-page").div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-6")
                            .class("offset-md-3")
                            .class("col-xs-12")
                            .render_fn(|nodes| self.render_login(nodes));
                    });
                });
            });
        });
    }
}

impl super::Login {
    fn render_login(&self, nodes: spair::Nodes<Self>) {
        let comp = nodes.comp();
        nodes
            .h1(|h| h.class("text-xs-center").r#static("Sign in").done())
            .p(|p| {
                p.class("text-xs-center").a(|a| {
                    a.href(&crate::routes::Route::Register)
                        .r#static("Need an account?");
                });
            })
            .render(crate::error::ErrorView(self.error.as_ref()))
            .form(|f| {
                f.fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.login_info.email)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_email(input.value());
                                }
                            }))
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Text)
                            .placeholder("Email");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.login_info.password)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_password(input.value());
                                }
                            }))
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Password)
                            .placeholder("Password");
                    });
                })
                .static_nodes()
                .button(|b| {
                    b.class("btn")
                        .class("btn-lg")
                        .class("btn-primary")
                        .class("pull-xs-right")
                        .r#type(spair::ButtonType::Button)
                        .on_click(comp.handler_mut(super::Login::send_login_request))
                        .r#static("Sign in");
                });
            });
    }
}

impl spair::WithParentComp for super::Login {
    type Parent = crate::app::App;
    type Properties = ();
    fn init(
        parent: &spair::Comp<Self::Parent>,
        _: &spair::Comp<Self>,
        _: Self::Properties,
    ) -> Self {
        Self::new(parent.clone())
    }
}
