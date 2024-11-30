use std::slice::Iter;

use super::tag::{Tag, TagGroup};

// Iterating

pub enum TagGroupIterator<'a> {
    Tag(Option<&'a Tag>),
    Group {
        items: Iter<'a, TagGroup>,
        subiterator: Option<Box<TagGroupIterator<'a>>>,
    },
}

impl <'a> TagGroupIterator<'a> {
    pub fn new_over_tag(tag: &'a Tag) -> Self {
        Self::Tag(Some(tag))
    }

    pub fn new_over_group(items: Iter<'a, TagGroup>) -> Self {
        TagGroupIterator::Group {
            items,
            subiterator: None,
        }
    }
}

impl TagGroup {
    pub fn tags(&self) -> TagGroupIterator {
        match self {
            TagGroup::Tag(tag) => TagGroupIterator::new_over_tag(tag),
            TagGroup::Group(items) => TagGroupIterator::new_over_group(items.into_iter()),
        }
    }
}

impl <'a> Iterator for TagGroupIterator<'a> {
    type Item = &'a Tag;

    fn next(&mut self) -> Option<&'a Tag> {
        match self {
            TagGroupIterator::Tag(tag_opt) => {
                tag_opt.take()
            }
            TagGroupIterator::Group { items, subiterator: subiterator_opt } => {
                // Loop because subiterators could be exhausted.
                // Need to loop until a subiterator 
                loop {
                    // Optional subitem from a contained TagGroup. This subitem might be None if
                    // there is no next item in the subiterator, or if the subiterator itself is
                    // None.
                    let next_subitem = subiterator_opt.as_mut()
                        .and_then(|subiterator| subiterator.next());
                    match next_subitem {
                        Some(tag) => {
                            return Some(tag);
                        },
                        None => {
                            match items.next() {
                                Some(next_item) => {
                                    // Replace subiterator with the next TagGroup's iterator.
                                    match subiterator_opt {
                                        Some(subiterator) => {
                                            **subiterator = next_item.tags();
                                        },
                                        None => {
                                            *subiterator_opt = Some(Box::new(next_item.tags()));
                                        },
                                    }
                                    // Continue onto the next iteration.
                                },
                                None => {
                                    // No items left.
                                    return None;
                                }
                            }
                        },
                    }
                }
            },
        }
    }
}



