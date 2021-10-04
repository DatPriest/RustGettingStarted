fn main() {
    println!("Hello, world!");
    let x = Data::new("thatskey", "thatsvalue");
    println!("{} {}", x.key, x.value);
    println!(" {}", x.summarize());
    println!(" {}", x.kind());
    range()
}

#[derive(Debug)]
enum Numbers {
    Uneven(i128),
    Even(i128),
}

impl Numbers {
    fn new(v: i128) -> Self {
        if v % 2 == 0 {
            Self::Even(v)
        } else {
            Self::Uneven(v)
        }
    }

    /// Returns `true` if the numbers is [`Uneven`].
    ///
    /// [`Uneven`]: Numbers::Uneven
    fn is_uneven(&self) -> bool {
        matches!(self, Self::Uneven(..))
    }

    /// Returns `true` if the numbers is [`Even`].
    ///
    /// [`Even`]: Numbers::Even
    fn is_even(&self) -> bool {
        matches!(self, Self::Even(..))
    }

    fn into_i128(self) -> i128 {
        match self {
            Numbers::Uneven(v) => v,
            Numbers::Even(v) => v,
        }
    }
}

fn range() {
    let range = 100000;

    let iter = (0..range).into_iter();

    let result = iter
        .filter(|x| x % 3 == 0)
        .map(|x| if x % 5 == 0 { x * x } else { x })
        .map(Numbers::new)
        .fold((0, 0), |(even, uneven), elem| match elem {
            Numbers::Uneven(x) => (even + x, uneven),
            Numbers::Even(y) => (even, uneven - y),
        });

    println!("{:?}", result);
}

struct Data {
    key: String,
    value: String,
}

impl Data {
    fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl X for Data {
    fn summarize(&self) -> String {
        format!("Key: {}\nValue: {}", self.key, self.value).
    }
}

impl CanBeVariable for Data {
    fn kind(&self) -> String {
        String::from("Data")
    }
}

pub trait CanBeVariable {
    fn kind(&self) -> String;
}

pub trait X {
    fn summarize(&self) -> String;
}
// 030586020909
