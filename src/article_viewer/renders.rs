use spair::prelude::*;

impl spair::Component for super::ArticleViewer {
    type Routes = crate::routes::Route;

    fn init(comp: &spair::Comp<Self>) {
        spair::update_component(comp.callback(Self::get_article));
    }

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("article-page")
                .match_if(|mi| match self.article.as_ref() {
                    None => spair::set_arm!(mi).done(),
                    Some(article) => spair::set_arm!(mi).render(article).done(),
                });
        });
    }
}

impl spair::WithParentComp for super::ArticleViewer {
    type Parent = crate::app::App;
    type Properties = (Option<types::UserInfo>, super::ArticleToView);

    fn init(
        _parent: &spair::Comp<Self::Parent>,
        _comp: &spair::Comp<Self>,
        props: Self::Properties,
    ) -> Self {
        Self::new(props)
    }
}

impl spair::Render<super::ArticleViewer> for &types::ArticleInfo {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let state = nodes.state();
        nodes
            .div(|d| {
                d.class("banner").div(|d| {
                    d.class("container")
                        .h1(|h| h.render(&self.title).done())
                        .render(ArticleMeta(self))
                        .render(crate::renders::Error(state.error.as_ref()));
                });
            })
            .div(|d| {
                d.class("container")
                    .class("page")
                    .div(|d| {
                        d.class("row")
                            .class("article-content")
                            .div(|d| {
                                let parser = pulldown_cmark::Parser::new(&self.body);
                                let mut html_text = String::new();
                                pulldown_cmark::html::push_html(&mut html_text, parser);
                                d.class("col-md-12")
                                    .set_inner_html_raw(&html_text);
                            })
                            .ul(|u| {
                                u.class("tag-list").list_with_render(
                                    self.tag_list.iter(),
                                    spair::ListElementCreation::Clone,
                                    "li",
                                    |tag, li| {
                                        li.class("tag-default")
                                            .class("tag-pill")
                                            .class("tag-outline")
                                            .render(tag);
                                    },
                                );
                            });
                    })
                    .horizontal_line()
                    .div(|d| {
                        d.class("article-actions").render(ArticleMeta(self));
                    })
                    .div(|d| {
                        d.class("row").div(|d| {
                            d.class("col-xs-12")
                                .class("col-md-8")
                                .class("offset-md-2")
                                .match_if(|mi| match state.user.as_ref() {
                                    None => spair::set_arm!(mi).render(LoginRegister).done(),
                                    Some(user) => {
                                        spair::set_arm!(mi).render(CommentForm(user)).done()
                                    }
                                })
                                .list(
                                    state.comments.iter().flat_map(|v| v.iter()),
                                    spair::ListElementCreation::Clone,
                                );
                        });
                    });
            });
    }
}

struct ArticleMeta<'a>(&'a types::ArticleInfo);
impl<'a> spair::Render<super::ArticleViewer> for ArticleMeta<'a> {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let profile = crate::routes::Route::Profile(self.0.author.username.clone());
        nodes.div(|d| {
            d.class("article-meta")
                .a(|a| {
                    a.href(&profile).img(|i| {
                        i.src(&self.0.author.image).alt(&self.0.author.username);
                    });
                })
                .div(|d| {
                    d.class("info")
                        .a(|a| {
                            a.href(&profile)
                                .class("author")
                                .render(&self.0.author.username);
                        })
                        .span(|s| {
                            s.class("date").render(&self.0.created_at.to_string());
                        });
                })
                .render(ArticleActions(self.0));
        });
    }
}

