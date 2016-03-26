use std::ops::Deref;
use std::iter::{IntoIterator, FromIterator};
use std::option;

/// Extract a single value from an iterator
///
/// This only exists because `Option` doesn't implement `FromIterator`.
pub struct One<T>(Option<T>);

impl<T> One<T> {
    pub fn is_empty(&self) -> bool { self.0.is_none() }
}

impl<T> Deref for One<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> AsRef<Option<T>> for One<T> {
    fn as_ref(&self) -> &Option<T> { &self.0 }
}

impl<T> Into<Option<T>> for One<T> {
    fn into(self) -> Option<T> { self.0 }
}

impl<T> FromIterator<T> for One<T> {
    fn from_iter<IT>(iter: IT) -> Self
        where IT: IntoIterator<Item=T>
    {
        One(iter.into_iter().next())
    }
}

impl<T> IntoIterator for One<T> {
    type Item = T;
    type IntoIter = option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
