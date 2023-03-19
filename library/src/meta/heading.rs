use typst::font::FontWeight;

use super::{Counter, CounterUpdate, LocalName, Numbering};
use crate::layout::{BlockNode, HNode, VNode};
use crate::meta::Count;
use crate::prelude::*;
use crate::text::{TextNode, TextSize};

/// A section heading.
///
/// With headings, you can structure your document into sections. Each heading
/// has a _level,_ which starts at one and is unbounded upwards. This level
/// indicates the logical role of the following content (section, subsection,
/// etc.)  A top-level heading indicates a top-level section of the document
/// (not the document's title).
///
/// Typst can automatically number your headings for you. To enable numbering,
/// specify how you want your headings to be numbered with a
/// [numbering pattern or function]($func/numbering).
///
/// Independently from the numbering, Typst can also automatically generate an
/// [outline]($func/outline) of all headings for you. To exclude one or more
/// headings from this outline, you can set the `outlined` parameter to
/// `{false}`.
///
/// ## Example
/// ```example
/// #set heading(numbering: "1.a)")
///
/// = Introduction
/// In recent years, ...
///
/// == Preliminaries
/// To start, ...
/// ```
///
/// ## Syntax
/// Headings have dedicated syntax: They can be created by starting a line with
/// one or multiple equals signs, followed by a space. The number of equals
/// signs determines the heading's logical nesting depth.
///
/// Display: Heading
/// Category: meta
#[node(Locatable, Synthesize, Count, Show, Finalize, LocalName)]
pub struct HeadingNode {
    /// The logical nesting depth of the heading, starting from one.
    #[default(NonZeroUsize::ONE)]
    pub level: NonZeroUsize,

    /// How to number the heading. Accepts a
    /// [numbering pattern or function]($func/numbering).
    ///
    /// ```example
    /// #set heading(numbering: "1.a.")
    ///
    /// = A section
    /// == A subsection
    /// === A sub-subsection
    /// ```
    pub numbering: Option<Numbering>,

    /// Whether the heading should appear in the outline.
    ///
    /// ```example
    /// #outline()
    ///
    /// #heading[Normal]
    /// This is a normal heading.
    ///
    /// #heading(outlined: false)[Hidden]
    /// This heading does not appear
    /// in the outline.
    /// ```
    #[default(true)]
    pub outlined: bool,

    /// The heading's title.
    #[required]
    pub body: Content,
}

impl Synthesize for HeadingNode {
    fn synthesize(&mut self, _: &Vt, styles: StyleChain) {
        self.push_level(self.level(styles));
        self.push_numbering(self.numbering(styles));
        self.push_outlined(self.outlined(styles));
    }
}

impl Show for HeadingNode {
    fn show(&self, _: &mut Vt, styles: StyleChain) -> SourceResult<Content> {
        let mut realized = self.body();
        if let Some(numbering) = self.numbering(styles) {
            realized =
                Counter::of(Self::id()).display(numbering, false).spanned(self.span())
                    + HNode::new(Em::new(0.3).into()).with_weak(true).pack()
                    + realized;
        }
        Ok(BlockNode::new().with_body(Some(realized)).pack())
    }
}

impl Finalize for HeadingNode {
    fn finalize(&self, realized: Content, styles: StyleChain) -> Content {
        let level = self.level(styles).get();
        let scale = match level {
            1 => 1.4,
            2 => 1.2,
            _ => 1.0,
        };

        let size = Em::new(scale);
        let above = Em::new(if level == 1 { 1.8 } else { 1.44 }) / scale;
        let below = Em::new(0.75) / scale;

        let mut map = StyleMap::new();
        map.set(TextNode::set_size(TextSize(size.into())));
        map.set(TextNode::set_weight(FontWeight::BOLD));
        map.set(BlockNode::set_above(VNode::block_around(above.into())));
        map.set(BlockNode::set_below(VNode::block_around(below.into())));
        map.set(BlockNode::set_sticky(true));
        realized.styled_with_map(map)
    }
}

impl Count for HeadingNode {
    fn update(&self) -> Option<CounterUpdate> {
        self.numbering(StyleChain::default())
            .is_some()
            .then(|| CounterUpdate::Step(self.level(StyleChain::default())))
    }
}

cast_from_value! {
    HeadingNode,
    v: Content => v.to::<Self>().ok_or("expected heading")?.clone(),
}

impl LocalName for HeadingNode {
    fn local_name(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::GERMAN => "Abschnitt",
            Lang::ENGLISH | _ => "Section",
        }
    }
}
