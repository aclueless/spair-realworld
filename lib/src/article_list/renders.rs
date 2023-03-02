use spair::prelude::*;

impl spair::Component for super::ArticleList {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback_once_mut(Self::request_article_list).queue();
    }

    fn render(&self, element: spair::Element<Self>) {
        element.match_if(|mi| match self.article_list.as_ref() {
            None => spair::set_arm!(mi)
                .static_text("Loading articles...")
                .done(),
            Some(article_list) if article_list.articles.is_empty() => {
                spair::set_arm!(mi).static_text("No articles found").done()
            }
            Some(article_list) => spair::set_arm!(mi)
                .list_clone(article_list.articles.iter(), "div", render_article)
                .rfn(|nodes| {
                    render_pagination(self.current_page, article_list.articles_count, nodes)
                })
                .done(),
        });
    }
}

impl spair::AsChildComp for super::ArticleList {
    const ROOT_ELEMENT_TAG: spair::TagName = spair::TagName::Html(spair::HtmlTag("div"));
    type Properties = super::ArticleFilter;
    fn init(comp: &spair::Comp<Self>, filter: Self::Properties) -> Self {
        Self::new(comp.clone(), filter)
    }
}

fn render_article(ai: &types::ArticleInfo, element: spair::Element<super::ArticleList>) {
    let comp = element.comp();
    let profile = crate::routes::Route::Profile(ai.author.username.clone());
    let article_slug = ai.slug.clone();
    let favorited = ai.favorited;
    element
        .static_attributes()
        .class("article-preview")
        .div(|d| {
            d.static_attributes()
                .class("article-meta")
                .a(|a| {
                    a.href(&profile).img(|i| i.src(&ai.author.image).done());
                })
                .div(|d| {
                    d.static_attributes()
                        .class("info")
                        .a(|a| {
                            a.href(&profile)
                                .static_attributes()
                                .class("author")
                                .update_text(&ai.author.username);
                        })
                        .span(|s| {
                            s.static_attributes()
                                .class("date")
                                .update_text(ai.created_at.to_string());
                        });
                })
                .button(|b| {
                    b.on_click(
                        comp.handler_mut(move |state| {
                            state.toggle_favorite(favorited, &article_slug)
                        }),
                    )
                    .static_attributes()
                    .class("btn")
                    .class_or(ai.favorited, "btn-primary", "btn-outline-primary")
                    .class("btn-sm")
                    .class("pull-xs-right")
                    .i(|i| i.static_attributes().class("ion-heart").done())
                    .update_text(ai.favorites_count);
                });
        })
        .a(|a| {
            let route = crate::routes::Route::Article(ai.slug.clone());
            a.href(&route)
                .static_attributes()
                .class("preview-link")
                .h1(|h| h.update_text(&ai.title).done())
                .p(|p| p.update_text(&ai.description).done())
                .static_nodes()
                .span(|s| s.static_text("Read more...").done());
        })
        .ul(|u| {
            u.static_attributes().class("tag-list").list_clone(
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
}

fn render_pagination(
    current_page: u32,
    article_count: u32,
    nodes: spair::Nodes<super::ArticleList>,
) {
    let comp = nodes.comp();
    let page_count =
        article_count / crate::ARTICLES_PER_PAGE + 1.min(article_count % crate::ARTICLES_PER_PAGE);
    nodes.match_if(|mi| {
        if article_count < crate::ARTICLES_PER_PAGE {
            spair::set_arm!(mi);
        } else {
            spair::set_arm!(mi).nav(|n| {
                n.ul(|u| {
                    u.class("pagination")
                        .list_clone(0..page_count, "li", |page_number, l| {
                            l.class("page-item")
                                .class_if(current_page == page_number, "active")
                                .on_click(comp.handler_arg_mut(
                                    move |state, arg: spair::MouseEvent| {
                                        arg.raw().prevent_default();
                                        state.set_current_page(page_number)
                                    },
                                ))
                                .a(|a| {
                                    a.class("page-link")
                                        .href_str("")
                                        .update_text(page_number + 1);
                                });
                        });
                });
            });
        }
    });
}
