pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warnig: You've used over your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn if_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();

        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }
}
