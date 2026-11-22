use super::*;
use foundry_core::nbt::Tag;
use foundry_world::native_load_context::{DataLoadOrigin, LoadContext, LoadInputs, LoopSide};

fn load() -> LoadContext {
    LoadContext::new(
        DataLoadOrigin::ServerDataLoad,
        std::sync::Arc::new(LoadInputs::new(Default::default(), Default::default())),
        [],
    )
}

fn parse(source: &str, items: &Items) -> Ingredient {
    Ingredient::parse(&serde_json::from_str(source).unwrap(), items).unwrap()
}

fn resources() -> Items {
    let root = std::path::PathBuf::from(
        std::env::var_os("FOUNDRY_TEST_RESOURCES").expect("prepared resources"),
    );
    let defaults = foundry_world::item_defaults::read(&root).unwrap();
    Items::catalog(&root, &foundry_world::loot::PackOverlay::NONE, &defaults).unwrap()
}

mod families {
    use super::*;
    include!(concat!(
        env!("OUT_DIR"),
        "/builtin_ingredient_test_modules.rs"
    ));
}
