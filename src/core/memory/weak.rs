// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::marker::PhantomData;
use std::rc::Rc as StdRc;
use std::rc::Weak as StdWeak;

#[derive(Debug)]
pub struct Weak<T> {
    inner: StdWeak<T>,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
pub struct Rc<T> {
    inner: StdRc<T>,
    _marker: PhantomData<T>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        Rc {
            inner: StdRc::new(value),
            _marker: PhantomData,
        }
    }

    pub fn downgrade(this: &Self) -> Weak<T> {
        Weak {
            inner: StdRc::downgrade(&this.inner),
            _marker: PhantomData,
        }
    }

    pub fn strong_count(&self) -> usize {
        StdRc::strong_count(&self.inner)
    }

    pub fn weak_count(&self) -> usize {
        StdRc::weak_count(&self.inner)
    }

    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        StdRc::try_unwrap(this.inner).map_err(|rc| Rc {
            inner: rc,
            _marker: PhantomData,
        })
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Rc {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Weak<T> {
    pub fn new() -> Self {
        Weak {
            inner: StdWeak::new(),
            _marker: PhantomData,
        }
    }

    pub fn upgrade(&self) -> Option<Rc<T>> {
        self.inner.upgrade().map(|rc| Rc {
            inner: rc,
            _marker: PhantomData,
        })
    }

    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    pub fn weak_count(&self) -> usize {
        self.inner.weak_count()
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Weak {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Weak<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_basic() {
        let rc = Rc::new(42);
        let weak = Rc::downgrade(&rc);

        assert_eq!(Rc::strong_count(&rc), 1);
        assert_eq!(weak.strong_count(), 1);

        let rc2 = weak.upgrade().unwrap();
        assert_eq!(*rc2, 42);
        assert_eq!(Rc::strong_count(&rc), 2);
    }

    #[test]
    fn test_weak_upgrade_after_drop() {
        let weak: Weak<i32>;
        {
            let rc = Rc::new(100);
            weak = Rc::downgrade(&rc);
            assert!(weak.upgrade().is_some());
        }
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_multiple_weak() {
        let rc = Rc::new("hello".to_string());
        let weak1 = Rc::downgrade(&rc);
        let weak2 = weak1.clone();

        assert_eq!(weak1.weak_count(), 2);

        drop(weak1);
        assert_eq!(weak2.weak_count(), 1);

        drop(rc);
        assert!(weak2.upgrade().is_none());
    }

    #[test]
    fn test_rc_weak_interaction() {
        let rc = Rc::new(vec![1, 2, 3]);
        let weak1 = Rc::downgrade(&rc);
        let weak2 = Rc::downgrade(&rc);

        assert_eq!(Rc::strong_count(&rc), 1);
        assert_eq!(weak1.weak_count(), 2);

        drop(rc);

        assert!(weak1.upgrade().is_none());
        assert!(weak2.upgrade().is_none());
    }

    #[test]
    fn test_strong_and_weak_counts() {
        let rc = Rc::new(42);
        assert_eq!(Rc::strong_count(&rc), 1);
        assert_eq!(Rc::weak_count(&rc), 0);

        let weak = Rc::downgrade(&rc);
        assert_eq!(Rc::weak_count(&rc), 1);

        let rc2 = weak.upgrade().unwrap();
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(Rc::strong_count(&rc2), 2);

        drop(rc);
        assert_eq!(Rc::strong_count(&rc2), 1);

        drop(rc2);
        assert!(weak.upgrade().is_none());
    }
}
