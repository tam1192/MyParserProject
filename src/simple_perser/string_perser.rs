type Output<INPUT, OUTPUT> = Option<(INPUT, OUTPUT)>;
trait Parser<INPUT, OUTPUT>: Fn(INPUT) -> Output<INPUT, OUTPUT> {}
