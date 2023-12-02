pub struct Stack<T> {
    stack: Vec<T>,
    count: std::primitive::usize,
}

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            stack: Vec::<T>::new(),
            count: 0,
        }
    }

    pub fn count(&self) -> std::primitive::usize {
        self.count
    }

    pub fn push(&mut self, data: T) {
        self.stack.insert(self.count, data);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            Err(Error::new("can not pop empty stack"))
        } else {
            self.count -= 1;
            Ok(self.stack.remove(self.count))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack() {
        let mut test_stack = Stack::new();
        let (test_data_1, test_data_2, test_data_3) = (10, 42, 59492957);

        test_stack.push(test_data_1.clone());
        test_stack.push(test_data_2.clone());
        test_stack.push(test_data_3.clone());

        assert!(test_stack.pop().unwrap() == test_data_3);
        assert!(test_stack.pop().unwrap() == test_data_2);
        assert!(test_stack.pop().unwrap() == test_data_1);
        assert!(test_stack.pop().is_err());
    }
}
