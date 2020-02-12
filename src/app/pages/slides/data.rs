use log::info;
use stdweb::js;
use stdweb::web::Node;
use yew::virtual_dom::VNode;
use yew::{html, Html};

const AMOUNT: usize = 30;

pub struct PresentationData {
    data: [Html; AMOUNT],
}

impl PresentationData {
    pub fn new() -> Self {
        PresentationData {
            data: populated_data(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_content(&self, index: &usize) -> Html {
        if let Some(val) = self.data.get(*index) {
            val.clone()
        } else {
            html! { <p>{"Something went wrong"}</p> }
        }
    }
}

fn colorize(code: &str) -> Html {
    info!("Raw: {}", code);
    let raw_html = (js! {
        return Prism.highlight(@{code}, Prism.languages.rust, "rust");
    })
    .into_string()
    .unwrap();
    info!("raw_html: {}", raw_html);
    let node = Node::from_html(&format!(
        "<div class=\"rust-code\"><pre><code class=\"language-rust\">{}</code></pre></div>",
        raw_html
    ))
    .unwrap();
    VNode::VRef(node)
}

fn populated_data() -> [Html; AMOUNT] {
    [
        html! {
            <div>
                <img src="/images/ferris_original.svg" alt="Ferris" class="ferris-main"/>
                <img src="/images/logo_rust.svg" alt="Rust logo" />
            </div>
        },
        html! {
            <div>
                <a target="_blank" href="https://www.rust-lang.org/">
                    <h1> { "The Rust Website "}  </h1>
                </a>
                <img src="/images/meme.png" alt="Meme" />
            </div>
        },
        html! {
            <div>
                <a target="_blank" href="https://duckduckgo.com/?t=ffab&q=rust+lang&atb=v162-1&ia=about">
                    <h1> { "A quick search "}  </h1>
                </a>
            </div>
        },
        html! {
            <div>
                <h1>{ "Understanding Ownership" }</h1>
                <p> {"Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it’s important to understand how ownership works in Rust. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory."}</p>
            </div>
        },
        html! {
            <div>
                <img src="/images/code_stack_and_heap.svg" alt="Stack and heap painting" class="stack-and-heap"/>
            </div>
        },
        html! {
            <div>
                <img src="/images/code_give_book.svg" alt="Stick man give book" class="stick-man"/>
                { colorize("\nfn gives_book() {\n    let book = String::from(\"Carls scribbles\");\n    do_something(book);\n}\n\nfn do_something(book: String) {\n    ...\n}\n " )}
                {colorize("")}
            </div>
        },
        html! {
            <div>
                <img src="/images/code_give_book2.svg" alt="Stick man borrow book" class="stick-man"/>
                { colorize("\nfn gives_book() {\n    let book = String::from(\"Carls scribbles\");\n    do_something(book);\n    println!(\"Let's read! {}\", book);\n}\n\nfn do_something(book: String) {\n    ...\n}\n " )}
                <div class="error-wrapper">
                    <p>{ "Borrow of moved value: `book` E0382"}</p>
                    <img src="/images/ferris_panics.svg" alt="Ferris panics" class="ferris-panic"/>
                </div>
            </div>
        },
        html! {
            <div>
                <img src="/images/code_two_books.svg" alt="Stick man clone book" class="stick-man"/>
                { colorize("\nfn gives_book() {\n    let book = String::from(\"Carls scribbles\");\n    do_something(book.Clone());\n    println!(\"Let's read! {}\", book);\n}\n " )}
            </div>
        },
        html! {
           <div>
                <img src="/images/code_read_book.svg" alt="Stick man read book" class="stick-man"/>
                { colorize("\nfn gives_book() {\n    let book = String::from(\"Carls scribbles\");\n    do_something(&book);\n    println!(\"Let's read! {}\", book);\n}\n\nfn do_something(book: &String) {\n    ...\n}\n " )}
           </div>
        },
        html! {
           <div>
                <img src="/images/code_edit_book.svg" alt="Stick man edit book" class="stick-man"/>
                { colorize("\nfn gives_book() {\n    let mut book = String::from(\"Carls scribbles\");\n    do_something(&mut book);\n}\n\nfn do_something(book: &mut String) {\n    book.push_str(\"The smurfs are actually evil\");\n}\n " )}
           </div>
        },
        html! {
           <div>
                <img src="/images/code_multiple_readers.svg" alt="stick man multiple readers" class="stick-man"/>
                <ul>
                    <li>{ "Box<T> for allocating values on the heap" } </li>
                    <li>{ "Rc<T>, a reference counting type that enables multiple ownership" } </li>
                    <li>{ "Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time" } </li>
                </ul>
           </div>
        },
        html! {
            <div>
                <img src="/images/code_two_books.svg" alt="Stick man two books" class="stick-man"/>
                { colorize("\nfn book_length() {\n    let book1 = String::from(\"Carls scribbles\");\n    let book2 = String::from(\"Carls scribbles longer\");\n    let longest = longest(&book1, &book2);\n    println!(\"something something longest book {longest}\");\n}\n\nfn longest(book1: &String, book2: &String) -> &String {\n    if book1.len() > book2.len() {\n        book1\n    } else {\n        book2\n    }\n}\n ")}
                <div class="error-wrapper">
                    <p>{ "Missing lifetime specifier E0106" }</p>
                    <p>{ "Expected lifetime parameter" }</p>
                    <img src="/images/ferris_panics.svg" alt="Ferris panics" class="ferris-panic"/>
                </div>
            </div>
        },
        html! {
           <div>
                { colorize("\nfn book_length() {\n    let book1 = String::from(\"Carls scribbles\");\n    let book2 = String::from(\"Carls scribbles longer\");\n    let longest = longest(&book1, &book2);\n    println!(\"something something longest book {longest}\");\n}\n\nfn longest<'a>(book1: &'a String, book2: &'a String) -> &'a String {\n    if book1.len() > book2.len() {\n        book1\n    } else {\n        book2\n    }\n }\n ")}
           </div>
        },
        html! {
           <div>
                { colorize("\nfn book_length() {\n    let book1 = String::from(\"Carls scribbles\");\n    let longest = {\n        let book2 = String::from(\"Carls scribbles longer\");\n        longest(&book1, &book2);\n    }\n    println!(\"something something longest book {longest}\");\n}\n ")}
                <div class="error-wrapper">
                    <p>{ "book2 does not live long enough E0597" }</p>
                    <img src="/images/ferris_panics.svg" alt="Ferris panics" class="ferris-panic"/>
                </div>
           </div>
        },
        html! {
            <div>
                    <h1> { "Do not communicate by sharing memory; instead, share memory by communicating.​"}  </h1>
                    <h1> { "instead, share memory by communicating.​"}  </h1>
            </div>
        },
        html! {
            <div>
                { colorize("\nfn simple_threads() {\n    let my_string = \"My nice string literal!\"\n    thread::spawn(move || {\n        for i in 0..10 {\n            println!(\"Second thread, print nr {}\", i);\n            thread::sleep(Duration::from_millis(1));\n                println!(\"Str: {}\", my_string);\n        }\n\n        println!(\"Str: {}\", my_string);\n        for 1 in 0..5 {\n            println!(\"Main thread, print nr {}\",i);\n            thread::sleep(Duration::from_millis(1));\n        }\n    });\n}\n ")}
                <ul class="result-list">
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Main thread, print nr 0​" } </li>
                    <li>{ "Second thread, print nr 0​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 1​" } </li>
                    <li>{ "Main thread, print nr 1​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 2​" } </li>
                    <li>{ "Main thread, print nr 2​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 3​" } </li>
                    <li>{ "Main thread, print nr 3​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 4​" } </li>
                    <li>{ "Main thread, print nr 4​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 5​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 6​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 7​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 8​" } </li>
                    <li>{ "Str: My nice string literal!​" } </li>
                    <li>{ "Second thread, print nr 9​" } </li>
                </ul>
            </div>
        },
        html! {
            <div>
                { colorize("\nfn multi_producer_single_consumer() {\n    let (transmitter, receiver) = mpsc::channel();\n thread::spawn(move || {\n let values = vec![0,1,2,3,4,5];\n thread::sleep(Duration::from_millis(10));\n transmitter.send(values).expect(\"Could not transmit values\");\n });\n\n let values = receiver.recv().expect(\"Could not receive values\");\n for x in values {\n println!(\"Value: {}\", x);\n }\n }\n ")}
                <div>
                    <p>{ "Value: 0​" }</p>
                    <p>{ "Value: 1​" }</p>
                    <p>{ "Value: 2​" }</p>
                    <p>{ "Value: 3​" }</p>
                    <p>{ "Value: 4​" }</p>
                    <p>{ "Value: 5​" }</p>
                </div>
            </div>
        },
        html! {
            <div>
                { colorize("\nfn share_memory() {\n    let counter = Arc::new(Mutex::new(0));\n    let mut threads = Vec::new();\n    for _ in 0..10 {\n        let inner_counter = Arc::clone(&counter);\n        let thread = thread::spawn(move || {\n            let mut num = inner_counter.lock().unwrap();\n            *num +=1;\n        });\n        threads.push(thread);\n    }\n\n    threads.into_inter().for_each(|x| x.join().unwrap());\n    println!(\"Total value: {}\", *counter.lock().unwrap());\n }\n ")}
                <div>
                    <p>{ "Total value: 10​" }</p>
                </div>
            </div>
        },
        html! {
            <div>
                <h1>{ "Rust is strict" }</h1>
                <h2>{ "But if it compiles, you can be sure it won't crash from a memory issue" }</h2>
            </div>
        },
        html! {
            <div>
                <h1>{ "No exceptions & no null" }</h1>
                <p>{ "(no not undefined either)" }</p>
                { colorize("\nenum Result<T, E> {\n    Ok(T),\n    Err(E),\n }\n\nenum Option<T> {\n    Some(T),\n    None,\n }\n ")}
            </div>
        },
        html! {
            <div>
                { colorize("\nfn use_option(arr: Vec<usize>) {\n    let exists = arr.get(1); // <- Some(5)\n    let nope = arr.get(98); // <- None\n    match(exists) {\n        Some(val) => println!(\"The value is: {}\", val),\n    }\n}\n\nfn use_result() {\n    let file = File::open(\"...SomePath\"); // <-- Err(std::io::error::Error)\n    if let Ok(val) = file {\n        println!(\"Content: {}\", val)\n    } else {\n        println!(\"The file does not exist, if let Ok discarded the error, to bad\")\n    }\n}\n ")}
                <div class="error-wrapper">
                    <p>{ "Match must be exhaustive [E0004]"}</p>
                </div>
            </div>
        },
        html! {
            <div>
                <h1>{ "Rust has a lot of syntax sugar, and it helps!" }</h1>
                <p>{ "Dealing with all cases can be tedious, use ? to pass errors upwards" }</p>
                { colorize("\nfn use_result() -> Result<File, Error> {\n    let file = File::open(\"...SomePath\")? // <-- Err(std::io::error::Error)\n}\n ")}
                <ul>
                <li>{"*"}</li>
                <li>{"?"}</li>
                <li>{"(No return)"}</li>
                <li>{"if let .. ="}</li>
                <li>{"Infers types"}</li>
                <li>{"macros"}</li>
                <li>{"Awesome match capabilities!"}</li>
                <li>{"..."}</li>
                </ul>
            </div>
        },
        html! {
            <div>
                { colorize("\nfn patterns_loops(arr: Vec<usize>) -> Result<String, ()> {\n    let mut results = Vec::new();\n    for v in arr {\n        if let Some(x) = v {\n            let r = match x {\n                0 => \"No\",\n                1 | 2 => \"1 or 2\",\n                _ if(*y % 2 == 0) => \"Even value\",\n                _ => \"Nope, no match found\"\n            };\n            results.push(r);\n        }\n    }\n    var res = results\n        .iter()\n        .filter(|x| !x.contains(\"no\"))\n        .join();\n    Ok(res)\n}\n ")}
            </div>
        },
        html! {
            <div>
            <h1> { "Traits, mods, structs" }</h1>
            <p> { "Interface, namespace, class" }</p>
                { colorize("\nfn main() {\n    let animals: Vec<Box<dyn Animal>> = vec![\n        Box::new(Chicken),\n        Box::new(Cow {color: \"blue\".to_owned()})\n    ];\n    animals.iter().for_each(|x| x.do_your_thing());\n}\n\nmod namespace {\n    pub trait Animal {\n        fn do_your_thing(&self);\n    }\n\n    pub struct Chicken;\n\n    pub struct Cow {\n        pub color: String,\n    }\n\n    impl Animal for Chicken{\n        fn do_your_thing(&self) {\n            println!(\"cluck cluck cluck\");\n        }\n    }\n\n    impl Animal for Cow {\n        fn do_your_thing(&self) {\n            println!(\"the {} cow says Moo\", self.color);\n        }\n    }\n}\n ")}
            </div>
        },
        html! {
            <div>
                <h1> { "Async programming is here" }</h1>
                <p> { "And with (almost) no penalty in performance!" }</p>
                { colorize("\nasync fn hello() -> Result<String> {\n    Ok(...)\n}\n\n#[tokio::main]\nfn main() -> Result<()>{\n    if ler Ok(val) = hello().await {\n        ...\n    }\n    Ok(())\n}\n ")}
            </div>
        },
        html! {
            <div>
                <h1> { "You've seen what Rust code can look like" }</h1>
                <h2> { "You've seen good and bad practices" }</h2>
                <h3> { "Let's talk Option::none code" }</h3>
            </div>
        },
        html! {
            <div>
                <h1> {"Cargo"} </h1>
                <h2> {"your everything tool, like npm"} </h2>
                <p> {"Cargo can do so much cool stuff"} </p>
                <ul>
                    <li>{ "build" }</li>
                    <li>{ "test (run tests IN your documentation)" }</li>
                    <li>{ "generate" }
                        <a href={"https://docs.rs/yew/0.11.0/yew/"}> {"documentation"} </a>
                    </li>
                </ul>
            </div>
        },
        html! {
            <div>
                <a target="_blank" href="https://www.rust-lang.org/governance">
                    <h1> { "Meet the maker"}  </h1>
                </a>
            </div>
        },
        html! {
            <div>
                <a target="_blank" href="https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted">
                    <h1> { "Stackoverflow 2019"}  </h1>
                </a>
                 <a target="_blank" href="https://insights.stackoverflow.com/survey/2018#most-loved-dreaded-and-wanted">
                    <h1> { "Stackoverflow 2018"}  </h1>
                </a>
                <a target="_blank" href="https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted">
                    <h1> { "Stackoverflow 2017"}  </h1>
                </a>
            </div>
        },
        html! {
            <div>
                <img src="/images/logo_vscode.png" alt="VsCode" class="small-logo"/>
                <img src="/images/logo_npm.png" alt="Npm" class="small-logo" />
                <img src="/images/logo_cloudflare.svg" alt="CloudFlare" class="big-logo" />
                <img src="/images/logo_mozilla.png" alt="Mozilla" class="big-logo" />
                <img src="/images/logo_yelp.png" alt="Yelp" class="small-logo" />
                <img src="/images/logo_microsoft.png" alt="Microsoft" class="small-logo" />
                <img src="/images/logo_dropbox.svg" alt="Dropbox" class="small-logo"/>
                <img src="/images/logo_google.png" alt="Google" class="small-logo" />
                <img src="/images/logo_discord.png" alt="Discord" class="big-logo" />

            </div>
        },
    ]
}
