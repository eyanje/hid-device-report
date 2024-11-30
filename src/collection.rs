// Collections
// Need to get report items, build reports. and ensure alignment.

use super::report::Report;
use super::usage::Usage;
use super::field_types::{CollectionType, Delimiter, DesignatorIndex, StringIndex};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionItem {
    Report(Report),
    Collection(Collection),
}

impl From<Report> for CollectionItem {
    fn from(report: Report) -> Self {
        Self::Report(report)
    }
}

impl From<Collection> for CollectionItem {
    fn from(collection: Collection) -> Self {
        Self::Collection(collection)
    }
}

/// Helper struct to convert various containers into a collection of items
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionItems(Box<[CollectionItem]>);

impl CollectionItems {
    pub fn as_boxed(self) -> Box<[CollectionItem]> {
        self.0
    }
}

impl From<Box<[CollectionItem]>> for CollectionItems {
    fn from(items: Box<[CollectionItem]>) -> Self {
        Self(items)
    }
}

impl <I: Into<CollectionItem>, const SIZE: usize> From<[I; SIZE]> for CollectionItems {
    fn from(items: [I; SIZE]) -> Self {
        Self(items.into_iter().map(|i| i.into()).collect())
    }
}

macro_rules! impl_from_tuple {
    ($($i:ident: $t:ident),+) => {
        impl <$($t: Into<CollectionItem>),+> From<($($t,)+)> for CollectionItems {
            fn from(($($i,)+): ($($t,)+)) -> Self {
                Self(Box::new([$($i.into()),+]))
            }
        }
    }
}

impl_from_tuple!(a: A);
impl_from_tuple!(a: A, b: B);
impl_from_tuple!(a: A, b: B, c: C);
impl_from_tuple!(a: A, b: B, c: C, d: D);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
impl_from_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);



// 6.2.2.6 Page 33. Collections must have a Usage.
// Collections inherit their Usage attribute like Record with other attributes: from the last Usage
// attribute (at any level, including inside the last collection).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Collection {
    pub collection_type: CollectionType,
    pub usage: Usage,
    pub items: Box<[CollectionItem]>,

    // Optional items: string and physical indices, and delimiters
    pub designator_index: Option<DesignatorIndex>,
    pub string_index: Option<StringIndex>,
    pub delimiter: Option<Delimiter>, // probably not how it's used
}

impl Collection {
    /// Construct an empty collection.
    pub fn empty(t: CollectionType, usage: Usage) -> Self {
        Self {
            collection_type: t,
            usage,
            items: Box::new([]),
            designator_index: None,
            string_index: None,
            delimiter: None,
        }
    }

    /// Construct a collection with a usage and items.
    pub fn new<I: Into<CollectionItems>>(t: CollectionType, usage: Usage, items: I) -> Self {
        Self {
            collection_type: t,
            usage,
            items: items.into().as_boxed(),
            designator_index: None,
            string_index: None,
            delimiter: None,
        }
    }

    pub fn items<'a>(&'a self) -> &'a [CollectionItem] {
        &self.items
    }

}


