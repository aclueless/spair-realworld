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
                    Some(article) => spair::set_arm!(mi)
                        .rfn(|nodes| render_article(article, nodes))
                        .done(),
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

fn render_article(ai: &types::ArticleInfo, nodes: spair::Nodes<super::ArticleViewer>) {
    nodes
        .div(|d| {
            d.class("banner").div(|d| {
                d.class("container")
                    .h1(|h| h.update_text(&ai.title).done())
                    .rfn(|nodes| render_article_meta(ai, nodes));
            });
        })
        .div(|d| {
            d.class("container")
                .class("page")
                .div(|d| {
                    d.class("row").class("article-content").div(|d| {
                        let parser = pulldown_cmark::Parser::new(&ai.body);
                        let mut html_text = String::new();
                        pulldown_cmark::html::push_html(&mut html_text, parser);
                        d.class("col-md-12")
                            .div(|d| d.dangerously_set_inner_html(&html_text))
                            .ul(|u| {
                                u.class("tag-list").list_clone(
                                    ai.tag_list.iter(),
                                    "li",
                                    |tag, li| {
                                        li.class("tag-default")
                                            .class("tag-pill")
                                            .class("tag-outline")
                                            .update_text(tag);
                                    },
                                );
                            });
                    });
                })
                .horizontal_line()
                .div(|d| {
                    d.class("article-actions")
                        .rfn(|nodes| render_article_meta(ai, nodes));
                })
                .div(render_comment_list);
        });
}

fn render_article_meta(ai: &types::ArticleInfo, nodes: spair::Nodes<super::ArticleViewer>) {
    let profile = crate::routes::Route::Profile(ai.author.username.clone());
    nodes.div(|d| {
        d.class("article-meta")
            .a(|a| {
                a.href(&profile).img(|i| {
                    i.src(&ai.author.image).alt(&ai.author.username);
                });
            })
            .div(|d| {
                d.class("info")
                    .a(|a| {
                        a.href(&profile)
                            .class("author")
                            .update_text(&ai.author.username);
                    })
                    .span(|s| {
                        s.class("date").update_text(ai.created_at.to_string());
                    });
            })
            .rfn(|nodes| render_article_actions(ai, nodes));
    });
}

fn render_article_actions(ai: &types::ArticleInfo, nodes: spair::Nodes<super::ArticleViewer>) {
    let state = nodes.state();
    let comp = nodes.comp();
    nodes.match_if(
        |mi| match state.is_logged_in_username(&ai.author.username) {
            Some(true) => spair::set_arm!(mi)
                .a(|a| {
                    a.class("btn")
                        .class("btn-sm")
                        .class("btn-outline-secondary")
                        .href(&crate::routes::Route::Editor(Some(state.slug.clone())))
                        .i(|i| i.class("ion-edit").done())
                        .static_text("Edit Article");
                })
                .button(|b| {
                    b.class("btn")
                        .class("btn-sm")
                        .class("btn-outline-danger")
                        .on_click(comp.handler(super::ArticleViewer::delete_article))
                        .i(|i| i.class("ion-trash-a").done())
                        .static_text("Delete Article");
                })
                .done(),
            Some(false) => spair::set_arm!(mi)
                .button(|b| {
                    b.class("btn")
                        .class("btn-sm")
                        .class_or(
                            ai.author.following,
                            "btn-secondary",
                            "btn-outline-secondary",
                        )
                        .on_click(comp.handler(super::ArticleViewer::toggle_follow))
                        .i(|i| i.class("ion-plus-round").done())
                        .static_text(" Follow ")
                        .update_text(&ai.author.username);
                })
                .static_text("\u{00A0}\u{00A0}")
                .button(|b| {
                    b.class("btn")
                        .class("btn-sm")
                        .class_or(ai.favorited, "btn-primary", "btn-outline-primary")
                        .on_click(comp.handler(super::ArticleViewer::toggle_favorite))
                        .i(|i| i.class("ion-heart").done())
                        .static_text(" Favorite Post ")
                        .span(|s| {
                            s.class("counter")
                                .static_text("(")
                                .update_text(ai.favorites_count)
                                .static_text(")");
                        });
                })
                .done(),
            None => spair::set_arm!(mi).done(),
        },
    );
}

fn render_sign_in_up(nodes: spair::Nodes<super::ArticleViewer>) {
    nodes
        .static_nodes()
        .a(|a| {
            a.href(&crate::routes::Route::Login).static_text("Sign in");
        })
        .static_text(" or ")
        .a(|a| {
            a.href(&crate::routes::Route::Register)
                .static_text("Sign up");
        })
        .static_text(" to comment.");
}

fn render_comment_form(ui: &types::UserInfo, nodes: spair::Nodes<super::ArticleViewer>) {
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
                    .match_if(|mi| match ui.image.as_ref() {
                        None => spair::set_arm!(mi).done(),
                        Some(image) => spair::set_arm!(mi)
                            .img(|i| {
                                i.class("comment-author-img").src(image).alt(&ui.username);
                            })
                            .done(),
                    })
                    .button(|b| {
                        b.class("btn")
                            .class("btn-sm")
                            .class("btn-primary")
                            .on_click(comp.handler_mut(super::ArticleViewer::post_comment))
                            .disabled(state.new_comment.is_empty())
                            .static_text("Post comment");
                    });
            });
    });
}

fn render_comment_list(element: spair::Element<super::ArticleViewer>) {
    let state = element.state();
    element.class("row").div(|d| {
        d.class("col-xs-12")
            .class("col-md-8")
            .class("offset-md-2")
            .match_if(|mi| match state.logged_in_user.as_ref() {
                None => spair::set_arm!(mi).rfn(render_sign_in_up).done(),
                Some(user) => spair::set_arm!(mi)
                    .rfn(|nodes| crate::error::render_error(state.error.as_ref(), nodes))
                    .rfn(|nodes| render_comment_form(user, nodes))
                    .done(),
            })
            .list_clone(
                state.comments.iter().flat_map(|v| v.iter()),
                "div",
                render_comment,
            );
    });
}

fn render_comment(ci: &types::CommentInfo, element: spair::Element<super::ArticleViewer>) {
    let comp = element.comp();
    element
        .class("card")
        .div(|d| {
            d.class("card-block")
                .p(|p| p.class("card-text").update_text(&ci.body).done());
        })
        .div(|d| {
            let profile = crate::routes::Route::Profile(ci.author.username.clone());
            d.class("card-footer")
                .a(|a| {
                    a.class("comment-author").href(&profile).img(|i| {
                        i.class("comment-author-img")
                            .src(&ci.author.image)
                            .alt(&ci.author.username);
                    });
                })
                .static_text(" ")
                .a(|a| {
                    a.class("comment-author")
                        .href(&profile)
                        .update_text(&ci.author.username);
                })
                .span(|s| {
                    s.class("date-posted")
                        .update_text(ci.created_at.to_string());
                })
                .match_if(
                    |mi| match mi.state().is_logged_in_username(&ci.author.username) {
                        Some(true) => spair::set_arm!(mi)
                            .span(|s| {
                                s.class("mod-options").i(|i| {
                                    let comment_id = ci.id;
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
