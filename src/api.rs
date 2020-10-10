/// The main API with credentials and such
#[derive(Default, Clone)]
pub struct APIClient;

/// Some value that the endpoint will return
#[derive(Default)]
pub struct Value;
type ReturnedValue = Result<Value, Box<dyn std::error::Error>>;

/// The API requires a client for authentication reasons and etc, which
/// contains all the different endpoints. This example shows how the same
/// endpoint can be implemented in different ways.
///
/// See the main function for what's important: how the user has access to
/// the endpoint. The rest of the code has to do with the inner implementation
/// of some of the endoints.
impl APIClient {
    /// The actual code for the endpoint, to avoid repetition later on
    pub(super) fn actual_endpoint(
        &self,
        name: &str,
        opt1: Option<u32>,
        opt2: Option<i32>,
    ) -> ReturnedValue {
        println!("params: {} {:?} {:?}", name, opt1, opt2);

        Ok(Default::default())
    }

    pub fn approach_a(&self, name: &str, opt1: Option<u32>, opt2: Option<i32>) -> ReturnedValue {
        self.actual_endpoint(name, opt1, opt2)
    }

    pub fn approach_b<T1, T2>(&self, name: &str, opt1: T1, opt2: T2) -> ReturnedValue
    where
        T1: Into<Option<u32>>,
        T2: Into<Option<i32>>,
    {
        self.actual_endpoint(name, opt1.into(), opt2.into())
    }

    pub fn approach_c(&self, data: &params::ApproachC) -> ReturnedValue {
        self.actual_endpoint(&data.name, data.opt1, data.opt2)
    }

    pub fn approach_d(&self, data: &params::ApproachD) -> ReturnedValue {
        self.actual_endpoint(&data.name, data.opt1, data.opt2)
    }

    pub fn approach_f(&self, name: &str) -> ApproachFBuilder {
        ApproachFBuilder::default().client(self).name(name)
    }

    pub fn group(&self) -> GroupBuilder {
        GroupBuilder::default().client(self)
    }
}

pub mod params {
    #[derive(Default)]
    pub struct ApproachC {
        pub name: String,
        pub opt1: Option<u32>,
        pub opt2: Option<i32>,
    }

    #[derive(Default, Builder)]
    pub struct ApproachD {
        #[builder(setter(into))]
        pub name: String,
        #[builder(setter(strip_option), default)]
        pub opt1: Option<u32>,
        #[builder(setter(strip_option), default)]
        pub opt2: Option<i32>,
    }
}

#[derive(Default, Builder)]
#[builder(build_fn(private))]
pub struct ApproachE {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(strip_option), default)]
    pub opt1: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub opt2: Option<i32>,
}

impl ApproachEBuilder {
    pub fn call(&self, client: &APIClient) -> ReturnedValue {
        let data = self.build()?; // This might fail!
        client.actual_endpoint(&data.name, data.opt1, data.opt2)
    }
}

#[derive(Default, Builder)]
#[builder(build_fn(private), pattern = "owned")]
pub struct ApproachF<'a> {
    #[builder(setter(strip_option), default)]
    client: Option<&'a APIClient>,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(strip_option), default)]
    pub opt1: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub opt2: Option<i32>,
}

impl ApproachFBuilder<'_> {
    pub fn call(self) -> ReturnedValue {
        let data = self.build()?; // This should never fail
        data.client
            .unwrap()
            .actual_endpoint(&data.name, data.opt1, data.opt2)
    }
}

#[derive(Default, Builder)]
#[builder(build_fn(private), pattern = "owned")]
pub struct Group<'a> {
    #[builder(setter(strip_option), default)]
    client: Option<&'a APIClient>,
    #[builder(setter(strip_option), default)]
    pub opt1: Option<u32>,
    #[builder(setter(strip_option), default)]
    pub opt2: Option<i32>,
}

impl GroupBuilder<'_> {
    pub fn approach_g(self, name: &str) -> ReturnedValue {
        let data = self.build().unwrap(); // This should never fail
        data.client
            .unwrap()
            .actual_endpoint(name, data.opt1, data.opt2)
    }

    pub fn approach_h(self, name: &str) -> ReturnedValue {
        let data = self.build().unwrap(); // This should never fail

        // This endpoint doesn't use `opt1`. It can either be ignored, or
        // be an error.
        if data.opt1.is_some() {
            panic!("opt1 isn't needed for enpoint_h")
        }

        data.client.unwrap().actual_endpoint(name, None, data.opt2)
    }
}
