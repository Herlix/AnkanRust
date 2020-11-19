#[derive(Clone, PartialEq)]
pub struct Code<'a> {
    pub value: &'a str,
    pub message: Option<&'a str>,
}

#[derive(Clone, PartialEq)]
pub struct Slide<'a> {
    pub images: &'a [&'a str],
    pub code: Option<Code<'a>>,
    pub text: Option<&'a str>,
    pub title: &'a str,
    pub slug: &'a str,
}

pub const SLIDES: &'static [Slide<'static>] = &[
    Slide {
        images: &["", ""],
        code: None,
        text: None,
        title: "[0] Welcome",
        slug: "FirstAndForMost",
    },
    Slide {
        images: &["", ""],
        code: None,
        text: None,
        title: "[1] Welcome",
        slug: "SecondAndForMost",
    },
];
