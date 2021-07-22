impl spair::Component for crate::pages::HomePage {
    type Routes = ();
    fn render(&self, element: spair::Element<Self>) {
        element
            .class("home-page")
            .render(Banner)
            .render(HomePageContent)
        ;
    }
}

struct Banner;
impl spair::Render<crate::pages::HomePage> for Banner {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes
            .static_nodes()
            .div(|d| {
                d.class("banner")
                .div(|d| {
                    d.class("container")
                    .h1(|h| h.class("logo-font").r#static("conduit").done())
                    .p(|p| p.r#static("A place to share your knowledge.").done());
                });
            });
    }
}

struct HomePageContent;
impl spair::Render<crate::pages::HomePage> for HomePageContent {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes
            .div(|d| {
                d.class("container").class("page")
                    .div(|d| {
                        d.class("row")
                            .div(|d| {
                                d.class("col-md-9")
                                    .render(FeedTabs)
                                ;
                            })
                            .div(|d| {
                                d.class("col-md-3")
                                ;
                            })
                    })
            })
    }
}

struct FeedTabs;
impl spair::Render<crate::pages::HomePage> for FeedTabs {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        let state = nodes.state();
        let comp = nodes.comp();
        nodes
        .div(|d| {
            d.class("feed-toggle")
                .ul(|u| {
                    u.class("nav")
                        .class("nav-pills")
                        .class("outline-active")
                        .li(|i| {
                            i.class("nav-item")
                                .a(|a| a.class("nav-link").class_if("actived", state.is_your_feed()).href_str("").r#static("Your Feed").done())
                        })
                        .li(|i| {
                            i.class("nav-item")
                                .a(|a| a.class("nav-link").class_if("actived", state.is_your_feed()).href_str("").r#static("Global Feed").done())
                        });
                })
        })
    }
}

struct FeedTab<'a, F: spair::Click> {
    title: &'a str,
    active: bool,
    handler: F,
}
impl<'a> spair::Render<crate::pages::HomePage> for FeedTab<'a> {
    fn render(self, nodes: spair::Nodes<crate::pages::HomePage>) {
        nodes.render(self.title);
    }
}


<div class="home-page">

  <div class="container page">
    <div class="row">

      <div class="col-md-9">
        <div class="feed-toggle">
          <ul class="nav nav-pills outline-active">
            <li class="nav-item">
              <a class="nav-link disabled" href="">Your Feed</a>
            </li>
            <li class="nav-item">
              <a class="nav-link active" href="">Global Feed</a>
            </li>
          </ul>
        </div>

        <div class="article-preview">
          <div class="article-meta">
            <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
            <div class="info">
              <a href="" class="author">Eric Simons</a>
              <span class="date">January 20th</span>
            </div>
            <button class="btn btn-outline-primary btn-sm pull-xs-right">
              <i class="ion-heart"></i> 29
            </button>
          </div>
          <a href="" class="preview-link">
            <h1>How to build webapps that scale</h1>
            <p>This is the description for the post.</p>
            <span>Read more...</span>
          </a>
        </div>

        <div class="article-preview">
          <div class="article-meta">
            <a href="profile.html"><img src="http://i.imgur.com/N4VcUeJ.jpg" /></a>
            <div class="info">
              <a href="" class="author">Albert Pai</a>
              <span class="date">January 20th</span>
            </div>
            <button class="btn btn-outline-primary btn-sm pull-xs-right">
              <i class="ion-heart"></i> 32
            </button>
          </div>
          <a href="" class="preview-link">
            <h1>The song you won't ever stop singing. No matter how hard you try.</h1>
            <p>This is the description for the post.</p>
            <span>Read more...</span>
          </a>
        </div>

      </div>

      <div class="col-md-3">
        <div class="sidebar">
          <p>Popular Tags</p>

          <div class="tag-list">
            <a href="" class="tag-pill tag-default">programming</a>
            <a href="" class="tag-pill tag-default">javascript</a>
            <a href="" class="tag-pill tag-default">emberjs</a>
            <a href="" class="tag-pill tag-default">angularjs</a>
            <a href="" class="tag-pill tag-default">react</a>
            <a href="" class="tag-pill tag-default">mean</a>
            <a href="" class="tag-pill tag-default">node</a>
            <a href="" class="tag-pill tag-default">rails</a>
          </div>
        </div>
      </div>

    </div>
  </div>

</div>
*/
