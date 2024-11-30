use super::collection::Collection;
use super::optimizer::TagOptimizer;
use super::item::{ShortItems};
use super::tag::{Tag, TagGroup};

impl Collection {
    pub fn into_bytes(self) -> Box<[u8]> {
        // Convert a report into a tree of tags.
        let tag_groups = TagGroup::collection(self);
        // Linearize tag structure
        let tags: Vec<Tag> = tag_groups.tags().cloned().collect();
        // Remove duplicate tags
        let tags_cleaned = TagOptimizer::from_iter(tags)
            .remove_duplicates();
        // Compile tags down into ShortItems
        let tag_items = ShortItems::from_iter(tags_cleaned);
        // Convert ShortItems to bytes
        tag_items.into_bytes()
    }
}
