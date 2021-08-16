use rweb::*;
use serde::Serialize;

pub mod without_components {
    use super::*;

    #[derive(Debug, Serialize, Schema)]
    pub struct Bar {
        pub foo: Box<Foo>,
    }

    #[derive(Debug, Serialize, Schema)]
    pub struct Foo {
        pub bar: Option<Box<Bar>>,
    }

    #[get("/")]
    pub fn index() -> Json<Foo> {
        Json::from(Foo {
            bar: Some(Box::new(Bar {
                foo: Box::new(Foo { bar: None }),
            })),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should error instead.
        #[test]
        fn this_stackoverflows() {
            rweb::openapi::spec().build(|| index());
        }
    }
}

pub mod with_components {
    use super::*;

    #[derive(Debug, Serialize, Schema)]
    #[schema(component = "Bar")]
    pub struct Bar {
        pub foo: Box<Foo>,
    }

    #[derive(Debug, Serialize, Schema)]
    #[schema(component = "Foo")]
    pub struct Foo {
        pub bar: Option<Box<Bar>>,
    }

    #[get("/")]
    pub fn index() -> Json<Foo> {
        Json::from(Foo {
            bar: Some(Box::new(Bar {
                foo: Box::new(Foo { bar: None }),
            })),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should be allowed.
        #[test]
        fn this_stackoverflows() {
            rweb::openapi::spec().build(|| index());
        }
    }
}
