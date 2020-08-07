mod range_iterator;
use super::ScanOrder;
use range_iterator::RangeIterator;

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

trait RangeOrder {
    fn generate(&self) -> Vec<u16>;
}

pub struct SerialRange {
    start: u16,
    end: u16,
}

impl RangeOrder for SerialRange {
    fn generate(&self) -> Vec<u16> {
        (self.start..self.end).collect()
    }
}

pub struct RandomRange {
    start: u16,
    end: u16,
}

impl RangeOrder for RandomRange {
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
