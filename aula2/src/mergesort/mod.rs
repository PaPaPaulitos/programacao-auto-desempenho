use rayon::join;

pub struct MergeSort<T> {
    data: Vec<T>,
}

impl<T: Ord + Send + Copy> MergeSort<T> {
    pub fn new(data: Vec<T>) -> Self {
        MergeSort { data }
    }

    pub fn sort(self) -> Vec<T> {
        let len = self.data.len();
        if len <= 1 {
            return self.data;
        }

        let mid = len / 2;
        let (left, right) = self.data.split_at(mid);
        let left_vec = left.to_vec();
        let right_vec = right.to_vec();

        let (sorted_left, sorted_right) = rayon::join(|| MergeSort::new(left_vec).sort(), || MergeSort::new(right_vec).sort());

        MergeSort::merge(sorted_left, sorted_right)
    }

    fn merge(left: Vec<T>, right: Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut left_iter = left.into_iter();
        let mut right_iter = right.into_iter();

        let mut left_elem = left_iter.next();
        let mut right_elem = right_iter.next();

        while left_elem.is_some() || right_elem.is_some() {
            match (left_elem, right_elem) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        result.push(l);
                        left_elem = left_iter.next();
                    } else {
                        result.push(r);
                        right_elem = right_iter.next();
                    }
                },
                (Some(l), None) => {
                    result.push(l);
                    left_elem = left_iter.next();
                },
                (None, Some(r)) => {
                    result.push(r);
                    right_elem = right_iter.next();
                },
                (None, None) => break,
            }
        }

        result
    }
}
