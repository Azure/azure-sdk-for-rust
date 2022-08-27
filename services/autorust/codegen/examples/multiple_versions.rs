// cargo run --example multiple_versions
// report tags that use multiple versions
// in general, we want to avoid this
// https://github.com/Azure/azure-sdk-for-rust/issues/563

use autorust_codegen::{get_mgmt_readmes, get_svc_readmes, io, Result, Spec, SpecReadme};
use autorust_openapi::StatusCode;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    println!("CONTROL PLANE");
    check(&get_mgmt_readmes()?)?;
    println!();
    println!("DATA PLANE");
    check(&get_svc_readmes()?)?;
    Ok(())
}

fn check(readmes: &[SpecReadme]) -> Result<()> {
    // let mut services = BTreeSet::new();
    let mut tags = 0;
    for readme in readmes {
        let readme_path = readme.readme();
        for tag in readme.config()?.tags() {
            let input_files = io::join_several(readme_path, &tag.input_files())?;
            match Spec::read_files(&input_files) {
                Ok(spec) => {
                    for operation in spec.operations()? {
                        let responses = autorust_codegen::status_codes::get_success_responses(&operation.responses);
                        // if respones.len() > 1 {
                        //     // let mut codes: Vec<_> = respones.into_iter().map(|(sc, rsp)| sc).collect();
                        //     // codes.sort();

                        //     // if codes
                        //     //     == vec![
                        //     //         StatusCode::Code(200),
                        //     //         StatusCode::Code(201),
                        //     //         StatusCode::Code(202),
                        //     //         StatusCode::Code(204),
                        //     //     ]
                        //     // {
                        //     //     println!("{} {} {:?} {:?}", readme.spec(), &tag.name(), operation.id, codes);
                        //     // }

                        //     services.insert(codes);
                        // }

                        for (sc, rsp) in responses.iter() {
                            for hdr in &rsp.headers {
                                match hdr.1 {
                                    autorust_openapi::ReferenceOr::Reference {
                                        reference,
                                        title,
                                        description,
                                        type_,
                                        read_only,
                                        x_ms_client_flatten,
                                        xml,
                                        x_nullable,
                                    } => {
                                        println!("{} {} {:?} using header reference", readme.spec(), &tag.name(), operation.id);
                                    },
                                    autorust_openapi::ReferenceOr::Item(item) => {
                                        if Some(true) == item.required {
                                            println!("{} {} {:?}", readme.spec(), &tag.name(), operation.id);
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    // let versions = spec.api_versions();
                    // if versions.len() > 1 {
                    //     println!("{} {}", readme.spec(), &tag.name());
                    //     for version in versions {
                    //         println!("  {}", version);
                    //     }
                    //     tags += 1;
                    //     services.insert(readme.spec());
                    // }
                }
                // Err(err) => println!("Error {}", err),
                Err(_err) => {}
            }
        }
    }
    // println!();
    // println!("{} tags", tags);
    // println!("{} services:", services.len());
    // for service in services {
    //     println!("  {:?}", service);
    // }
    Ok(())
}
