use yew::prelude::*;

const SLOT: &str = "icon";

/// Props for [`MatTabIcon`]
#[derive(Properties, Clone)]
pub struct TabIconProps {
    pub children: Children,
}

/// Defines title for [`MatTab`][crate::MatTab].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatTabIcon {
    props: TabIconProps,
}

impl Component for MatTabIcon {
    type Message = ();
    type Properties = TabIconProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let children = self
            .props
            .children
            .iter()
            .map(|child| {
                match child {
                    Html::VTag(mut vtag) => {
                        vtag.add_attribute("slot", "title");
                        Html::VTag(vtag)
                    }
                    _ => {
                        html! {
                            <span slot=SLOT>
                                { child }
                            </span>
                        }
                    }
                }
            })
            .collect::<Html>();

        html! {
            { children }
        }
    }
}
