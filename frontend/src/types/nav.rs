use crate::Route;

pub struct NavItem<'a> {
    pub route: Route,
    pub text: &'a str,
}
