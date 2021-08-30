use spair::prelude::*;

impl spair::Component for super::Profile {
    type Routes = crate::routes::Route;
    fn register_routing_callback(
        router: &mut <Self::Routes as spair::Routes>::Router,
        comp: &spair::Comp<Self>,
    ) {
        router.profile_comp = Some(comp.clone());
    }

    fn remove_routing_callback(router: &mut <Self::Routes as spair::Routes>::Router) {
        router.profile_comp = None;
    }

    fn render(&self, element: spair::Element<Self>) {
        element
            .match_if(|mi| match self.profile.as_ref() {
                None => spair::set_arm!(mi).done(),
                Some(profile) => spair::set_arm!(mi).render(profile).done(),
            });
    }
}

impl spair::WithParentComp for super::Profile {
    type Parent = crate::app::App;
    type Properties = String;
    fn init(
        _: &spair::Comp<Self::Parent>,
        _: &spair::Comp<Self>,
        username: Self::Properties,
    ) -> Self {
        Self::new(username)
    }
}

impl spair::Render<super::Profile> for &types::ProfileInfo {
    fn render(self, nodes: spair::Nodes<super::Profile>) {
        nodes.div(|d| {
            d.class("profile-page")
                .div(|d| {
                    d.class("user-info")
                        .div(|d| {
                            d.class("container")
                                .div(|d| {
                                    d.class("row")
                                        .div(|d| {
                                            d.class("col-xs-12")
                                            .class("col-md-10")
                                            .class("offset-md-1")
                                            .render(UserInfo(self));
                                        });
                                });
                        });
                });
        });
    }
}

