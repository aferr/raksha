#[cfg(test)]
mod test {
    use std::fs;
    use crate::ast::*;
    use crate::compilation_top_level::*;
    use crate::test::test_queries::test::QueryTest;
    use crate::souffle::souffle_interface::*;
    // this is used for generating keypairs:
    use crate::signing::tink_interface::*;

    fn query_test_with_imports(t: QueryTest) {
        compile(&t.filename.to_string(), 
                &"test_inputs".to_string(),
                &"test_outputs".to_string());
        run_souffle(&format!("test_outputs/{}.dl", t.filename),
                    &"test_outputs".to_string());
        for (qname, intended_result) in t.query_expects {
            let queryfile = format!("test_outputs/{}.csv", qname);
            assert!(is_file_empty(&queryfile) != intended_result);
        }
    }

    #[test]
    fn test_signature_importing() {
        store_new_keypair_cleartext(
            &"test_keys/principal1e_pub.json".to_string(),
            &"test_keys/principal1e_priv.json".to_string());

        // generate exported statements from test_inputs/exporting
        compile(&"importing_export_half".to_string(),
            &"test_inputs".to_string(),
            &"test_outputs".to_string());

        // import statements into test_inputs/importing
        // and check queries for expected results
        query_test_with_imports(QueryTest {
            filename: "importing",
            query_expects: vec!(
                ("q1", true),
                ("q2", false),
                ("q3", true),
                ("q4", false)
            )
        })
    }
}

