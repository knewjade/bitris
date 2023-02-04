/// A collection of operations to take one from a order.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum PopOp {
    #[default] First,
    Second,
}

/// Preserves the reference status of the order.
/// The next items to be manipulated can be identified.
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct OrderCursor<'a, T> {
    items: &'a [T],

    /// Current index of the item in the entire order.
    /// When `current` is None, `next` is also always None, indicating that the cursor is empty.
    current: Option<usize>,

    /// Current index of remaining items.
    /// When `next` is None, indicating that there are no items other than `current`.
    next: Option<usize>,
}

impl<'a, T> OrderCursor<'a, T> {
    /// Returns `true` if a pop-able item exists next.
    #[inline]
    pub fn has_next(&self) -> bool {
        self.current.is_some()
    }

    /// Returns the count of items not used.
    #[inline]
    pub fn len_remaining(&self) -> usize {
        let current = self.current.and(Some(1)).unwrap_or(0);
        let next = self.next.map(|next| self.items.len() - next).unwrap_or(0);
        current + next
    }

    /// Returns shapes that have not been used as an order.
    #[inline]
    pub fn iter_remaining(&self) -> impl Iterator<Item=&T> {
        let current = match self.current {
            Some(index) => &self.items[index..=index],
            None => &[],
        };
        let next = match self.next {
            Some(index) => &self.items[index..],
            None => &[],
        };
        current.iter().chain(next.iter())
    }

    /// Returns a popped item and a next cursor.
    /// If nothing that can be popped next, None is returned for the item.
    /// The next cursor is always returned as available.
    ///
    /// The item returned by the first is that received before the second.
    /// Therefore, this function ensures the following behaviors.
    ///
    /// * If the first returns None, the second is always None.
    ///   The last item is always assigned to the first.
    ///
    /// * If only the first is used, it's equivalent to consuming from the current of the order.
    ///   In other words, equivalent to not using a hold.
    ///   Note, however, this means that "The second is not always the hold because the last one is assigned to the first, regardless of the hold".
    #[inline]
    pub fn pop(self, op: PopOp) -> (Option<&'a T>, OrderCursor<'a, T>) {
        return match op {
            PopOp::First => {
                match self.current {
                    None => (None, self),
                    Some(current_index) => {
                        let item = self.items.get(current_index);
                        match self.next {
                            None => (item, OrderCursor {
                                items: self.items,
                                current: None,
                                next: None,
                            }),
                            Some(next_index) => {
                                (item, OrderCursor {
                                    items: self.items,
                                    current: self.next,
                                    next: if next_index + 1 < self.items.len() {
                                        Some(next_index + 1)
                                    } else {
                                        None
                                    },
                                })
                            }
                        }
                    }
                }
            }
            PopOp::Second => {
                match self.next {
                    None => (None, self),
                    Some(index) => {
                        let item = self.items.get(index);
                        (item, OrderCursor {
                            items: self.items,
                            current: self.current,
                            next: if index + 1 < self.items.len() {
                                Some(index + 1)
                            } else {
                                None
                            },
                        })
                    }
                }
            }
        };
    }

    /// Returns a current item based on Op.
    #[inline]
    pub fn peek(&self, op: PopOp) -> Option<&T> {
        match op {
            PopOp::First => self.first(),
            PopOp::Second => self.second(),
        }
    }

    /// Returns a current first item.
    #[inline]
    pub fn first(&self) -> Option<&T> {
        self.current.map(|index| &self.items[index])
    }

    /// Returns a current second item.
    #[inline]
    pub fn second(&self) -> Option<&T> {
        self.next.map(|index| &self.items[index])
    }
}

impl<'a, T> From<&'a [T]> for OrderCursor<'a, T> {
    fn from(items: &'a [T]) -> Self {
        match items.len() {
            0 => Self { items, current: None, next: None },
            1 => Self { items, current: Some(0), next: None },
            _ => Self { items, current: Some(0), next: Some(1) },
        }
    }
}

