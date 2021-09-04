use spair::prelude::*;

impl<P: spair::Component> spair::Component for super::ArticleList<P> {
    type Routes = crate::routes::Route;
    fn init(comp: &spair::Comp<Self>) {
        let cb = comp.callback_once_mut(Self::request_article_list);
        spair::update_component(cb);
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .render(crate::error::ErrorView(self.error.as_ref()))
            .match_if(|mi| match self.article_list.as_ref() {
                None => spair::set_arm!(mi).r#static("Loading articles...").done(),
                Some(article_list) => spair::set_arm!(mi)
                    .list(
                        article_list.articles.iter(),
                        spair::ListElementCreation::Clone,
                    )
                    .render(Pagenation{
                        current_page: self.page_number,
                        article_count: article_list.articles_count,
                    })
                    .done(),
            });
    }
}

impl<P: spair::Component> spair::WithParentComp for super::ArticleList<P> {
    type Parent = P;
    type Properties = super::ArticleFilter;
    fn init(
        _parent: &spair::Comp<Self::Parent>,
        _comp: &spair::Comp<Self>,
        filter: Self::Properties,
    ) -> Self {
        Self::new(filter)
    }
}

impl<P: spair::Component> spair::ListItemRender<super::ArticleList<P>> for &types::ArticleInfo {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(self, element: spair::Element<super::ArticleList<P>>) {
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
                                    .render(&self.author.username);
                            })
                            .span(|s| {
                                s.static_attributes()
                                    .class("date")
                                    .render(&self.created_at.to_string());
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
                        .r#static(" ")
                        .render(self.favorites_count);
                    });
            })
            .a(|a| {
                let route = crate::routes::Route::Article(From::from(self.slug.clone()));
                a.href(&route)
                    .static_attributes()
                    .class("preview-link")
                    .h1(|h| h.render(&self.title).done())
                    .p(|p| p.render(&self.description).done())
                    .static_nodes()
                    .span(|s| s.r#static("Read more...").done());
            })
            .ul(|u| {
                u.static_attributes().class("tag-list").list_with_render(
                    self.tag_list.iter(),
                    spair::ListElementCreation::Clone,
                    "li",
                    |tag, li| {
                        li.class("tag-default")
                            .class("tag-pill")
                            .class("tag-outlinepill")
                            .render(tag);
                    },
                );
            });
    }
}
/*
impl spair::ListItemRender<super::ArticleList> for &types::ArticleInfo {
    fn render(self, element: spair::Element<super::ArticleList>) {
        let comp = nodes.comp();
        element
            .static_attributes()
            .class("article-preview")
            .div(|d| {
                let profile = crate::routes::Route::Profile(self.author.username.clone());
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
                                    .render(&self.author.username);
                            })
                            .span(|s| {
                                s.static_attributes()
                                    .class("date")
                                    .render(&self.created_at.to_string());
                            });
                    })
                    .button(|b| {
                        let article_slug = self.slug.clone();
                        b.on_click(
                            comp.handler_mut(move |state|  toggle_favorite_fn(state, &article_slug)),
                        )
                        .class_or(article.favorited, "btn-primary", "btn-outline-primary")
                        .static_attributes()
                        .class("btn")
                        .class("btn-sm")
                        .class("pull-xs-right")
                        .i(|i| i.static_attributes().class("ion-heart").done())
                        .r#static(" ")
                        .render(self.favorites_count);
                    });
            })
            .a(|a| {
                let route = crate::routes::Route::Article(From::from(self.slug.clone()));
                a.href(&route)
                    .static_attributes()
                    .class("preview-link")
                    .h1(|h| h.render(&self.title).done())
                    .p(|p| p.render(&self.description).done())
                    .static_nodes()
                    .span(|s| s.r#static("Read more...").done());
            })
            .ul(|u| {
                u.static_attributes().class("tag-list").list_with_render(
                    self.tag_list.iter(),
                    spair::ListElementCreation::Clone,
                    "li",
                    |tag, li| {
                        li.static_attributes()
                            .class("tag-default")
                            .class("tag-pill")
                            .class("tag-outlinepill")
                            .render(tag);
                    },
                );
            });
    }
}
*/
struct Pagenation {
    current_page: u32,
    article_count: u32,
}
impl<P: spair::Component> spair::Render<super::ArticleList<P>> for Pagenation {
    fn render(self, nodes: spair::Nodes<super::ArticleList<P>>) {
        let comp = nodes.comp();
        let page_count = self.article_count / crate::ARTICLES_PER_PAGE + 1.min(self.article_count % crate::ARTICLES_PER_PAGE);
        nodes.nav(|n| {n.ul(|u| {
            u.class("pagination")
                .list_with_render(
                    0..page_count,
                    spair::ListElementCreation::Clone,
                    "li",
                    |page_number, l| {
                        l.class("page-item")
                            .class_if(self.current_page == page_number, "active")
                            .on_click(comp.handler_mut(move |state| state.set_page_number(page_number)))
                            .a(|a| {
                                a.class("page-link")
                                    .href_str("")
                                    .render(page_number+1);
                            });
                    }
                );
        });});
    }
}
