use spair::prelude::*;

impl spair::Component for super::Register {
    type Routes = crate::routes::Route;
    fn render(&self, element: spair::Element<Self>) {
        element.div(|d| {
            d.class("auth-page").div(|d| {
                d.class("container").class("page").div(|d| {
                    d.class("row").div(|d| {
                        d.class("col-md-6")
                            .class("offset-md-3")
                            .class("col-xs-12")
                            .render_fn(|nodes| self.render_register(nodes));
                    });
                });
            });
        });
    }
}

impl super::Register {
    fn render_register(&self, nodes: spair::Nodes<Self>) {
        nodes
            .h1(|h| h.class("text-xs-center").r#static("Sign up").done())
            .p(|p| {
                p.class("text-xs-center").a(|a| {
                    a.href(&crate::routes::Route::Login)
                        .r#static("Have an account?");
                });
            })
            .ul(|u| {
                u.class("error-messages")
                    .li(|li| li.r#static("That email is already taken").done());
            })
            .form(|f| {
                f.fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Text)
                            .placeholder("Your Name");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Text)
                            .placeholder("Email");
                    });
                })
                .fieldset(|f| {
                    f.class("form-group").input(|i| {
                        i.class("form-control")
                            .class("form-control-lg")
                            .r#type(spair::InputType::Password)
                            .placeholder("Password");
                    });
                })
                .button(|b| {
                    b.class("btn")
                        .class("btn-lg")
                        .class("btn-primary")
                        .class("pull-xs-right")
                        .r#static("Sign up");
                });
            });
    }
}

/*
<div class="auth-page">
  <div class="container page">
    <div class="row">

      <div class="col-md-6 offset-md-3 col-xs-12">
        <h1 class="text-xs-center">Sign up</h1>
        <p class="text-xs-center">
          <a href="">Have an account?</a>
        </p>

        <ul class="error-messages">
          <li>That email is already taken</li>
        </ul>

        <form>
          <fieldset class="form-group">
            <input class="form-control form-control-lg" type="text" placeholder="Your Name">
          </fieldset>
          <fieldset class="form-group">
            <input class="form-control form-control-lg" type="text" placeholder="Email">
          </fieldset>
          <fieldset class="form-group">
            <input class="form-control form-control-lg" type="password" placeholder="Password">
          </fieldset>
          <button class="btn btn-lg btn-primary pull-xs-right">
            Sign up
          </button>
        </form>
      </div>

    </div>
  </div>
</div>
*/

impl spair::WithParentComp for super::Register {
    type Parent = crate::app::App;
    type Properties = ();
    fn init(_: &spair::Comp<Self::Parent>, _: &spair::Comp<Self>, _: Self::Properties) -> Self {
        Self::new()
    }
}
