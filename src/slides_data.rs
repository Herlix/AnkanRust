#[derive(Clone, PartialEq)]
pub struct Code<'a> {
    pub value: &'a str,
    pub message: Option<&'a str>,
}

#[derive(Clone, PartialEq)]
pub struct Img<'a> {
    pub path: &'a str,
    pub alt: &'a str,
}
#[derive(Clone, PartialEq)]
pub struct Slide<'a> {
    pub images: &'a [Img<'a>],
    pub code: Option<Code<'a>>,
    pub text: Option<&'a str>,
    pub title: &'a str,
    pub slug: &'a str,
}

pub const SLIDES: &[Slide<'static>] = &[
    Slide {
        images: &[
            Img {
                path: "/images/ferris_original.svg",
                alt: "Ferris",
            },
            Img {
                path: "/images/logo_rust.svg",
                alt: "Rust logo",
            },
        ],
        code: None,
        text: Some("One shall understand at the end of this talk"),
        title: "Welcome",
        slug: "FirstAndForMost",
    },
    Slide {
        images: &[Img {
            path: "/images/ferris_original.svg",
            alt: "Ferris",
        }],
        code: None,
        text: None,
        title: "Who is using rust?",
        slug: "UsingRust",
    },
];
