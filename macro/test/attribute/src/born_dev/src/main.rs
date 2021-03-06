// https://github.com/rust-lang/rust/blob/master/src/libstd/macros.rs
// https://www.google.com/search?client=firefox-b-d&q=what+is+token+tree+in+rust
// https://doc.rust-lang.org/rust-by-example/trait/derive.html
// https://www.google.com/search?client=firefox-b-d&q=nested macro not allowed in rust
// https://www.google.com/search?client=firefox-b-d&q=struct inheritance vs composition rust

macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}

// Include public
// https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
// https://users.rust-lang.org/t/best-way-to-make-a-macro-with-required-and-optional-arguments/27514
// (You can use optional.)
// https://doc.rust-lang.org/reference/macros-by-example.html#metavariables

// https://doc.rust-lang.org/reference/macros-by-example.html
// vis - a possibly empty Visibility qualifier, You don't need a optional parameter ? with this.
// vis may only be followed by one of: , an identifier other than a non-raw priv, any token that can begin a type, or a metavariable with a ident, ty, or path fragment specifier.
macro_rules! public_struct {
    (pub struct $basestruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $basestruct {
                    () => {
                        pub struct $basestruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        pub struct $basestruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };

                    (pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };

                    (pub struct $name:ident) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}

// A struct with two fields
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    name: String,
    description: String,
    image: String,
}

public_struct!(
    // pub is required here before struct
    pub struct UserBase {
        pub id: i64,
        pub email: String,
        pub profile: Profile,
    }
);

UserBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub active: bool,
    }
);

// public by default. You should use pub before every struct keyord.
// pub before fields is optional.
public_struct!(
    // pub is required here before struct
    pub struct MessageBase {
        pub author_id: i64,
        pub text: String,
        // pub text: String // , here is not required?
    }
);

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        // read: bool, // pub is optional to the fields
        pub read: bool,
    }
);

impl Message {
    fn update_text(&mut self, new_message: String) {
        self.text = new_message
    }
    fn read(&mut self) {
        if self.read == false {
            self.read = true;
        }
    }
}

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct MessageCreateRequest
);

MessageBase!(
    // #[derive(Debug, Clone, PartialEq)]
    pub struct MessageUpdateRequest
);

fn main() {
    // public_struct!(
    //    pub struct Infunction {
    //        test: bool
    //    }
    // );

    let image = "https://avatars0.githubusercontent.com/u/32325099?s=460&u=cd848fc83d9739939a4ea2d38108c8bcee199109&v=4".into();

    let user = User {
        id: 1i64,
        email: "steady@learner.com".into(),
        profile: Profile {
            name: "Steadylearner".into(),
            description: "Rust programmer".into(),
            image,
        },
        active: true,
    };
    println!("{:#?}", user);

    let message_create_request = MessageCreateRequest {
        author_id: user.id,
        text: "First message.".into(),
    };

    let mut message = Message {
        author_id: message_create_request.author_id,
        text: message_create_request.text,
        read: false,
    };
    println!("{:#?}", &message);

    assert_eq!(message, message.clone());

    let message_update_request = MessageUpdateRequest {
        author_id: user.id,
        text: "Updated message.".into(),
    };

    message.update_text(message_update_request.text);
    println!("{:#?}", &message);

    message.read();
    println!("{:#?}", &message);
}
