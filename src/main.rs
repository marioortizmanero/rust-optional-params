mod api;

#[macro_use]
extern crate derive_builder;

use crate::api::{params, APIClient, ApproachEBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = APIClient {};
    let param2: u32 = 324;

    // A) Using `Option<T>`
    api.approach_a("option", Some(param2), Some(1234))?;
    api.approach_a("option", None, None)?;

    // B) With `Into<Option<T>>`
    api.approach_b("into_option", param2, 1234)?;
    // This still works
    api.approach_b("into_option", Some(param2), Some(123))?;
    api.approach_b("into_option", None, None)?;

    // C) With a custom struct
    let call1 = params::ApproachC {
        name: "builder".to_string(),
        opt1: Some(param2),
        opt2: Some(123),
    };
    api.approach_c(&call1)?;
    let call2 = params::ApproachC {
        name: "builder".to_string(),
        ..Default::default()
    };
    api.approach_c(&call2)?;

    // D) With the builder pattern
    let call1 = params::ApproachDBuilder::default()
        .name("builder")
        .opt1(param2)
        .opt2(2134)
        .build()?;
    api.approach_d(&call1)?;
    let call2 = params::ApproachDBuilder::default()
        .name("builder")
        .build()?;
    api.approach_d(&call2)?;

    // E) Endpoint-oriented interface with the builder pattern
    ApproachEBuilder::default()
        .name("endpoint-oriented")
        .opt1(param2)
        .opt2(1111)
        .call(&api)?;
    ApproachEBuilder::default()
        .name("endpoint-oriented")
        .call(&api)?;

    // F) Hybrid builder pattern
    api.approach_f("hybrid-builder-pattern")
        .opt1(param2)
        .opt2(2222)
        .call()?;

    // G,H) Endpoints grouped up with the builder pattern
    api.group()
        .opt1(param2)
        .opt2(2222)
        .approach_g("group-builder-pattern")?;
    api.group().opt2(2222).approach_h("group-builder-pattern")?;

    Ok(())
}
