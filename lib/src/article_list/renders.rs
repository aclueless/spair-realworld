use spair::prelude::*;

impl spair::Component for super::ArticleList {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        comp.callback_once_mut(Self::request_article_list).queue();
    }

    fn render(&self, element: spair::Element<Self>) {
        element.match_if(|mi| match self.article_list.as_ref() {
            None => spair::set_arm!(mi).rstatic("Loading articles...").done(),
            Some(article_list) if article_list.articles.is_empty() => spair::set_arm!(mi)
                .rstatic("No articles found").done(),
            Some(article_list) => spair::set_arm!(mi)
                .list_clone(article_list.articles.iter())
                .rupdate(Pagination {
                    current_page: self.current_page,
                    article_count: article_list.articles_count,
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

impl spair::ElementRender<super::ArticleList> for &types::ArticleInfo {
    const ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::ArticleList>) {
        let comp = element.comp();
        let profile = crate::routes::Route::Profile(self.author.username.clone());
        let article_slug = self.slug.clone();
        let favorited = self.favorited;
        element
            .static_attributes()
            .class("article-preview")
            .div(|d| {
                d.static_attributes()
                    .class("article-meta")
                    .a(|a| {
                        a.href(&profile).img(|i| i.src(&self.author.image).done());
                    })
                    .div(|d| {
                        d.static_attributes()
                            .class("info")
                            .a(|a| {
                                a.href(&profile)
                                    .static_attributes()
                                    .class("author")
                                    .rupdate(&self.author.username);
                            })
                            .span(|s| {
                                s.static_attributes()
                                    .class("date")
                                    .rupdate(&self.created_at.to_string());
                            });
                    })
                    .button(|b| {
                        b.on_click(comp.handler_mut(move |state| {
                            state.toggle_favorite(favorited, &article_slug)
                        }))
                        .static_attributes()
                        .class("btn")
                        .class_or(self.favorited, "btn-primary", "btn-outline-primary")
                        .class("btn-sm")
                        .class("pull-xs-right")
                        .i(|i| i.static_attributes().class("ion-heart").done())
                        .rupdate(self.favorites_count);
                    });
            })
            .a(|a| {
                let route = crate::routes::Route::Article(self.slug.clone());
                a.href(&route)
                    .static_attributes()
                    .class("preview-link")
                    .h1(|h| h.rupdate(&self.title).done())
                    .p(|p| p.rupdate(&self.description).done())
                    .static_nodes()
                    .span(|s| s.rstatic("Read more...").done());
            })
            .ul(|u| {
                u.static_attributes().class("tag-list").lwr_clone(
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
    }
}

struct Pagination {
    current_page: u32,
    article_count: u32,
}

impl spair::Render<super::ArticleList> for Pagination {
    fn render(self, nodes: spair::Nodes<super::ArticleList>) {
        let comp = nodes.comp();
        let page_count = self.article_count / crate::ARTICLES_PER_PAGE
            + 1.min(self.article_count % crate::ARTICLES_PER_PAGE);
        nodes.match_if(|mi| {
            if self.article_count < crate::ARTICLES_PER_PAGE {
                spair::set_arm!(mi);
            } else {
                spair::set_arm!(mi).nav(|n| {
                    n.ul(|u| {
                        u.class("pagination")
                            .lwr_clone(0..page_count, "li", |current_page, l| {
                                l.class("page-item")
                                    .class_if(self.current_page == current_page, "active")
                                    .on_click(comp.handler_arg_mut(
                                        move |state, arg: spair::MouseEvent| {
                                            arg.raw().prevent_default();
                                            state.set_current_page(current_page)
                                        },
                                    ))
                                    .a(|a| {
                                        a.class("page-link").href_str("").rupdate(current_page + 1);
                                    });
                            });
                    });
                });
            }
        });
    }
}
