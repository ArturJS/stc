use std::cell::RefCell;
use std::rc::Rc;
use swc_common::Span;

pub trait Visit<T: ?Sized> {
    fn visit(&mut self, value: &T);
}

pub trait VisitWith<V: ?Sized> {
    fn visit_with(&self, visitor: &mut V)
    where
        V: Visit<Self>,
    {
        visitor.visit(self)
    }

    fn visit_children_with(&self, visitor: &mut V);
}

impl<T, V> Visit<T> for V
where
    V: ?Sized,
    T: ?Sized + VisitWith<Self>,
{
    default fn visit(&mut self, val: &T) {
        val.visit_children_with(self)
    }
}

impl<T, V> VisitWith<V> for Box<T>
where
    T: ?Sized,
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, v: &mut V) {
        v.visit(&**self)
    }
}

impl<T, V> VisitWith<V> for [T]
where
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, visitor: &mut V) {
        self.iter().for_each(|value| visitor.visit(value))
    }
}

impl<T, V> VisitWith<V> for RefCell<T>
where
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, v: &mut V) {
        v.visit(&*self.borrow())
    }
}

impl<T, V> Visit<T> for &'_ mut V
where
    T: VisitWith<Self>,
    V: Visit<T>,
{
    fn visit(&mut self, value: &T) {
        (**self).visit(value)
    }
}

impl<V> VisitWith<V> for Span
where
    V: ?Sized,
{
    /// Noop
    #[inline]
    fn visit_children_with(&self, _: &mut V) {}
}

impl<T, V> VisitWith<V> for Rc<T>
where
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, visitor: &mut V) {
        visitor.visit(&**self)
    }
}

impl<T, V> VisitWith<V> for Vec<T>
where
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, visitor: &mut V) {
        self.iter().for_each(|value| visitor.visit(value))
    }
}

impl<T, V> VisitWith<V> for Option<T>
where
    V: ?Sized + Visit<T>,
{
    fn visit_children_with(&self, visitor: &mut V) {
        match self {
            Some(value) => visitor.visit(value),
            None => {}
        }
    }
}