use spair::prelude::*;

impl spair::Component for super::Settings {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("settings-page")
            .match_if(|mi| match self.user_info.as_ref(){
                None => spair::set_arm!(mi).r#static("Sign in to view your settings.").done(),
                Some(_) => spair::set_arm!(mi).div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-6")
                            .class("offset-md-3")
                            .class("col-xs-12")
                            .h1(|h| {
                                h.class("text-xs-center").r#static("Your Settings");
                            })
                            .render(crate::error::ErrorView(self.error.as_ref()))
                            .render(&self.user_update_info);
                    });
                });
            }).done(),
            });
        });
    }
}

impl spair::WithParentComp for super::Settings {
    type Parent = crate::app::App;
    type Properties = Option<types::UserInfo>;
    fn init(parent: &spair::Comp<Self::Parent>, _comp: &spair::Comp<Self>, user_info: Self::Properties) -> Self {
        Self::new(parent.clone(), user_info)
    }
}

impl spair::Render<super::Settings> for &types::UserUpdateInfo {
    fn render(self, nodes: spair::Nodes<super::Settings>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.form(|f| {
            f.fieldset(|f| {
                f.fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i
                            .value(&self.image)
                            .class("form-control")
                            .r#type(spair::InputType::Text)
                            .placeholder("URL of profile picture")
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_image(input.value());
                                }
                            }));
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i
                            .value(&self.username)
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Text)
                            .placeholder("Your Name")
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_username(input.value());
                                }
                            }));
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").textarea(|t| {
                        t
                            .value(&self.bio)
                            .class("form-control")
                            .class("form-control-lg")
                            .rows(8)
                            .placeholder("Short bio about you")
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(ta) = event.target_as::<spair::web_sys::HtmlTextAreaElement>() {
                                    state.set_bio(ta.value());
                                }
                            }));
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i
                            .value(&self.email)
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Email)
                            .placeholder("Email")
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_email(input.value());
                                }
                            }));
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i
                            .value(&state.new_password)
                            .class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Password)
                            .placeholder("Password")
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(input) = event.target_as_input_element() {
                                    state.set_password(input.value());
                                }
                            }));
                    });
                })
                .button(|b| {
                    b.class("btn btn-lg")
                        .class("btn-primary")
                        .class("pull-xs-right")
                        .r#type(spair::ButtonType::Button)
                        .enabled(state.is_valid())
                        .on_click(comp.handler(super::Settings::request_update_user_info))
                        .r#static("Update settings");
                });
            });
        })
        .horizontal_line()
        .button(|b| {
            b.class("btn btn-lg")
                .class("btn-outline-danger")
                .on_click(comp.handler(super::Settings::logout))
                .r#static("Or click here to logout.");
        });
    }
}
