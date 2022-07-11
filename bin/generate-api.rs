use gear_program::builder::Pre;

fn main() {
    let ext = Pre::default();

    if let Err((expected, _)) = ext.check_spec_version() {
        // ext.build_gear();
        ext.generate_gear_api(expected).unwrap();
    }
}
