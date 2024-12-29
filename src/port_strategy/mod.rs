//! Provides a means to hold configuration options specifically for port scanning.

use crate::input::ScanOrder;
use blackrock2::BlackRockIter;
use rand::seq::SliceRandom;
use std::collections::Bound;
use std::ops::RangeBounds;
use itertools::Either;

/// Represents options of port scanning.
///
/// Right now all these options involve ranges, but in the future
/// it will also contain custom lists of ports.
#[derive(Debug)]
pub enum PortStrategy {
    List(Box<[u16]>),
    Serial(SerialRange),
    Random(RandomRange),
}

impl PortStrategy {
    pub fn range(range: impl RangeBounds<u16>, order: ScanOrder) -> Self {
        fn _range(range: ExclusivePortRange, order: ScanOrder) -> PortStrategy {
            match order {
                ScanOrder::Serial => PortStrategy::Serial(SerialRange(range)),
                ScanOrder::Random => PortStrategy::Random(RandomRange(range)),
            }
        }
        
        _range(ExclusivePortRange::from(range), order)
    }

    pub fn list(list: impl Into<Box<[u16]>>, order: ScanOrder) -> Self {
        fn _list(mut list: Box<[u16]>, order: ScanOrder) -> PortStrategy {
            match order {
                ScanOrder::Serial => PortStrategy::List(list),
                ScanOrder::Random => {
                    list.shuffle(&mut rand::thread_rng());
                    PortStrategy::List(list)
                }
            }
        }
        
        _list(list.into(), order)
    }
    


    pub fn iter(&self) -> impl Iterator<Item = u16> + use<'_> {
        match self {
            PortStrategy::List(ports) => {
                Either::Right(ports.iter().copied())
            }
            PortStrategy::Serial(range) => Either::Left(Either::Right(range.iter())),
            PortStrategy::Random(range) => Either::Left(Either::Left(range.iter())),
        }
    }
}

/// Trait associated with a port strategy. Each PortStrategy must be able
/// to generate an order for future port scanning.
trait RangeOrder {
    fn iter(&self) -> impl Iterator<Item = u16>;
}

/// As the name implies SerialRange will always generate a vector in
/// ascending order.
#[derive(Debug)]
pub struct SerialRange(ExclusivePortRange);

impl RangeOrder for SerialRange {
    fn iter(&self) -> impl Iterator<Item = u16> {
        (self.0.start..self.0.end).map(|x| x as u16)
    }
}

/// As the name implies RandomRange will always generate a vector with
/// a random order. This vector is built following the LCG algorithm.
#[derive(Copy, Clone, Debug)]
pub struct ExclusivePortRange {
    start: u32,
    end: u32,
}

impl<B: RangeBounds<u16>> From<B> for ExclusivePortRange {
    fn from(value: B) -> Self {
        let start = match value.start_bound() {
            Bound::Included(&inclusive) => inclusive as u32,
            Bound::Excluded(&excluded) => excluded as u32 + 1,
            Bound::Unbounded => 0,
        };
        let end = match value.end_bound() {
            Bound::Included(&inclusive) => inclusive as u32 + 1,
            Bound::Excluded(&excluded) => excluded as u32,
            Bound::Unbounded => 1 << 16,
        };

        Self { start, end }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RandomRange(ExclusivePortRange);

impl RangeOrder for RandomRange {
    // use blackrock like the original masscan
    fn iter(&self) -> impl Iterator<Item = u16> {
        let range = self.0.end.saturating_sub(self.0.start);
        // only ever accessed if self.start < 1 << 16
        let offset = self.0.start as u16;
        BlackRockIter::new(range as u64).map(move |x| (x as u16) + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::PortStrategy;
    use crate::input::ScanOrder;
    use itertools::Itertools;

    #[test]
    fn serial_strategy_with_range() {
        let expected_range = 1..=100;

        let strategy = PortStrategy::range(expected_range.clone(), ScanOrder::Serial);
        assert!(expected_range.eq(strategy.iter()));
    }
    #[test]
    fn random_strategy_with_range() {
        let range = 1..=100;

        let strategy = PortStrategy::range(range.clone(), ScanOrder::Random);

        assert!(range.clone().ne(strategy.iter()));
        assert!(range.eq(strategy.iter().sorted()));
    }

    #[test]
    fn serial_strategy_with_ports() {
        let list = [80, 443];
        let strategy = PortStrategy::list(list, ScanOrder::Serial);
        let result = strategy.iter();
        assert!(list.iter().copied().eq(result));
    }

    #[test]
    fn random_strategy_with_ports() {
        let expected_range = 1..300;

        let strategy = PortStrategy::list(expected_range.clone().collect_vec(), ScanOrder::Random);

        assert!(expected_range.clone().ne(strategy.iter()));
        assert!(expected_range.eq(strategy.iter().sorted()));
    }
}
