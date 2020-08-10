mod range_iterator;
use super::ScanOrder;
use range_iterator::RangeIterator;

/// Represents options of port scanning.
///
/// Right now all these options involve ranges, but in the future
/// it will also contain custom lists of ports.
pub enum PortStrategy {
    Serial(SerialRange),
    Random(RandomRange),
}

impl PortStrategy {
    pub fn pick(start: u16, end: u16, scan_type: ScanOrder) -> Self {
        match scan_type {
            ScanOrder::Serial => PortStrategy::Serial(SerialRange { start, end }),
            ScanOrder::Random => PortStrategy::Random(RandomRange { start, end }),
        }
    }

    pub fn order(&self) -> Vec<u16> {
        match self {
            PortStrategy::Serial(range) => range.generate(),
            PortStrategy::Random(range) => range.generate(),
        }
    }
}

/// Trait associated with a port strategy. Each PortStrategy must be able
/// to generate an order for future port scanning.
trait RangeOrder {
    fn generate(&self) -> Vec<u16>;
}

/// As the name implies SerialRange will always generate a vector in
/// ascending order.
pub struct SerialRange {
    start: u16,
    end: u16,
}

impl RangeOrder for SerialRange {
    fn generate(&self) -> Vec<u16> {
        (self.start..self.end).collect()
    }
}

/// As the name implies RandomRange will always generate a vector with
/// a random order. This vector is built following the LCG algorithm.
pub struct RandomRange {
    start: u16,
    end: u16,
}

impl RangeOrder for RandomRange {
    // Right now using RangeIterator and generating a range + shuffling the
    // vector is pretty much the same. The advantages of it will come once
    // we have to generate different ranges for different IPs without storing
    // actual vectors.
    //
    // Another benefit of RangeIterator is that it always generate a range with
    // a certain distance between the items in the Array. The chances of having
    // port numbers close to each other are pretty slim due to the way the
    // algorithm works.
    fn generate(&self) -> Vec<u16> {
        RangeIterator::new(self.start.into(), self.end.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::PortStrategy;
    use crate::ScanOrder;

    #[test]
    fn serial_strategy() {
        let strategy = PortStrategy::pick(1, 100, ScanOrder::Serial);
        let result = strategy.order();
        let expected_range = (1..100).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);
    }
    #[test]
    fn random_strategy() {
        let strategy = PortStrategy::pick(1, 100, ScanOrder::Random);
        let mut result = strategy.order();
        let expected_range = (1..100).into_iter().collect::<Vec<u16>>();
        assert_ne!(expected_range, result);

        result.sort();
        assert_eq!(expected_range, result);
    }
}
