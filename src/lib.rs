use std::env;

struct Count<T: Iterator> {
    iter: T,
    next: Option<T::Item>
}

impl<T: Iterator> From<T> for Count<T> {
    fn from (iter: T) -> Count<T> {
        Count {
            iter: iter, next: None
        }
    }
}

impl<T:Iterator> Iterator for Count<T> where T::Item: PartialEq {
    type Item = (T::Item, i32);

    fn next (&mut self) -> Option<(T::Item, i32)> {
        let ch = match self.next.take() {
            Some (ch) => ch, // still have an item from previous run
            None => match self.iter.next() {
                Some (ch) => ch,
                None => return None // parent iterator is mpty
            }
        };

        let mut count = 1;

        loop {
            match self.iter.next() {
                None => {return Some ((ch, count))},
                Some (next) => {
                    if next == ch {
                        count += 1
                    } else {
                        self.next = Some (next);
                        return Some ((ch, count));
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        // turn argument into string
        let s = args[1].to_string();
        let t = args[2].to_string();

        println!("Euclidean Distance: {:?}", distance(s,t));
    }
}

pub fn distance(s: String, t: String) -> f64 {

    // Vector of tuples (counts, character)
    let mut v = Vec::<(i32, char)>::new();

        // use iterator on string
        for count in Count::from (s.chars()) {
            v.push((count.1, count.0));
        }

        for count in Count::from (t.chars()) {
            if  v.iter().any(|&(_, a)| a == count.0) {

                v.iter_mut().find(|&&mut (_, b)| b == count.0)
                    .map(|tag| {
                        tag.0 -= count.1
                    });

            } else {
                v.push((count.1, count.0));
            }

            // if verbous
            //println!("{:?}", v);
        }

        // multiply by itself and sum everything together
        let sum = v.iter()
            .map(|&(i, _)| i * i)
            .fold(0, |acc, i| acc + i);

        // reteurn sum squared
        (sum as f64).sqrt()
}



#[test]
fn returns_zero_on_identical_strings() {
        assert_eq!( 0.0, distance("A".to_string(), "A".to_string()) );
}

#[test]
fn returns_f64_on_nonidentical_strings() {
        assert_eq!( 1.4142135623730951, distance("John".to_string(), "Johnny".to_string()) );
}
