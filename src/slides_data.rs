#[derive(Clone, PartialEq)]
pub struct Code<'a> {
    pub value: &'a str,
    pub message: Option<&'a str>,
}

#[derive(Clone, PartialEq)]
pub struct Slide<'a> {
    pub images: &'a [(&'a str, &'a str)],
    pub code: Option<Code<'a>>,
    pub text: Option<&'a str>,
    pub title: &'a str,
    pub slug: &'a str,
}

pub const SLIDES: &'static [Slide<'static>] = &[
    Slide {
        images: &[
            ("/images/ferris_original.svg", "Ferris"),
            ("/images/logo_rust.svg", "Rust logo"),
        ],
        code: None,
        text: Some("One shall understand at the end of this talk"),
        title: "Welcome",
        slug: "FirstAndForMost",
    },
    Slide {
        images: &[("/images/ferris_original.svg", "Ferris")],
        code: None,
        text: None,
        title: "Who is using rust?",
        slug: "UsingRust",
    },
];
