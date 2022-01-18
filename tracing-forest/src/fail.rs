/* layer */
#[cold]
pub fn span_not_in_context<T>() -> T {
    panic!("Span not in context, this is a bug");
}

#[cold]
pub fn tree_span_opened_not_in_extensions<T>() -> T {
    panic!("Span extension doesn't contain `TreeSpanOpened`, this is a bug");
}

#[cold]
pub fn multiple_tags_on_event() -> ! {
    panic!("More than one tag was passed to an event, this is likely a mistake");
}

/* id */
#[cold]
pub fn subscriber_not_found<'a, S>() -> &'a S {
    panic!(
        "Subscriber could not be downcasted to `{}`",
        std::any::type_name::<S>()
    );
}

#[cold]
pub fn no_current_span<T>() -> T {
    panic!("The subscriber isn't in any spans");
}

#[cold]
pub fn no_tree_layer<T>() -> T {
    panic!("The span has no `TreeSpan` in extensions, perhaps you forgot to add a `TreeLayer` to your subscriber?");
}

/* tag */
#[cold]
pub fn tag_unset(id: u64) -> crate::tag::TagData {
    panic!("No tag type set, but a tag was received: {}. If this is intentional, ensure that none of your field names are `{}` to avoid this.", id, crate::layer::TAG_KEY);
}
