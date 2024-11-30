/// Iterables for reports

use std::iter::FlatMap;
use std::slice::Iter;

use super::collection::{Collection, CollectionItem};
use super::error::MissingIdError;
use super::field_types::ReportId;
use super::format::{ReportFormat, ReportItem, TooLargeError};
use super::report::Report;

/// Helper function for folding an iterator into distinct items, preserving order.
fn fold_unique<T: PartialEq>(mut vec: Vec<T>, item: T) -> Vec<T> {
    if !vec.contains(&item) {
        vec.push(item)
    }
    vec
}

pub struct ReportIter<'a> {
    // Because an immutable and mutable borrow cannot exist at the same time, we are guaranteed
    // that, while the ReportIterator lives, the collection remains unchanged.
    remaining: Iter<'a, CollectionItem>,
    subiterator: Option<Box<ReportIter<'a>>>,
}

impl <'a> ReportIter<'a> {
    pub fn over(collection: &'a Collection) -> Self {
        Self {
            remaining: collection.items().iter(),
            subiterator: None,
        }
    }
}

impl <'a> Iterator for ReportIter<'a> {
    type Item = &'a Report;

    fn next(&mut self) -> Option<Self::Item> {
        loop { // Loop to model tail recursion
            // Attempt to get the next subitem
            if let Some(subiterator) = &mut self.subiterator {
                if let Some(report) = subiterator.next() {
                    return Some(report);
                }
            }
    
            // If no subitem exists, go to the next mainitem and potentially recurse.
            let next_item = self.remaining.next();
            match next_item {
                Some(CollectionItem::Report(report)) => {
                    return Some(report);
                },
                Some(CollectionItem::Collection(collection)) => {
                    self.subiterator = Some(Box::new(ReportIter::over(collection)));
                    // Restart loop.
                    // Normally, this would be handled by tail recursion.
                },
                None => {
                    return None;
                },
            }
        }
    }
}



// Implement methods on the collection to generate reports and report formats
pub trait ToReportIterator<'a>: Sized {
    type ReportIter: Iterator<Item = &'a Report>;
    /// Returns an iterator over all reports in this Collection.
    fn to_report_iter(self) -> Self::ReportIter;

    /// Create an unfilled ReportFormat with this Collection's input reports.
    fn input_report_format(self, report_id: Option<ReportId>) -> Result<ReportFormat, TooLargeError> {
        let report_items = self.to_report_iter()
            .filter(|report| report.is_input())
            .filter(|report| report.report_id == report_id)
            .flat_map(|report|
                      [ReportItem::from_report(report)].repeat(report.report_count as usize));

        ReportFormat::new_with_opt_id(report_id).copy_from_iter(report_items)
    }
    
    /// Create an unfilled ReportFormat with this Collection's output reports.
    fn output_report_format(self, report_id: Option<ReportId>) -> Result<ReportFormat, TooLargeError> {
        let report_items = self.to_report_iter()
            .filter(|report| report.is_output())
            .filter(|report| report.report_id == report_id)
            .flat_map(|report|
                      [ReportItem::from_report(report)].repeat(report.report_count as usize));

        ReportFormat::new_with_opt_id(report_id).copy_from_iter(report_items)
    }
    
    /// Create an unfilled ReportFormat with this Collection's feature reports.
    fn feature_report_format(self, report_id: Option<ReportId>) -> Result<ReportFormat, TooLargeError> {
        let report_items = self.to_report_iter()
            .filter(|report| report.is_feature())
            .filter(|report| report.report_id == report_id)
            .flat_map(|report|
                      [ReportItem::from_report(report)].repeat(report.report_count as usize));

        ReportFormat::new_with_opt_id(report_id).copy_from_iter(report_items)
    }

    /// Collect all IDs contained.
    /// Returns MissingIdError if some but not all reports have IDs.
    fn input_ids(self) -> Result<Box<[ReportId]>, MissingIdError> {
        let report_id_opts = self.to_report_iter()
            .filter(|report| report.is_input())
            .map(|report| report.report_id)
            .fold(Vec::new(), fold_unique);
        
        if report_id_opts.len() == 1 && report_id_opts[0] == None {
            Ok(Box::new([]))
        } else if report_id_opts.iter().all(Option::is_some) {
            Ok(report_id_opts.into_iter()
               .map(Option::unwrap)
               .collect())
        } else {
            Err(MissingIdError {})
        }
    }
}

impl<'a> ToReportIterator<'a> for &'a Collection {
    type ReportIter = ReportIter<'a>;
    /// Returns an iterator over all reports in this Collection.
    fn to_report_iter(self) -> ReportIter<'a> {
        ReportIter::over(self)
    }
}

impl<'a> ToReportIterator<'a> for &'a [Collection] {
    type ReportIter = FlatMap<
        std::slice::Iter<'a, Collection>,
        ReportIter<'a>,
        fn(&'a Collection) -> <&'a Collection as ToReportIterator<'_>>::ReportIter>;

    /// Returns an iterator over all reports in this Collection.
    fn to_report_iter(self) -> Self::ReportIter {
        self.iter().flat_map(<&'a Collection>::to_report_iter)
    }
}

