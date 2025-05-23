use std::ops::Range;

use tree_sitter::Point;

use self::enums::{Capability, Search};

use super::{enums, metadata::RunnableMeta};

#[derive(Clone, Debug)]
pub struct Runnable {
    pub name: String,
    pub filepath: String,
    pub range: Range<CursorPosition>,
    pub meta: RunnableMeta,
}

pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}
pub struct Buffer<'a> {
    pub content: &'a str,
    pub filepath: String,
    pub position: CursorPosition,
}

impl Buffer<'_> {
    pub fn new(content: &str, filepath: String, position: CursorPosition) -> Buffer {
        Buffer {
            content,
            filepath,
            position,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct CursorPosition {
    pub row: usize,
    pub col: usize,
}

impl CursorPosition {
    pub(crate) fn to_point(&self) -> Point {
        Point::new(self.row, self.col)
    }

    pub(crate) const fn from_point(point: Point) -> CursorPosition {
        CursorPosition {
            row: point.row,
            col: point.column,
        }
    }

    pub(crate) const fn new(row: usize, col: usize) -> CursorPosition {
        CursorPosition { row, col }
    }

    pub(crate) fn in_range(&self, range: std::ops::Range<Point>) -> bool {
        let cursor = self.to_point();
        cursor.row >= range.start.row && cursor.row <= range.end.row
    }
}

pub struct Target<'a> {
    pub category: enums::Capability,
    pub buffer: Buffer<'a>,
    pub search_strategy: enums::Search,
}

impl Target<'_> {
    pub fn new(category: enums::Capability, buffer: Buffer) -> Target {
        Target {
            category,
            buffer,
            search_strategy: enums::Search::Nearest,
        }
    }

    pub fn override_search_strategy(&mut self, search_strategy: enums::Search) {
        self.search_strategy = search_strategy;
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CapabilityDetails {
    pub(crate) capability: Capability,
    pub(crate) search: Search,
    pub(crate) description: String,
}
