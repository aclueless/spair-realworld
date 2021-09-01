use spair::prelude::*;

pub struct ArticlePreview<'a, C> {
    pub article: &'a types::ArticleInfo,
    pub toggle_favorite_fn: fn(&mut C, slug: &types::Slug) -> spair::Command<C>,
}

impl<'a, C> ArticlePreview<'a, C>
where C: spair::Component<Routes = crate::routes::Route>
{
    pub fn render(self, nodes: spair::Nodes<C>) {
        let comp = nodes.comp();
        let ArticlePreview { article, toggle_favorite_fn } = self;
        nodes
            .div(|d| {
                let profile = crate::routes::Route::Profile(article.author.username.clone());
                d.static_attributes()
                    .class("article-meta")
                    .a(|a| {
                        a.href(&profile).img(|i| i.src(&article.author.image).done());
                    })
                    .div(|d| {
                        d.static_attributes()
                            .class("info")
                            .a(|a| {
                                a.href(&profile)
                                    .static_attributes()
                                    .class("author")
                                    .render(&article.author.username);
                            })
                            .span(|s| {
                                s.static_attributes()
                                    .class("date")
                                    .render(&article.created_at.to_string());
                            });
                    })
                    .button(|b| {
                        let article_slug = article.slug.clone();
                        b.on_click(
                            comp.handler_mut(move |state|  toggle_favorite_fn(state, &article_slug)),
                        )
                        .static_attributes()
                        .class("btn")
                        .class_or(article.favorited, "btn-primary", "btn-outline-primary")
                        .class("btn-sm")
                        .class("pull-xs-right")
                        .i(|i| i.static_attributes().class("ion-heart").done())
                        .r#static(" ")
                        .render(article.favorites_count);
                    });
            })
            .a(|a| {
                let route = crate::routes::Route::Article(From::from(article.slug.clone()));
                a.href(&route)
                    .static_attributes()
                    .class("preview-link")
                    .h1(|h| h.render(&article.title).done())
                    .p(|p| p.render(&article.description).done())
                    .static_nodes()
                    .span(|s| s.r#static("Read more...").done());
            })
            .ul(|u| {
                u.static_attributes().class("tag-list").list_with_render(
                    article.tag_list.iter(),
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
