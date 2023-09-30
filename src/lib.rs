pub use either::Either;
use Either::*;

/// Extention trait to create the Either type from a boolean. Can be implemented on any type.
pub trait Eitherable {
    /// If the boolean is true, returns `Either::Left(left)`. If it is false, returns `Either::Right(right)`.
    /// `left` and `right` are evaluated eagerly. For lazy evaluation, use `either_else()`
    /// # Examples
    ///
    /// ```
    /// use eitherable::*;
    ///
    /// let x = true;
    /// assert_eq!(x.either(1, "right"), Either::Left(1));
    ///
    /// let x = false;
    /// assert_eq!(x.either(1, "right"), Either::Right("right"));
    /// ```
    fn either<L, R>(&self, left: L, right: R) -> Either<L, R>;

    /// Same as `either()`, but the values are lazily evaluated.
    ///
    /// # Examples
    ///
    /// ```
    /// use eitherable::*;
    ///
    /// let x = true;
    /// assert_eq!(x.either_else(|| 1,|| "right"), Either::Left(1));
    ///
    /// let x = false;
    /// assert_eq!(x.either_else(|| 1,|| "right"), Either::Right("right"));
    /// ```
    fn either_else<L, FL: FnOnce() -> L, R, FR: FnOnce() -> R>(
        &self,
        l_func: FL,
        r_func: FR,
    ) -> Either<L, R>;
}

impl Eitherable for bool {
    fn either<L, R>(&self, left: L, right: R) -> Either<L, R> {
        if *self {
            Left(left)
        } else {
            Right(right)
        }
    }

    fn either_else<L, FL: FnOnce() -> L, R, FR: FnOnce() -> R>(
        &self,
        l_func: FL,
        r_func: FR,
    ) -> Either<L, R> {
        if *self {
            Left(l_func())
        } else {
            Right(r_func())
        }
    }
}
