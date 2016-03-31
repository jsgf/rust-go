pub trait Accum<T> {
    /// Apply each element of `Self` to `state` with a function
    ///
    /// Accumulates values from `self` into `state` with `func` until
    /// it stops producing values. Typically used with iterators and
    /// iterable types.
    fn accum<F, S>(self, state: S, func: F) -> S
        where F: Fn(&mut S, T);
}

impl<T, I> Accum<T> for I
    where I: IntoIterator<Item=T>
{
    fn accum<F, S>(self, state: S, func: F) -> S
        where F: Fn(&mut S, T)
    {
        let mut state = state;
        for v in self {
            func(&mut state, v)
        }
        state
    }
}

#[cfg(test)]
mod tests {
    use super::Accum;

    #[test] fn simple() {
        let seq = vec![1,2,3,4];
        assert_eq!(10, seq.accum(0, |mut s, v| *s += v));
    }
}
