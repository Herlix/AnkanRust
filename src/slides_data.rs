#[derive(Debug, PartialEq)]
pub struct Slide<'a> {
    pub slug: &'a str,
    pub tags: &'a [&'a str],
    pub title: &'a str,
    pub url: &'a str,
}

pub const SLIDES: &[Slide<'static>] = &[
    Slide {
        slug: "FirstAndForMost",
        title: "First",
        tags: &["Fist", "Post"],
        url: "/api/slide/page_0.md",
    },
    Slide {
        slug: "SecondAndMiddleMost",
        title: "Second",
        tags: &["Second", "Post"],
        url: "/api/slide/page_1.md",
    },
];
