pub enum StringOrRef<'a> {
    Owned(String),
    Borrowed(&'a str),
}

impl StringOrRef<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            StringOrRef::Owned(s) => &s,
            StringOrRef::Borrowed(s) => s,
        }
    }
}

impl Drop for StringOrRef<'_> {
    fn drop(&mut self) {
        print!(
            "Destroying the StringOrRef containing {} which is {}: ",
            self.as_str(),
            variant(self),
        );
        if matches!(self, StringOrRef::Owned(_)) {
            println!("memory on the heap will be freed");
        } else {
            println!("no memory on the heap will be freed");
        }
    }
}

pub fn to_lowercase(s: &str) -> StringOrRef<'_> {
    if s.chars().all(|c| c.is_lowercase()) {
        StringOrRef::Borrowed(s)
    } else {
        StringOrRef::Owned(s.to_lowercase())
    }
}

fn variant(x: &StringOrRef<'_>) -> &'static str {
    if matches!(x, StringOrRef::Owned(_)) {
        "owned"
    } else {
        "borrowed"
    }
}

fn main() {
    let s1 = to_lowercase("HeLlO");
    let s2 = to_lowercase("world");
    println!("s1 = {}, s2 = {}", s1.as_str(), s2.as_str());
    println!("s1 is {}, s2 is {}", variant(&s1), variant(&s2));
}
