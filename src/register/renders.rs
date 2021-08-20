use spair::prelude::*;

impl spair::Component for super::Register {
    type Routes = crate::routes::Route;
    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("auth-page").div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-6")
                            .class("offset-md-3")
                            .class("col-xs-12")
                            .render_fn(|nodes| self.render_register(nodes));
                    });
                });
            });
        });
    }
}

impl super::Register {
    fn render_register(&self, nodes: spair::Nodes<Self>) {
        let comp = nodes.comp();
        nodes
            .h1(|h| h.class("text-xs-center").r#static("Sign up").done())
            .p(|p| {
                p.class("text-xs-center").a(|a| {
                    a.href(&crate::routes::Route::Login)
                        .r#static("Have an account?");
                });
            })
            .render(crate::renders::Error(self.error.as_ref()))
            .form(|f| {
                f
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.register_info.username)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(value) =
                                    event.target_as_input_element().map(|i| i.value())
                                {
                                    state.set_username(value);
                                }
                            }))
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Text)
                            .placeholder("Your Name");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.register_info.email)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(value) =
                                    event.target_as_input_element().map(|i| i.value())
                                {
                                    state.set_email(value);
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
                        i.value(&self.register_info.password)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(value) =
                                    event.target_as_input_element().map(|i| i.value())
                                {
                                    state.set_password(value);
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
                        .on_click(comp.handler_mut(super::Register::send_register_request))
                        .r#static("Sign up");
                });
            });
    }
}

impl spair::WithParentComp for super::Register {
    type Parent = crate::app::App;
    type Properties = ();
    fn init(parent: &spair::Comp<Self::Parent>, _: &spair::Comp<Self>, _: Self::Properties) -> Self {
        Self::new(parent.clone())
    }
}
