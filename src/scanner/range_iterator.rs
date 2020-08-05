use gcd::Gcd;
use rand::Rng;

pub struct RangeIterator {
    normalized_end: u32,
    normalized_first_pick: Option<u32>,
    normalized_pick: Option<u32>,
    actual_start: u32,
    step: u32,
}

/// An iterator that follows the `Linear Congruential Generator` algorithm.
///
/// For more information: https://en.wikipedia.org/wiki/Linear_congruential_generator
impl RangeIterator {
    /// Receives the the start and end of a range and normalize
    /// these values before selecting a coprime for the end of the range
    /// which will server as the step for the algorithm.
    ///
    /// For example, the range `1000-2500` will be normalized to `0-1500`
    /// before going through the algorithm.
    pub fn new(start: u32, end: u32) -> Self {
        let normalized_end = end - start;
        let step = pick_random_coprime(normalized_end);

        Self {
            normalized_end,
            step,
            normalized_first_pick: None,
            normalized_pick: None,
            actual_start: start,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = u16;

    // The next step is always bound by the formula: N+1 = (N + STEP) % TOP_OF_THE_RANGE
    // It will only stop once we generate a number equal to the first generated number.
    fn next(&mut self) -> Option<Self::Item> {
        // Randomly choose a number within the range to be the first
        // and assign it as a pick.
        if self.normalized_first_pick.is_none() {
            let mut rng = rand::thread_rng();
            let normalized_first_pick = rng.gen_range(0, self.normalized_end);

            self.normalized_first_pick = Some(normalized_first_pick);
            self.normalized_pick = Some(normalized_first_pick);
            return Some((self.actual_start + normalized_first_pick) as u16);
        }

        let current_pick = self.normalized_pick.unwrap();
        let next_pick = (current_pick + self.step) % self.normalized_end;

        // If the next pick is equal to the first pick this means that
        // we have iterated through the entire range.
        if next_pick == self.normalized_first_pick.unwrap() {
            return None;
        }

        self.normalized_pick = Some(next_pick);
        Some((self.actual_start + next_pick) as u16)
    }
}

/// The probability that two random integers are coprime to one another
/// works out to be around 61%, given that we can safely pick a random
/// number and test it. Just in case we are having a bad day and we cannot
/// pick a coprime number after 10 tries we just return "end - 1" which
/// is guaranteed to be a coprime, but won't provide ideal randomization.
///
/// We pick between "lower_range" and "upper_range" since values too close to
/// the boundaries, which in these case are the "start" and "end" arguments
/// would also provide non-ideal randomization as discussed on the paragraph
/// above.
fn pick_random_coprime(end: u32) -> u32 {
    let range_boundary = end / 4;
    let lower_range = range_boundary;
    let upper_range = end - range_boundary;
    let mut rng = rand::thread_rng();
    let mut candidate = rng.gen_range(lower_range, upper_range);

    for _ in 0..10 {
        if end.gcd(candidate) == 1 {
            return candidate;
        } else {
            candidate = rng.gen_range(lower_range, upper_range);
        }
    }

    end - 1
}

#[cfg(test)]
mod tests {
    use super::RangeIterator;

    #[test]
    fn range_iterator_iterates_through_the_entire_range() {
        let result = generate_sorted_range(1, 10);
        let expected_range = (1..10).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);

        let result = generate_sorted_range(1, 100);
        let expected_range = (1..100).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);

        let result = generate_sorted_range(1, 1000);
        let expected_range = (1..1000).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);

        let result = generate_sorted_range(1, 65_535);
        let expected_range = (1..65_535).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);

        let result = generate_sorted_range(1000, 2000);
        let expected_range = (1000..2000).into_iter().collect::<Vec<u16>>();
        assert_eq!(expected_range, result);
    }

    fn generate_sorted_range(start: u32, end: u32) -> Vec<u16> {
        let range = RangeIterator::new(start, end);
        let mut result = range.into_iter().collect::<Vec<u16>>();
        result.sort();

        result
    }
}
