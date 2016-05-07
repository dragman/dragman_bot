use std::ops::*;
use chrono::*;

pub struct DateIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate> {
    curr: Option<NaiveDate>,
    next: Option<NaiveDate>,
    func: F,
}

impl<F> DateIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate> {
    pub fn new(f: F) -> DateIterator<F> {
        DateIterator::from_date(Local::today().naive_local(), f)
    }

    pub fn from_date(date: NaiveDate, f: F) -> DateIterator<F> {
        DateIterator {
            curr: Some(date),
            next: Some(date),
            func: f,
        }
    }
}

impl<F> Iterator for DateIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate> {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        if let Some(d) = self.next {
            let new_next = (self.func)(&d);
            self.curr = self.next;
            self.next = new_next;
            return self.curr;
        } else {
            self.curr = None;
            self.next = None;
            return self.curr;
        }
    }
}

#[test]
fn it_works()
{
    let test_date = Local::today().naive_local();
    let d = DateIterator::from_date(test_date, |&d| d.checked_add(Duration::days(1)));
    let ds: Vec<_> = d.take(10).collect();
    println!("{:?}", ds);
    assert!(false);
}
