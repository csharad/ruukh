use syn::{
    parenthesized,
    parse::{Error, Parse, ParseStream, Result as ParseResult},
    spanned::Spanned,
    Attribute, FnArg, Ident, ReturnType, Token,
};

/// The syntax for the `#[events]` attribute TokenStream.
///
/// i.e. Parses ```ignore,compile_fail
/// #[events(
///     fn event_name(&self, arg: type, ...) -> type;
///     fn event_name(&self, arg: type, ...) -> type;
///     fn event_name(&self, arg: type, ...) -> type;
/// )]```
pub struct EventsSyntax {
    pub events: Vec<EventSyntax>,
}

impl Parse for EventsSyntax {
    fn parse(input: ParseStream<'_>) -> ParseResult<Self> {
        let content;
        parenthesized!(content in input);

        let mut events = vec![];
        while !content.is_empty() {
            let event: EventSyntax = content.parse()?;
            events.push(event);
        }
        Ok(EventsSyntax { events })
    }
}

/// The syntax of a single event.
///
/// ```ignore,compile_fail
/// #[optional]
/// fn event_name(&self, arg: type, ...) -> type;
/// ```
pub struct EventSyntax {
    pub attr: Option<Attribute>,
    pub ident: Ident,
    pub args: Vec<FnArg>,
    pub return_type: ReturnType,
}

impl Parse for EventSyntax {
    fn parse(input: ParseStream<'_>) -> ParseResult<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        if attrs.len() > 1 {
            Err(Error::new(
                attrs[0].span(),
                "Multiple attributes found. Only one allowed.",
            ))?;
        }

        input.parse::<Token![fn]>()?;
        let ident = input.parse()?;

        let content;
        parenthesized!(content in input);
        let args = content
            .parse_terminated::<_, Token![,]>(FnArg::parse)?
            .into_iter()
            .collect();

        let return_type = input.parse()?;
        // The `#[component]` macro was pointed to when the last one errored.
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        } else {
            Err(input.error("expected `;`"))?;
        }

        Ok(EventSyntax {
            attr: if attrs.is_empty() {
                None
            } else {
                Some(attrs.remove(0))
            },
            ident,
            args,
            return_type,
        })
    }
}
