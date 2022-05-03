use std::rc::Rc;

// 内部ではvecを使用している.
#[derive(Clone)]
pub struct VecStack<T>
where
    T: Clone
{
    inner : Vec<T>
}

impl<T> VecStack<T> 
where
    T: Clone,
    VecStack<T>: AsRef<VecStack<T>>
{
    pub fn new() -> Self {
        Self{
            inner: Vec::<T>::new()
        }
    }
    pub fn new_with_init_data() -> Self {
        Self {
            inner: Vec::<T>::new()
        }
    }
    pub fn push(&mut self, data: T) {
        self.inner.push(data);
    }
    // pushした後に、stackのcopyが欲しい時
    pub fn push_clone(&mut self, data: T) -> Self {
        self.inner.push(data);
        self.clone()
    }
    // pushした後に、stackのRcが欲しい時
    pub fn push_rc(&mut self, data: T) -> Rc<Self> {
        self.inner.push(data);
        Rc::new(self.clone())
    }
}

pub struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(None)
    }
    pub fn new_with_init_data(x: Vec<T>) -> Self {
        let mut s = Self::new();
        for v in x.into_iter().rev() {
            s = s.unshift(v)
        }
        s
    }
    // 先頭に要素xを追加する.
    pub fn unshift(self, x:T) -> Self {
        Self(Some(Rc::new((x, self))))
    }
    // 先頭要素を削除する.
    // pub fn pop(self) -> Self {
    //     let inner = match self.0 {
    //         Some(v) => v,
    //         None => return Self(None),
    //     };
    //     let cdr = inner.1;
    // }
    pub fn peek(&self) -> Option<&T> {
        if let Some(rc) = &self.0 {
            Some(&rc.0)
        } else {
            None
        }
    }
}
impl<T: Clone> Stack<T> {
    // 先頭要素を削除する.
    pub fn pop(self) -> (Self, Option<T>) {
        if let Some(rc) = self.0 {
            // TODO: 基本ここのcloneは呼ばれない認識でoK?
            let (head, tail) = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            (tail, Some(head))
        } else {
            (Self(None), None)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let s = Stack::new();
        let s = s.unshift(3);
        assert_eq!(&3, s.peek().unwrap());
        let s = s.pop().0;
        assert_eq!(None, s.peek());
    }

    #[test]
    fn test_new_with_initial_data() {
        let v:Vec<i32> = vec![1,2,3];
        let s = Stack::new_with_init_data(v);
        assert_eq!(&1, s.peek().unwrap());
        let s = s.pop().0;
        assert_eq!(&2, s.peek().unwrap());
        let s = s.pop().0;
        assert_eq!(&3, s.peek().unwrap());
    }
}
