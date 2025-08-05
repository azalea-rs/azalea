use azalea_buf::checksum::AzaleaChecksum;
use azalea_chat::FormattedText;
use azalea_inventory::components::{CustomName, MapColor};

#[test]
fn test_custom_name_checksum() {
    let c = CustomName {
        name: FormattedText::from("meow"),
    };
    println!("{:?}", c);
    assert_eq!(c.azalea_checksum().0, 2222287064);
}
#[test]
fn test_map_color_checksum() {
    let c = MapColor { color: 1 };
    assert_eq!(c.azalea_checksum().0, 1565579036);
}