impl<'a, T> From<&'a Vec<T>> for OrderCursor<'a, T> {
    fn from(items: &'a Vec<T>) -> Self {
        Self::from(items.as_slice())
    }
}


#[cfg(test)]
mod tests {
    use itertools::*;

    use crate::prelude::*;

    #[test]
    fn empty() {
        let shapes = Vec::<Shape>::new();
        let cursor = OrderCursor::<Shape>::from(&shapes);

        // []()
        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), vec![].iter());
        assert_eq!(cursor.first(), None);
        assert_eq!(cursor.second(), None);

        let (shape, cursor) = cursor.pop(PopOp::First);
        assert!(!cursor.has_next());
        assert_eq!(shape, None);

        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert!(!cursor.has_next());
        assert_eq!(shape, None);
    }

    #[test]
    fn one() {
        use Shape::*;

        let shapes = vec![T];
        let cursor = OrderCursor::<Shape>::from(&shapes);

        // [](T)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);
        assert_equal(cursor.iter_remaining(), [T].iter());
        assert_eq!(cursor.first(), Some(&T));
        assert_eq!(cursor.second(), None);
        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(shape, None);

        // [](T)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);
        assert_equal(cursor.iter_remaining(), [T].iter());
        assert_eq!(cursor.first(), Some(&T));
        assert_eq!(cursor.second(), None);
        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&T));

        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), [].iter());
        assert_eq!(cursor.first(), None);
        assert_eq!(cursor.second(), None);
    }

    #[test]
    fn pop_first() {
        use Shape::*;

        let shapes = vec![O, S];
        let cursor = OrderCursor::<Shape>::from(&shapes);

        // [](O)S
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 2);
        assert_equal(cursor.iter_remaining(), [O, S].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&O));
        assert_eq!(cursor.peek(PopOp::Second), Some(&S));
        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&O));

        // [](S)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);
        assert_equal(cursor.iter_remaining(), [S].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&S));
        assert_eq!(cursor.peek(PopOp::Second), None);
        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&S));

        // []()
        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), [].iter());
        assert_eq!(cursor.peek(PopOp::First), None);
        assert_eq!(cursor.peek(PopOp::Second), None);
        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, None);

        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), [].iter());
        assert_eq!(cursor.peek(PopOp::First), None);
        assert_eq!(cursor.peek(PopOp::Second), None);
    }

    #[test]
    fn pop_second() {
        use Shape::*;

        let shapes = vec![O, S, T];
        let cursor = OrderCursor::<Shape>::from(&shapes);

        // [](O)ST
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 3);
        assert_equal(cursor.iter_remaining(), [O, S, T].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&O));
        assert_eq!(cursor.peek(PopOp::Second), Some(&S));
        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(shape, Some(&S));

        // [O](T)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 2);
        assert_equal(cursor.iter_remaining(), [O, T].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&O));
        assert_eq!(cursor.peek(PopOp::Second), Some(&T));
        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(shape, Some(&T));

        // [](O)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);
        assert_equal(cursor.iter_remaining(), [O].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&O));
        assert_eq!(cursor.peek(PopOp::Second), None);
        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(shape, None);

        // [](O)
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);
        assert_equal(cursor.iter_remaining(), [O].iter());
        assert_eq!(cursor.peek(PopOp::First), Some(&O));
        assert_eq!(cursor.peek(PopOp::Second), None);
        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&O));

        // []()
        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), [].iter());
        assert_eq!(cursor.peek(PopOp::First), None);
        assert_eq!(cursor.peek(PopOp::Second), None);
        let (index, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(index, None);

        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);
        assert_equal(cursor.iter_remaining(), vec![].iter());
        assert_eq!(cursor.peek(PopOp::First), None);
        assert_eq!(cursor.peek(PopOp::Second), None);
    }
}
