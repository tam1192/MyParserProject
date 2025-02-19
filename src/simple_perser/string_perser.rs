type Output<T, A> = Option<(T, A)>;
trait Parser<T, A>: Fn(A) -> Output<T, A> {}
