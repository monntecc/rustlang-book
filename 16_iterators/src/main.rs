/*
    The iterator pattern allows you to perform some task on a sequence of items in turn.
    An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
    When you use iterators, you don’t have to reimplement that logic yourself.
*/

#[allow(dead_code)]
#[allow(unused_variables)]

/* IMPLEMENTING OWN ITERATOR */
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    // Create simple iterator based on vector
    let v1_iter = v1.iter();
    // Loop through items
    for value in v1_iter {
        println!("Got: {}", value);
    }

    /* METHODS ON ITERATORS */
    let incremented: Vec<_> = v1.iter().map(|x| x + 1).collect();
    for value in incremented {
        println!("Got incremented value: {}", value);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

/*
    Function that accepts a vector of shoes and the shoe size,
    makes vector iterator, filter through and find all accuraces
*/
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        /*
            To specify an iterator, we have three options:
            - .iter() - immtable iterator
            - .iter_mut() - mutable iterator
            - .into_iter() - ownership model of iterator
        */
        let mut v1_iter = v1.iter();

        /*
          Iterator `.next()` methods, executed the next value
          of current collection.
        */
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();
        /*
            Sum method will call repeateatly the `.next()` method,
            to sum all of the items.
        */
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // Zip method concat two iterators into one
            .map(|(a, b)| a * b) // Map method modify each value
            .filter(|x| x % 3 == 0) // Filter method filters the values
            .sum(); // Sum method sum the all values

        assert_eq!(18, sum);
    }
}
