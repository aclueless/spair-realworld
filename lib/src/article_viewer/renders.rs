use spair::prelude::*;

impl spair::Component for super::ArticleViewer {
    type Routes = crate::routes::Route;

    fn init(comp: &spair::Comp<Self>) {
        comp.callback(Self::get_article).queue();
    }

    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("article-page")
                .match_if(|mi| match self.article.as_ref() {
                    None => spair::set_arm!(mi).done(),
                    Some(article) => spair::set_arm!(mi).rupdate(article).done(),
                });
        });
    }
}

impl spair::AsChildComp for super::ArticleViewer {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = super::Props;

    fn init(comp: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(comp.clone(), props)
    }
}

impl spair::Render<super::ArticleViewer> for &types::ArticleInfo {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        nodes
            .div(|d| {
                d.class("banner").div(|d| {
                    d.class("container")
                        .h1(|h| h.rupdate(&self.title).done())
                        .rupdate(ArticleMeta(self));
                });
            })
            .div(|d| {
                d.class("container")
                    .class("page")
                    .div(|d| {
                        d.class("row").class("article-content").div(|d| {
                            let parser = pulldown_cmark::Parser::new(&self.body);
                            let mut html_text = String::new();
                            pulldown_cmark::html::push_html(&mut html_text, parser);
                            d.class("col-md-12")
                                .div(|d| d.dangerously_set_inner_html(&html_text))
                                .ul(|u| {
                                    u.class("tag-list").lwr_clone(
                                        self.tag_list.iter(),
                                        "li",
                                        |tag, li| {
                                            li.class("tag-default")
                                                .class("tag-pill")
                                                .class("tag-outline")
                                                .rupdate(tag);
                                        },
                                    );
                                });
                        });
                    })
                    .horizontal_line()
                    .div(|d| {
                        d.class("article-actions").rupdate(ArticleMeta(self));
                    })
                    .relement(CommentList);
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
                                .rupdate(&self.0.author.username);
                        })
                        .span(|s| {
                            s.class("date").rupdate(&self.0.created_at.to_string());
                        });
                })
                .rupdate(ArticleActions(self.0));
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
                            .rstatic("Edit Article");
                    })
                    .button(|b| {
                        b.class("btn")
                            .class("btn-sm")
                            .class("btn-outline-danger")
                            .on_click(comp.handler(super::ArticleViewer::delete_article))
                            .i(|i| i.class("ion-trash-a").done())
                            .rstatic("Delete Article");
                    })
                    .done(),
                Some(false) => spair::set_arm!(mi)
                    .button(|b| {
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(
                                self.0.author.following,
                                "btn-secondary",
                                "btn-outline-secondary",
                            )
                            .on_click(comp.handler(super::ArticleViewer::toggle_follow))
                            .i(|i| i.class("ion-plus-round").done())
                            .rstatic(" Follow ")
                            .rupdate(&self.0.author.username);
                    })
                    .rstatic("\u{00A0}\u{00A0}")
                    .button(|b| {
                        b.class("btn")
                            .class("btn-sm")
                            .class_or(self.0.favorited, "btn-primary", "btn-outline-primary")
                            .on_click(comp.handler(super::ArticleViewer::toggle_favorite))
                            .i(|i| i.class("ion-heart").done())
                            .rstatic(" Favorite Post ")
                            .span(|s| {
                                s.class("counter")
                                    .rstatic("(")
                                    .rupdate(self.0.favorites_count)
                                    .rstatic(")");
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
                a.href(&crate::routes::Route::Login).rstatic("Sign in");
            })
            .rstatic(" or ")
            .a(|a| {
                a.href(&crate::routes::Route::Register).rstatic("Sign up");
            })
            .rstatic(" to comment.");
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
                                if let Some(ta) =
                                    event.target_as::<spair::web_sys::HtmlTextAreaElement>()
                                {
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
                                    i.class("comment-author-img")
                                        .src(image)
                                        .alt(&self.0.username);
                                })
                                .done(),
                        })
                        .button(|b| {
                            b.class("btn")
                                .class("btn-sm")
                                .class("btn-primary")
                                .on_click(comp.handler_mut(super::ArticleViewer::post_comment))
                                .disabled(state.new_comment.is_empty())
                                .rstatic("Post comment");
                        });
                });
        });
    }
}

struct CommentList;
impl spair::ElementRender<super::ArticleViewer> for CommentList {
    const ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::ArticleViewer>) {
        let state = element.state();
        element.class("row").div(|d| {
            d.class("col-xs-12")
                .class("col-md-8")
                .class("offset-md-2")
                .match_if(|mi| match state.logged_in_user.as_ref() {
                    None => spair::set_arm!(mi).rupdate(LoginRegister).done(),
                    Some(user) => spair::set_arm!(mi)
                        .rupdate(crate::error::ErrorView(state.error.as_ref()))
                        .rupdate(CommentForm(user)).done(),
                })
                .list_clone(state.comments.iter().flat_map(|v| v.iter()));
        });
    }
}

impl spair::ElementRender<super::ArticleViewer> for &types::CommentInfo {
    const ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::ArticleViewer>) {
        let comp = element.comp();
        element
            .class("card")
            .div(|d| {
                d.class("card-block")
                    .p(|p| p.class("card-text").rupdate(&self.body).done());
            })
            .div(|d| {
                let profile = crate::routes::Route::Profile(self.author.username.clone());
                d.class("card-footer")
                    .a(|a| {
                        a.class("comment-author").href(&profile).img(|i| {
                            i.class("comment-author-img")
                                .src(&self.author.image)
                                .alt(&self.author.username);
                        });
                    })
                    .rstatic(" ")
                    .a(|a| {
                        a.class("comment-author")
                            .href(&profile)
                            .rupdate(&self.author.username);
                    })
                    .span(|s| {
                        s.class("date-posted").rupdate(&self.created_at.to_string());
                    })
                    .match_if(
                        |mi| match mi.state().is_logged_in_username(&self.author.username) {
                            Some(true) => spair::set_arm!(mi)
                                .span(|s| {
                                    s.class("mod-options").i(|i| {
                                        let comment_id = self.id;
                                        i.class("ion-trash-a").on_click(comp.handler_mut(
                                            move |state| state.delete_comment(comment_id),
                                        ));
                                    });
                                })
                                .done(),
                            _ => spair::set_arm!(mi).done(),
                        },
                    );
            });
    }
}
