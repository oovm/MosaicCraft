use mosaic_pixel::{KeyColor, StorageManager};

#[test]
fn test() {
    let root = StorageManager::new("../../target/mosaic-craft").unwrap();
    let set1 = root.get_gallery("Super Mario").unwrap();
    let c1 = set1.get_closest_color(KeyColor::BLUE);
    set1.get_image(c1).unwrap();
    println!("{}", c1);
}
