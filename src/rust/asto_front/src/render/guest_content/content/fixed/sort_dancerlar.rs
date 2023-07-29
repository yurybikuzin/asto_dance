use super::*;

pub fn render() -> impl Signal<Item = Option<Dom>> {
    App::filtered_dancerlar_signal().map(|vec| {
        if vec.len() < 3 {
            None
        } else {
            Some(html!("div", {
                .class("sort")
                .children([
                    html!("label", {
                        .attr("for", "sort_by")
                        .text("Отсортированы по")
                    }),
                    html!("select", {
                        .attr("id", "sort_by")
                        .children(DancerSortBy::iter().map(|i|
                            html!("option", {
                                .attr("value", &(i as u8).to_string())
                                .attr_signal("selected",
                                    APP.data.route.signal_cloned().map(move |route|
                                        if let Route::Guest(GuestRoute{
                                            kind: GuestRouteKind::Dancer{sort_by, ..},
                                            ..
                                        }) = route.unwrap_or_else(route_default) {
                                            (i == sort_by).then_some("")
                                        } else {
                                            None
                                        }
                                    )
                                )
                                .text(&i.to_string())
                            })
                        ))
                        .with_node!(element => {
                            .event(move |_event: events::Change| {
                                if let Some(element) = element.dyn_ref::<HtmlSelectElement>() {
                                    let value = element.value().parse::<u8>().unwrap();
                                    if let Some(sort_by) = DancerSortBy::from_repr(value) {
                                        let url = route_to_url(&Route::Guest(GuestRoute {
                                            kind: GuestRouteKind::Dancer{sort_by, expanded: HashSet::new()},
                                            did_press: *APP.data.did_press.lock_ref(),
                                            search: APP.data.public_search.lock_ref().clone(),
                                        }));
                                        cancel_delayed_go_to_url();
                                        go_to_url(&url);
                                    }
                                }
                            })
                        })
                    }),
                ])
            }))
        }
    })
}