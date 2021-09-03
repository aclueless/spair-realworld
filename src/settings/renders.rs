use spair::prelude::*;

impl spair::Component for super::Settings {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element
            .div(|d| {
                d.class("settings-page")
                    .div(|d| {
                        d.class("container")
                            .class("page")
                            .div(|d| {
                                d.class("row")
                                    .div(|d| {
                                        d.class("col-md-6")
                                            .class("offset-md-3")
                                            .class("col-xs-12")
                                            .h1(|h| {
                                                h.class("text-xs-center")
                                                    .r#static("Your Settings");
                                            })
                                            .render(&self.user_info);
                                    });
                            });
                    });
            });
    }
}

impl spair::Render<super::Settings> for &types::UserUpdateInfo {
    fn render(self, nodes: spair::Nodes<super::Settings>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes
            .form(|f| {
                f.fieldset(|f| {
                    f.fieldset(|f| {
                        f.class("form-group")
                            .input(|i| {
                                i.class("form-control")
                                    .r#type(spair::InputType::Text)
                                    .placeholder("URL of profile picture")
                                    .value(&self.image);
                            });
                    }).fieldset(|f| {
                        f.class("form-group")
                            .input(|i| {
                                i.class("form-control")
                                    .class("form-control-lg")
                                    .r#type(spair::InputType::Text)
                                    .placeholder("Your Name")
                                    .value(&self.username);
                            });
                    }).fieldset(|f| {
                        f.class("form-group")
                            .textarea(|t| {
                                t.class("form-control")
                                    .class("form-control-lg")
                                    .rows(8)
                                    .placeholder("Short bio about you")
                                    .value(&self.bio);
                            });
                    }).fieldset(|f| {
                        f.class("form-group")
                            .input(|i| {
                                i.class("form-control")
                                    .class("form-control-lg")
                                    .r#type(spair::InputType::Email)
                                    .placeholder("Email")
                                    .value(&self.email);
                            });
                    }).fieldset(|f| {
                        f.class("form-group")
                            .input(|i| {
                                i.class("form-control")
                                    .class("form-control-lg")
                                    .r#type(spair::InputType::Password)
                                    .placeholder("Password")
                                    .value(&state.password);
                            });
                    });
                });
            });
    }
}
