use spair::prelude::*;

impl spair::Component for super::ArticleViewer {
    type Routes = crate::routes::Route;

    fn render(&self, element: spair::Element<Self>) {
        element
            .div(|d| {
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

    fn init(_parent: &spair::Comp<Self::Parent>, _comp: &spair::Comp<Self>, props: Self::Properties) -> Self {
        Self::new(props)
    }
}

impl spair::Render<super::ArticleViewer> for &types::ArticleInfo {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        nodes
            .div(|d| {
                d.class("banner")
                    .div(|d| {
                        d.class("container")
                            .h1(|h| h.render(&self.title).done())
                            .render(ArticleMeta(self));
                    })
            })
            .div(|d| {
                d.class("container").class("page")
                    .div(|d| {
                        d.class("row")
                            .class("article-content")
                            .div(|d| {
                                d.class("col-md-12")
                                .p(|p| p.render(&self.body))
                            })
                    })
                    .horizontal_line()
                    .div(|d| {
                        d.class("article-actions")
                    })
                    .div(|d| {
                        d.class("row")
                    })
            })
    }
}

struct ArticleMeta<'a>(&'a types::ArticleInfo);
impl<'a> spair::Render<super::ArticleViewer> for ArticleMeta<'a> {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let profile = crate::routes::Route::Profile(self.0.author.username.clone());
        nodes.div(|d| {
            d.class("article-meta")
                .a(|a| {
                    a.href(&profile)
                        .img(|i| {
                            i.src(&self.0.author.image)
                                .alt(&self.0.author.username);
                        });
                }).div(|d| {
                    d.class("info")
                        .a(|a| {
                            a.href(&profile)
                                .class("author")
                                .render(&self.0.author.username);
                        }).span(|s| {
                            s.class("date")
                                .render(&self.0.create_at);
                        });
                }).render(ArticleActions(self.0);
        })
    }
}

struct ArticleActions<'a>(&'a types::ArticleInfo);
impl<'a> spair::Render<super::ArticleViewer> for ArticleActions<'a> {
    fn render(self, nodes: spair::Nodes<super::ArticleViewer>) {
        let comp = nodes.comp();
        nodes
            .match_if(|mi| match mi.state().user_own_article(&self.0.author.username) {
                Some(true) => spair::set_arm!(mi).done(),
                Some(false) => match mi.state().user spair::set_arm!(mi).button(|b| {
                    let username = self.0.author.username.clone();
                    b
                        .class("btn")
                        .class("btn-sm")
                        .class_or(self.0.author.followed, "btn-secondary", "btn-outline-secondary")
                        .on_click(comp.handler(move |state| state.toggle_follow(&username)))
                        .i(|i| i.class("ion-plus-round").done())
                        .r#static(" Follow")
                        .render(&self.0.author.username);
                })
                .r#static("\u{00A0}\u{00A0}")
                .button(|b| b{
                    let username = self.0.author.username.clone();
                    b
                        .class("btn")
                        .class("btn-sm")
                        .class_or(self.0.author.followed, "btn-primary", "btn-outline-primary")
                        .on_click(comp.handler(move |state| state.toggle_favorite(&username)))
                        .i(|i| i.class("ion-heart").done())
                        .r#static(" Favorite Post ")
                        .span(|s| {
                            s.class("counter")
                            .r#static("(")
                            .render(self.0.favorite_count)
                            .r#static(")");
                        })
                        ;
                })
                .done(),
                None => spair::set_arm!(mi).done(),
            });
    }
}

/*
<div class="article-page">

  <div class="banner">
    <div class="container">

      <h1>How to build webapps that scale</h1>

      <div class="article-meta">
        <a href=""><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
        <div class="info">
          <a href="" class="author">Eric Simons</a>
          <span class="date">January 20th</span>
        </div>
        <button class="btn btn-sm btn-outline-secondary">
          <i class="ion-plus-round"></i>
          &nbsp;
          Follow Eric Simons <span class="counter">(10)</span>
        </button>
        &nbsp;&nbsp;
        <button class="btn btn-sm btn-outline-primary">
          <i class="ion-heart"></i>
          &nbsp;
          Favorite Post <span class="counter">(29)</span>
        </button>
      </div>

    </div>
  </div>

  <div class="container page">

    <div class="row article-content">
      <div class="col-md-12">
        <p>
        Web development technologies have evolved at an incredible clip over the past few years.
        </p>
        <h2 id="introducing-ionic">Introducing RealWorld.</h2>
        <p>It's a great solution for learning how other frameworks work.</p>
      </div>
    </div>

    <hr />

    <div class="article-actions">
      <div class="article-meta">
        <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
        <div class="info">
          <a href="" class="author">Eric Simons</a>
          <span class="date">January 20th</span>
        </div>

        <button class="btn btn-sm btn-outline-secondary">
          <i class="ion-plus-round"></i>
          &nbsp;
          Follow Eric Simons <span class="counter">(10)</span>
        </button>
        &nbsp;
        <button class="btn btn-sm btn-outline-primary">
          <i class="ion-heart"></i>
          &nbsp;
          Favorite Post <span class="counter">(29)</span>
        </button>
      </div>
    </div>

    <div class="row">

      <div class="col-xs-12 col-md-8 offset-md-2">

        <form class="card comment-form">
          <div class="card-block">
            <textarea class="form-control" placeholder="Write a comment..." rows="3"></textarea>
          </div>
          <div class="card-footer">
            <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
            <button class="btn btn-sm btn-primary">
             Post Comment
            </button>
          </div>
        </form>

        <div class="card">
          <div class="card-block">
            <p class="card-text">With supporting text below as a natural lead-in to additional content.</p>
          </div>
          <div class="card-footer">
            <a href="" class="comment-author">
              <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
            </a>
            &nbsp;
            <a href="" class="comment-author">Jacob Schmidt</a>
            <span class="date-posted">Dec 29th</span>
          </div>
        </div>

        <div class="card">
          <div class="card-block">
            <p class="card-text">With supporting text below as a natural lead-in to additional content.</p>
          </div>
          <div class="card-footer">
            <a href="" class="comment-author">
              <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
            </a>
            &nbsp;
            <a href="" class="comment-author">Jacob Schmidt</a>
            <span class="date-posted">Dec 29th</span>
            <span class="mod-options">
              <i class="ion-edit"></i>
              <i class="ion-trash-a"></i>
            </span>
          </div>
        </div>

      </div>

    </div>

  </div>

</div>
*/
