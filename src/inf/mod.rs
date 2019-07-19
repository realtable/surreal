use super::Surreal;

/// A struct to represent surreal numbers with at least one infinite set.
pub struct SurrealInf {
    left: Box<dyn Fn(u32) -> Option<Surreal>>,
    right: Box<dyn Fn(u32) -> Option<Surreal>>,
}

impl SurrealInf {
    /// Creates a new surreal number with infinite sets, where these sets are described by two
    /// closures that return surreal numbers with *non*-infinite sets. These series can also return
    /// the same number to represent a non-infinite set, as all non-infinite sets can be reduced to
    /// a single element.
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
    ///     Box::new(|n: u32| Some(surreal::ftos((n + 1) as f32))),
    ///     Box::new(|n: u32| None),
    /// );
    ///
    /// let epsilon = surreal::SurrealInf::new(
    ///     Box::new(|n: u32| Some(surreal::Surreal::new(vec![], vec![]))),
    ///     Box::new(|n: u32| Some(surreal::ftos(1.0/(2_i32.pow(n) as f32)))),
    /// );
    /// ```
    pub fn new(
        left: Box<dyn Fn(u32) -> Option<Surreal>>,
        right: Box<dyn Fn(u32) -> Option<Surreal>>,
    ) -> SurrealInf {
        SurrealInf { left, right }
    }

    /// Returns the left set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let omega = surreal::SurrealInf::new(
    ///     Box::new(|n: u32| Some(surreal::ftos((n + 1) as f32))),
    ///     Box::new(|n: u32| None),
    /// );
    ///
    /// if let Some(i) = omega.left(0) {
    ///     assert!(i == surreal::ftos(1.0));
    /// }
    /// ```
    pub fn left(&self, index: u32) -> Option<Surreal> {
        (self.left)(index)
    }

    /// Returns the right set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let epsilon = surreal::SurrealInf::new(
    ///     Box::new(|n: u32| Some(surreal::Surreal::new(vec![], vec![]))),
    ///     Box::new(|n: u32| Some(surreal::ftos(1.0/(2_i32.pow(n) as f32)))),
    /// );
    ///
    /// if let Some(i) = epsilon.right(0) {
    ///     assert!(i == surreal::ftos(1.0));
    /// }
    /// ```
    pub fn right(&self, index: u32) -> Option<Surreal> {
        (self.right)(index)
    }
}
