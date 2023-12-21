//! Spline [`Iterator`], in a nutshell.
//!
//! You can iterate over a [`Spline<K, V>`]’s keys with the [`IntoIterator`] trait on
//! `&Spline<K, V>`. This gives you iterated [`Key<K, V>`] keys.
//!
//! [`Spline<K, V>`]: crate::spline::Spline
//! [`Key<K, V>`]: crate::key::Key

use crate::{Key, Spline};

/// Iterator over spline keys.
///
/// This iterator type is guaranteed to iterate over sorted keys.
pub struct Iter<'a, T, V, const SIZE: usize>
where
  T: 'a,
  V: 'a,
{
  spline: &'a Spline<T, V, SIZE>,
  i: usize,
}

impl<'a, T, V, const SIZE: usize> Iterator for Iter<'a, T, V, SIZE> {
  type Item = &'a Key<T, V>;

  fn next(&mut self) -> Option<Self::Item> {
    let r = self.spline.0.get(self.i);

    if let Some(_) = r {
      self.i += 1;
    }

    r
  }
}

impl<'a, T, V, const SIZE: usize> IntoIterator for &'a Spline<T, V, SIZE> {
  type Item = &'a Key<T, V>;
  type IntoIter = Iter<'a, T, V, SIZE>;

  fn into_iter(self) -> Self::IntoIter {
    Iter { spline: self, i: 0 }
  }
}
