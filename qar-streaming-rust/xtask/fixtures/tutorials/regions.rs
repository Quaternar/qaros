// Fixture source for the tutorial generator tests: two anchored regions.

fn outside_any_region() {}

mod fixture_module {
    // ANCHOR: indented_region
    fn first_fixture_function() {
        let fixture_value = 1;

        let _ = fixture_value;
    }
    // ANCHOR_END: indented_region
}

// ANCHOR: top_level_region
fn second_fixture_function() {}
// ANCHOR_END: top_level_region
