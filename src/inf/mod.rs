use super::Surreal;

/// A struct to represent surreal numbers with infinite sets.
#[derive(Debug, Clone)]
pub struct SurrealInf<T: Fn(u32) -> Surreal> {
    left: Option<T>,
    right: Option<T>,
}

impl<T: Fn(u32) -> Surreal> SurrealInf<T> {
    /// Creates a new surreal number given two `Option` enums of closures that describe a series of
    /// regular `Surreal` types.
    ///
    /// # Panics
    ///
    /// Because infinite sets can't be fully computed, there is no guarantee that all items in the
    /// left set are less than all items in the right set. This means that, not only is comparison
    /// impossible, but arithmetic is as well. 
    ///
    /// # Examples
    ///
    /// ```
    /// let omega = surreal::SurrealInf::new(
    ///     Some(|n: u32| surreal::ftos(n as f32)),
    ///     None,
    /// );
    /// ```
    pub fn new(left: Option<T>, right: Option<T>) -> SurrealInf<T> {
        SurrealInf { left, right }
    }

    /// Returns the left set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let omega = surreal::SurrealInf::new(
    ///     Some(|n: u32| surreal::ftos(n as f32)),
    ///     None,
    /// );
    ///
    /// if let Some(i) = omega.left(0) {
    ///     assert!(i == zero);
    /// }
    /// ```
    pub fn left(&self, index: u32) -> Option<Surreal> {
        if let Some(ref i) = self.left {
            Some(i(index))
        } else {
            None
        }
    }
    
    /// Returns the right set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let omega = surreal::SurrealInf::new(
    ///     Some(|n: u32| surreal::ftos(n as f32)),
    ///     None,
    /// );
    ///
    /// assert!(omega.right(0) == None);
    /// ```
    pub fn right(&self, index: u32) -> Option<Surreal> {
        if let Some(ref i) = self.right {
            Some(i(index))
        } else {
            None
        }
    }
}
