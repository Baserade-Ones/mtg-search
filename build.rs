use archidekt::*;
use strum::VariantArray;

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_ARCH");
    if matches!(target.as_ref().map(|x| &**x), Ok("wasm32")) {
        for user in User::VARIANTS {
            let data = get_collections(&user).expect(&format!("Error fetching user: {user}"));

            let mut wrt = csv::Writer::from_path(format!("assets/{user}.csv"))
                .expect(&format!("Error creating writer for {user}"));
            for entry in data {
                eprintln!("{entry:?}");
                wrt.serialize(entry).unwrap();
            }
            wrt.flush()
                .expect(&format!("Error flushing writer for {user}"));
        }
    }
}
