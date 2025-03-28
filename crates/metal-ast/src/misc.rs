// SPDX-License-Identifier: MIT

use metal_lexer::{spanned, MaybeSpanned, Span, Spanned};

use crate::Item;

/// An identifier, such as `i_am_a_variable`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct Ident<'src> {
    pub inner: &'src str,
}

/// A block of code.
#[spanned]
#[derive(Debug, Spanned)]
pub struct Block<'src> {
    /// See [Item].
    pub items: Vec<Item<'src>>,
}

/// An item's visibility.
#[derive(Debug)]
pub enum Visibility {
    /// Public visibility.
    Pub(Span),
    /// Local visibility, default.
    Local,
}

/// A variable's mutability.
#[derive(Debug)]
pub enum Mutability {
    /// Mutable.
    Mut(Span),
    /// Immutable.
    Immut,
}

impl MaybeSpanned for Visibility {
    fn maybe_span(&self) -> Option<&Span> {
        match self {
            Visibility::Pub(span) => Some(span),
            Visibility::Local => None,
        }
    }
}

impl MaybeSpanned for Mutability {
    fn maybe_span(&self) -> Option<&Span> {
        match self {
            Mutability::Mut(span) => Some(span),
            Mutability::Immut => None,
        }
    }
}
