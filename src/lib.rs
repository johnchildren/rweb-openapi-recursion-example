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

    #[derive(Debug, Serialize, Schema)]
    pub struct Baz {
        pub baz: Option<Box<Baz>>,
    }

    #[get("/")]
    pub fn mutual_recursion() -> Json<Foo> {
        Json::from(Foo {
            bar: Some(Box::new(Bar {
                foo: Box::new(Foo { bar: None }),
            })),
        })
    }

    #[get("/")]
    pub fn recursion() -> Json<Baz> {
        Json::from(Baz {
            baz: Some(Box::new(Baz { baz: None })),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should error instead.
        #[test]
        fn mutual_recursion_stackoverflows() {
            rweb::openapi::spec().build(|| mutual_recursion());
        }

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should error instead.
        #[test]
        fn recursion_stackoverflows() {
            rweb::openapi::spec().build(|| recursion());
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

    #[derive(Debug, Serialize, Schema)]
    #[schema(component = "Baz")]
    pub struct Baz {
        pub baz: Option<Box<Baz>>,
    }

    #[get("/")]
    pub fn mutual_recursion() -> Json<Foo> {
        Json::from(Foo {
            bar: Some(Box::new(Bar {
                foo: Box::new(Foo { bar: None }),
            })),
        })
    }

    #[get("/")]
    pub fn recursion() -> Json<Baz> {
        Json::from(Baz {
            baz: Some(Box::new(Baz { baz: None })),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should be allowed.
        #[test]
        fn mutual_recursion_stackoverflows() {
            rweb::openapi::spec().build(|| mutual_recursion());
        }

        /// This test will stack overflow as the spec builder
        /// will recurse infinitely. Should be allowed.
        #[test]
        fn recursion_stackoverflows() {
            rweb::openapi::spec().build(|| recursion());
        }
    }
}
