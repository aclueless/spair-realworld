use spair::prelude::*;

impl spair::Component for super::Editor {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        spair::update_component(comp.callback_once_mut(super::Editor::get_article));
    }

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("editor-page").div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-10")
                            .class("offset-md-1")
                            .class("col-xs-12")
                            .render_fn(|nodes| self.render_form(nodes));
                    });
                });
            });
        });
    }
}

impl spair::WithParentComp for super::Editor {
    type Parent = crate::app::App;
    type Properties = Option<types::Slug>;
    fn init(
        parent: &spair::Comp<Self::Parent>,
        _: &spair::Comp<Self>,
        slug: Self::Properties,
    ) -> Self {
        Self::new(parent.clone(), slug)
    }
}

impl super::Editor {
    fn render_form(&self, nodes: spair::Nodes<Self>) {
        let comp = nodes.comp();
        nodes.form(|f| {
            f.fieldset(|f| {
                f.fieldset(|f| {
                    f.class("form-group")
                        .input(|i| {
                            i.value(&self.article.title)
                                .static_attributes()
                                .on_input(comp.handler_arg_mut(move |state, event: spair::InputEvent| {
                                    if let Some(input) = event.target_as_input_element() {
                                        state.set_title(input.value());
                                    }
                                }))
                                .r#type(spair::InputType::Text)
                                .class("form-control")
                                .class("form-control-lg")
                                .placeholder("Article Title");
                        });
                })
                .fieldset(|f| {
                    f.class("form-group")
                        .input(|i| {
                            i.value(&self.article.description)
                                .static_attributes()
                                .on_input(comp.handler_arg_mut(move |state, event: spair::InputEvent| {
                                    if let Some(input) = event.target_as_input_element() {
                                        state.set_description(input.value());
                                    }
                                }))
                                .r#type(spair::InputType::Text)
                                .class("form-control")
                                .placeholder("What's this article about?");
                        });
                })
                .fieldset(|f| {
                    f.class("form-group")
                        .textarea(|i| {
                            i.value(&self.article.body)
                                .static_attributes()
                                .on_input(comp.handler_arg_mut(move |state, event: spair::InputEvent| {
                                    if let Some(input) = event.target_as_input_element() {
                                        state.set_body(input.value());
                                    }
                                }))
                                .r#type(spair::InputType::Text)
                                .rows(8)
                                .class("form-control")
                                .placeholder("Write your article (in markdown)");
                        });
                })
                .fieldset(|f| {
                    f.class("form-group")
                        .input(|i| {
                            i//.value(&self.tag_string)
                                .static_attributes()
                                .on_key_up(comp.handler_arg_mut(move |state, event: spair::KeyboardEvent| {
                                    if event.raw().code() != "Enter" {
                                        return;
                                    }
                                    if let Some(input) = event.target_as::<spair::web_sys::HtmlInputElement>() {
                                        state.add_tag(input.value());
                                        input.set_value("");
                                    }
                                }))
                                .r#type(spair::InputType::Text)
                                .class("form-control")
                                .placeholder("Enter tags");
                        })
                        .div(|d| {
                            d.class("tag-list")
                                .list_with_render(
                                    self.article.tag_list.iter().flat_map(|tags| tags.iter()),
                                    spair::ListElementCreation::Clone,
                                    "span",
                                    |tag, s| {
                                        s
                                            .static_attributes()
                                            .class("tag-default")
                                            .class("tag-pill")
                                            .i(|i| {
                                                let tag = tag.to_string();
                                                i.on_click(comp.handler_mut(move |state| state.remove_tag(&tag)))
                                                    .static_attributes()
                                                    .class("ion-close-round");
                                            })
                                            .render(tag);
                                    }
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
                        .r#type(spair::ButtonType::Button)
                        .on_click(comp.handler(super::Editor::publish_article))
                        .render("Publish Article");
                });
            });
        });
    }
}

/*
<div class="editor-page">
  <div class="container page">
    <div class="row">

      <div class="col-md-10 offset-md-1 col-xs-12">
        <form>
          <fieldset>
            <fieldset class="form-group">
                <input type="text" class="form-control form-control-lg" placeholder="Article Title">
            </fieldset>
            <fieldset class="form-group">
                <input type="text" class="form-control" placeholder="What's this article about?">
            </fieldset>
            <fieldset class="form-group">
                <textarea class="form-control" rows="8" placeholder="Write your article (in markdown)"></textarea>
            </fieldset>
            <fieldset class="form-group">
                <input type="text" class="form-control" placeholder="Enter tags"><div class="tag-list"></div>
            </fieldset>
            <button class="btn btn-lg pull-xs-right btn-primary" type="button">
                Publish Article
            </button>
          </fieldset>
        </form>
      </div>

    </div>
  </div>
</div>
*/
