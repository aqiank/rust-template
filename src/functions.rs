use std::collections::BTreeMap;

pub type Func = fn(&[&str]) -> Option<String>;

// Initialize built-in functions
lazy_static! {
    pub static ref funcs: BTreeMap<&'static str, Func> = {
        let mut m: BTreeMap<&'static str, Func> = BTreeMap::new();
        m.insert(&"foo", foo);
        m
    };
}

fn foo(args: &[&str]) -> Option<String> {
    let mut output = String::new();

    for s in args {
        output.push_str(&format!("foo{}", s));
    }

    Some(output)
}
