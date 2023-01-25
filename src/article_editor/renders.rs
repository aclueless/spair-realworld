use spair::prelude::*;

impl spair::Component for super::ArticleEditor {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback_once_mut(super::ArticleEditor::get_article)
            .queue();
    }

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("editor-page").div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-10")
                            .class("offset-md-1")
                            .class("col-xs-12")
                            .rfn(|nodes| self.render_form(nodes));
                    });
                });
            });
        });
    }
}

impl spair::AsChildComp for super::ArticleEditor {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = super::Props;
    fn init(_: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(props)
    }
}

impl super::ArticleEditor {
    fn render_form(&self, nodes: spair::Nodes<Self>) {
        let comp = nodes.comp();
        nodes.form(|f| {
            f.fieldset(|f| {
                f.fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.article.title)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(
                                move |state, event: spair::InputEvent| {
                                    if let Some(input) = event.current_target_as_input_element() {
                                        state.set_title(input.value());
                                    }
                                },
                            ))
                            .input_type(spair::InputType::Text)
                            .class("form-control")
                            .class("form-control-lg")
                            .placeholder("Article Title");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.value(&self.article.description)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(
                                move |state, event: spair::InputEvent| {
                                    if let Some(input) = event.current_target_as_input_element() {
                                        state.set_description(input.value());
                                    }
                                },
                            ))
                            .input_type(spair::InputType::Text)
                            .class("form-control")
                            .placeholder("What's this article about?");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").textarea(|t| {
                        t.value(&self.article.body)
                            .static_attributes()
                            .on_input(comp.handler_arg_mut(
                                move |state, event: spair::InputEvent| {
                                    if let Some(input) =
                                        event.target_as::<spair::web_sys::HtmlTextAreaElement>()
                                    {
                                        state.set_body(input.value());
                                    }
                                },
                            ))
                            .input_type(spair::InputType::Text)
                            .rows(8)
                            .class("form-control")
                            .placeholder("Write your article (in markdown)");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group")
                        .input(|i| {
                            i //.value(&self.tag_string)
                                .static_attributes()
                                .on_key_up(comp.handler_arg_mut(
                                    move |state, event: spair::KeyboardEvent| {
                                        if event.raw().code() != "Enter" {
                                            return;
                                        }
                                        if let Some(input) =
                                            event.target_as::<spair::web_sys::HtmlInputElement>()
                                        {
                                            state.add_tag(input.value());
                                            input.set_value("");
                                        }
                                    },
                                ))
                                .input_type(spair::InputType::Text)
                                .class("form-control")
                                .placeholder("Enter tags");
                        })
                        .div(|d| {
                            d.class("tag-list").list_with_render(
                                self.article.tag_list.iter().flat_map(|tags| tags.iter()),
                                spair::ListElementCreation::Clone,
                                "span",
                                |tag, s| {
                                    s.static_attributes()
                                        .class("tag-default")
                                        .class("tag-pill")
                                        .i(|i| {
                                            let tag = tag.to_string();
                                            i.on_click(
                                                comp.handler_mut(move |state| {
                                                    state.remove_tag(&tag)
                                                }),
                                            )
                                            .static_attributes()
                                            .class("ion-close-round");
                                        })
                                        .rupdate(tag);
                                },
                            );
                        });
                })
                .static_nodes()
                .button(|b| {
                    b.class("btn")
                        .class("btn")
                        .class("btn-lg")
                        .class("pull-xs-right")
                        .class("btn-primary")
                        .button_type(spair::ButtonType::Button)
                        .on_click(comp.handler(super::ArticleEditor::publish_article))
                        .rupdate("Publish Article");
                });
            });
        });
    }
}
