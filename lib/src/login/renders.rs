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
                            .rfn(|nodes| self.render_login(nodes));
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
            .h1(|h| h.class("text-xs-center").rstatic("Sign in").done())
            .p(|p| {
                p.class("text-xs-center").a(|a| {
                    a.href(&crate::routes::Route::Register)
                        .rstatic("Need an account?");
                });
            })
            .rupdate(crate::error::ErrorView(self.error.as_ref()))
            .form(|f| {
                f.fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.login_info.email)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.current_target_as_input_element() {
                                    state.set_email(input.value());
                                }
                            }))
                            .class("form-control")
                            .class("form-control-lg")
                            .input_type(spair::InputType::Email)
                            .placeholder("Email");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.login_info.password)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.current_target_as_input_element() {
                                    state.set_password(input.value());
                                }
                            }))
                            .class("form-control")
                            .class("form-control-lg")
                            .input_type(spair::InputType::Password)
                            .placeholder("Password");
                    });
                })
                .static_nodes()
                .button(|b| {
                    b.class("btn")
                        .class("btn-lg")
                        .class("btn-primary")
                        .class("pull-xs-right")
                        .button_type(spair::ButtonType::Button)
                        .on_click(comp.handler_mut(super::Login::send_login_request))
                        .rstatic("Sign in");
                });
            });
    }
}

impl spair::AsChildComp for super::Login {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = spair::CallbackArg<types::UserInfoWrapper>;
    fn init(comp: &spair::Comp<Self>, set_user_callback: Self::Properties) -> Self {
        Self::new(comp.clone(), set_user_callback)
    }
}
