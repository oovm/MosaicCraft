use mosaic_pixel::WorkspaceStorage;

#[test]
fn test() {
    let root = WorkspaceStorage::new("../../target/mosaic-craft").unwrap();
    let set1 = root.get_gallery("Tank City").unwrap();
    let set2 = root.get_gallery("Super Mario").unwrap();
    println!("{:?}", set1);
    println!("{:?}", set2);
}



