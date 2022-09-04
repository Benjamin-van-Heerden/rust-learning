use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait AnotherTrait {
    fn this() -> String {
        String::from("that")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl AnotherTrait for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn summarize_stuff() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.default_summary());
}

// traits as params
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// this is known as trait bound syntax (chain multiple trait bounds with +)
// better_notify is then a function that accepts any input that extends both the Summary and AnotherTrait traits.
pub fn better_notify<T: Summary + AnotherTrait>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// function with many trait bounds can becom messy - rust has us covered with the where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    5
}
// the alternative looks like this
// fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {5}
// with the where clause it is more concise

// we can even have return values that implement some trait
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    } // can return a tweet or a newsarticle !!but not either!! - if we return a tweet at one place, the rest of the funtion
      // needs to also return tweets

    // Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, as you probably already know, people",
    //     ),
    //     reply: false,
    //     retweet: false,
    // }
}
// this is powerful, this being the ability to specify the return type by the trait it implements
