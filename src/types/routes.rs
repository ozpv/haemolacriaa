pub struct RouteInfo<'a> {
    pub name: &'a str,
    pub path: &'a str,
    /// Set this to true for routes that are statically generated
    pub external: bool,
}