struct ArticleActions<'a>(&'a types::ArticleInfo);
impl<'a> spair::Render<super::ArticleViewer> for ArticleActions<'a> {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.match_if(
            |mi| match state.is_logged_in_username(&self.0.author.username) {
                Some(true) => spair::set_arm!(mi)
                    .a(|a| {
                        a.class("btn")
                            .class("btn-sm")
                            .class("btn-outline-secondary")
                            .href(&crate::routes::Route::Editor(Some(state.slug.clone())))
                            .i(|i| i.class("ion-edit").done())
                            .r#static("Edit Article");
                    })
                    .button(|b| {
                        let username = self.0.author.username.clone();
                        b.class("btn")
                            .class("btn-sm")
                            .class("btn-outline-danger")
                            .on_click(comp.handler(super::ArticleViewer::delete_article))
                            .i(|i| i.class("ion-trash-a").done())
                            .r#static("Delete Article");
                    })
                    .done(),
                Some(false) => spair::set_arm!(mi)
                    .button(|b| {
                        let username = self.0.author.username.clone();
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(
                                self.0.author.following,
                                "btn-secondary",
                                "btn-outline-secondary",
                            )
                            .on_click(comp.handler(super::ArticleViewer::toggle_follow))
                            .i(|i| i.class("ion-plus-round").done())
                            .r#static(" Follow")
                            .render(&self.0.author.username);
                    })
                    .r#static("\u{00A0}\u{00A0}")
                    .button(|b| {
                        let username = self.0.author.username.clone();
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(self.0.favorited, "btn-primary", "btn-outline-primary")
                            .on_click(comp.handler(super::ArticleViewer::toggle_favorite))
                            .i(|i| i.class("ion-heart").done())
                            .r#static(" Favorite Post ")
                            .span(|s| {
                                s.class("counter")
                                    .r#static("(")
                                    .render(self.0.favorites_count)
                                    .r#static(")");
                            });
                    })
                    .done(),
                None => spair::set_arm!(mi).done(),
            },
        );
    }
}

struct LoginRegister;
impl spair::Render<super::ArticleViewer> for LoginRegister {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        nodes
            .static_nodes()
            .a(|a| {
                a.href(&crate::routes::Route::Login).r#static("Sign in");
            })
            .r#static(" or ")
            .a(|a| {
                a.href(&crate::routes::Route::Register).r#static("Sign up");
            })
            .r#static(" to comment.");
    }
}

struct CommentForm<'a>(&'a types::UserInfo);
impl<'a> spair::Render<super::ArticleViewer> for CommentForm<'a> {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes.form(|f| {
            f.class("card")
                .class("comment-form")
                .div(|d| {
                    d.class("card-block").textarea(|t| {
                        t.class("form-control")
                            .placeholder("Write a comment...")
                            .rows(3)
                            .value(&state.new_comment)
                            .on_input(comp.handler_arg_mut(|state, event: spair::InputEvent| {
                                if let Some(ta) = event.target_as::<spair::web_sys::HtmlTextAreaElement>() {
                                    state.set_new_comment(ta.value());
                                }
                            }));
                    });
                })
                .div(|d| {
                    d.class("card-footer")
                        .match_if(|mi| match self.0.image.as_ref() {
                            None => spair::set_arm!(mi).done(),
                            Some(image) => spair::set_arm!(mi)
                                .img(|i| {
                                    i.class("comment-author-img").src(image);
                                })
                                .done(),
                        })
                        .button(|b| {
                            b.class("btn")
                                .class("btn-sm")
                                .class("btn-primary")
                                .on_click(comp.handler(super::ArticleViewer::post_comment))
                                .enabled(state.new_comment.is_empty() == false)
                                .r#static("Post comment");
                        });
                });
        });
    }
}

impl spair::ListItemRender<super::ArticleViewer> for &types::CommentInfo {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::ArticleViewer>) {
        let comp = element.comp();
        element.div(|d| {
            d.class("card")
                .div(|d| {
                    d.class("card-block").render(&self.body);
                })
                .div(|d| {
                    let profile = crate::routes::Route::Profile(self.author.username.clone());
                    d.class("card-footer")
                        .a(|a| {
                            a.class("comment-author").href(&profile).img(|i| {
                                i.class("comment-author-img").src(&self.author.image);
                            });
                        })
                        .r#static(" ")
                        .a(|a| {
                            a.class("comment-author")
                                .href(&profile)
                                .render(&self.author.username);
                        })
                        .span(|s| {
                            s.class("date-posted").render(&self.created_at.to_string());
                        })
                        .match_if(|mi| {
                            match mi.state().is_logged_in_username(&self.author.username) {
                                Some(true) => spair::set_arm!(mi)
                                    .span(|s| {
                                        s.class("mod-options").i(|i| {
                                            let comment_id = self.id;
                                            i.class("ion-trash-a").on_click(
                                                comp.handler_mut(move |state| state.delete_comment(comment_id)),
                                            );
                                        });
                                    })
                                    .done(),
                                _ => spair::set_arm!(mi).done(),
                            }
                        });
                });
        });
    }
}
