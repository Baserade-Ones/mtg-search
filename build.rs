use archidekt::*;
use strum::VariantArray;

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_ARCH");
    if matches!(target.as_ref().map(|x| &**x), Ok("wasm32")) {
        for user in User::VARIANTS {
            let data = get_collections(user).expect(&format!("Error fetching user: {user}"));
            std::fs::write(format!("assets/{user}.csv"), data)
                .expect(&format!("Error saving user: {user}"));
        }
    }
}
