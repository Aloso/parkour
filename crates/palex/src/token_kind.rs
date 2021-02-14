/// The kind of the current token.
///
/// This enum acts as the state of the currently parsed argument; this is
/// necessary because an argument can consist of multiple tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// An argument that doesn't start with dashes, e.g. `program`.
    NoDash,

    /// An argument that starts with exactly 1 dash, e.g. `-foo`, `-V`,
    /// `-h=config`.
    OneDash,

    /// An argument that starts with 2 or more dashes, e.g. `--version` or
    /// `--help=config`.
    TwoDashes,

    /// An option or value of a single-dash argument, after an option has been
    /// eaten.
    ///
    /// ### Example when parsing `-abcd=efg,hij`
    ///
    /// ```text
    /// abcd=efg    # OneDash
    ///  bcd=efg    # AfterOneDash
    ///   cd=efg    # AfterOneDash
    ///      efg    # AfterEquals
    /// ```
    AfterOneDash,

    /// A value of an argument after the `=`, after the name of the argument has
    /// been eaten.
    ///
    /// ### Example when parsing `--abcd=efg,hij`
    ///
    /// ```text
    /// abcd=efg,hij    # TwoDashes
    ///      efg,hij    # AfterEquals
    ///          hij    # AfterEquals
    /// ```
    AfterEquals,
}
