pub enum Expression<'a> {
    Integer(i32),
    Float(f32),
    Symbol(&'a str),
}
