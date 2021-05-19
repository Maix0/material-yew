use std::borrow::Cow;
use std::fmt;
use yew::prelude::*;

/// Dialog action type.
#[derive(Clone)]
pub enum ActionType {
    /// Binds `to slot` of `primaryAction`
    Primary,
    /// Binds `to slot` of `secondaryAction`
    Secondary,
}

impl ActionType {
    fn to_cow_string(&self) -> Cow<'static, str> {
        let s = match self {
            ActionType::Primary => "primaryAction",
            ActionType::Secondary => "secondaryAction",
        };
        Cow::from(s)
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_cow_string())
    }
}

/// Props for [`MatDialogAction`]
#[derive(Properties, Clone)]
pub struct ActionProps {
    pub action_type: ActionType,
    #[prop_or_default]
    pub action: Option<Cow<'static, str>>,
    pub children: Children,
}

/// Defines actions for [`MatDialog`][crate::MatDialog].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatDialogAction {
    props: ActionProps,
}

impl Component for MatDialogAction {
    type Message = ();
    type Properties = ActionProps;

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
        let children = self.props.children.iter().map(|child| {
            match child {
                Html::VTag(mut vtag) => {
                    vtag.add_attribute("slot", self.props.action_type.to_string());
                    if let Some(action) = self.props.action.as_ref() {
                        vtag.add_attribute("dialogAction", action.to_owned());
                    }
                    Html::VTag(vtag)
                }
                _ => html! {
                    <span slot=self.props.action_type.to_string() dialogAction=self.props.action.clone()>
                        { child }
                    </span>
                }
            }
        }).collect::<Html>();

        html! {
            { children }
        }
    }
}
