pub trait AddDoubleQuoutes {
    fn add_double_quotes(self) -> Self;
}

impl AddDoubleQuoutes for String {
    fn add_double_quotes(self) -> Self {
        format!("\"{self}\"")
    }
}
pub trait Append
where Self: Sized
{
    fn append(self, append: impl Into<Self> + Sized) -> Self;
}

impl Append for String {
    fn append(self, append: impl Into<Self> + Sized) -> Self {
        let append: String = append.into();
        format!("{self}{append}")
    }
}