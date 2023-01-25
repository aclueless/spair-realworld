use spair::prelude::*;

impl spair::Component for super::Settings {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("settings-page")
                .match_if(|mi| match self.user_info.as_ref() {
                    None => spair::set_arm!(mi)
                        .rstatic("Sign in to view your settings.")
                        .done(),
                    Some(_) => spair::set_arm!(mi)
                        .div(|d| {
                            d.class("container").class("page").div(|d| {
                                d.class("row").div(|d| {
                                    d.class("col-md-6")
                                        .class("offset-md-3")
                                        .class("col-xs-12")
                                        .h1(|h| {
                                            h.class("text-xs-center").rstatic("Your Settings");
                                        })
                                        .rupdate(crate::error::ErrorView(self.error.as_ref()))
                                        .rupdate(&self.user_update_info);
                                });
                            });
                        })
                        .done(),
                });
        });
    }
}

impl spair::AsChildComp for super::Settings {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = super::Props;
    fn init(_comp: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(props)
    }
}

impl spair::Render<super::Settings> for &realworld_shared::types::UserUpdateInfo {
    fn render(self, nodes: spair::Nodes<super::Settings>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes
            .form(|f| {
                f.fieldset(|f| {
                    f.fieldset(|f| {
                        f.class("form-group").input(|i| {
                            i.value(&self.image)
                                .class("form-control")
                                .input_type(spair::InputType::Text)
                                .placeholder("URL of profile picture")
                                .on_input(comp.handler_arg_mut(
                                    |state, event: spair::InputEvent| {
                                        if let Some(input) = event.current_target_as_input_element() {
                                            state.set_image(input.value());
                                        }
                                    },
                                ));
                        });
                    })
                    .fieldset(|f| {
                        f.class("form-group").input(|i| {
                            i.value(&self.username)
                                .class("form-control")
                                .class("form-control-lg")
                                .input_type(spair::InputType::Text)
                                .placeholder("Your Name")
                                .on_input(comp.handler_arg_mut(
                                    |state, event: spair::InputEvent| {
                                        if let Some(input) = event.current_target_as_input_element() {
                                            state.set_username(input.value());
                                        }
                                    },
                                ));
                        });
                    })
                    .fieldset(|f| {
                        f.class("form-group").textarea(|t| {
                            t.value(&self.bio)
                                .class("form-control")
                                .class("form-control-lg")
                                .rows(8)
                                .placeholder("Short bio about you")
                                .on_input(comp.handler_arg_mut(
                                    |state, event: spair::InputEvent| {
                                        if let Some(ta) =
                                            event.target_as::<spair::web_sys::HtmlTextAreaElement>()
                                        {
                                            state.set_bio(ta.value());
                                        }
                                    },
                                ));
                        });
                    })
                    .fieldset(|f| {
                        f.class("form-group").input(|i| {
                            i.value(&self.email)
                                .class("form-control")
                                .class("form-control-lg")
                                .input_type(spair::InputType::Email)
                                .placeholder("Email")
                                .on_input(comp.handler_arg_mut(
                                    |state, event: spair::InputEvent| {
                                        if let Some(input) = event.current_target_as_input_element() {
                                            state.set_email(input.value());
                                        }
                                    },
                                ));
                        });
                    })
                    .fieldset(|f| {
                        f.class("form-group").input(|i| {
                            i.value(&state.new_password)
                                .class("form-control")
                                .class("form-control-lg")
                                .input_type(spair::InputType::Password)
                                .placeholder("Password")
                                .on_input(comp.handler_arg_mut(
                                    |state, event: spair::InputEvent| {
                                        if let Some(input) = event.current_target_as_input_element() {
                                            state.set_password(input.value());
                                        }
                                    },
                                ));
                        });
                    })
                    .button(|b| {
                        b.class("btn btn-lg")
                            .class("btn-primary")
                            .class("pull-xs-right")
                            .button_type(spair::ButtonType::Button)
                            .enabled(state.is_valid())
                            .on_click(comp.handler(super::Settings::request_update_user_info))
                            .rstatic("Update settings");
                    });
                });
            })
            .horizontal_line()
            .button(|b| {
                b.class("btn btn-lg")
                    .class("btn-outline-danger")
                    .on_click(comp.handler(super::Settings::logout))
                    .rstatic("Or click here to logout.");
            });
    }
}
