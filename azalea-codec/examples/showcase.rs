//! A showcase of some of what azalea-codec can do.

use azalea_codec::Codec;
use serde_json::json;

#[derive(Codec, PartialEq, Clone, Debug)]
enum FormattedText {
    Text {
        text: String,
        #[codec(flatten)]
        base: BaseComponent,
    },
    Translatable(TranslatableComponent),
}
#[derive(Codec, PartialEq, Clone, Debug)]
struct TranslatableComponent {
    #[codec(rename = "translate")]
    key: String,
    #[codec(flatten)]
    base: BaseComponent,
}
#[derive(Codec, PartialEq, Clone, Debug)]
#[codec(allow_unknown_keys)]
struct BaseComponent {
    #[codec(optional, flatten)]
    style: Box<Style>,
    #[codec(optional)]
    extra: Vec<FormattedText>,
}
#[derive(Codec, PartialEq, Clone, Debug)]
struct Style {
    #[codec(optional, default = "white".to_string())]
    color: String,
}

fn main() {
    // Reading JSON
    let json = json!({
        "text": "Hello, ",
        "extra": [{
            "translate": "entity.minecraft.cat",
            "color": "yellow",
            "unknown field!!!": "asdf"
        }]
    });
    let formatted_text = FormattedText::codec_read(&json, Default::default()).unwrap();
    assert_eq!(
        formatted_text,
        FormattedText::Text {
            text: "Hello, ".to_string(),
            base: BaseComponent {
                style: Box::new(Style {
                    color: "white".into()
                }),
                extra: vec![FormattedText::Translatable(TranslatableComponent {
                    key: "entity.minecraft.cat".into(),
                    base: BaseComponent {
                        style: Box::new(Style {
                            color: "yellow".into()
                        }),
                        extra: vec![]
                    }
                })]
            }
        }
    );

    // Writing NBT
    use simdnbt::owned::*;
    let nbt: NbtTag = formatted_text.codec_write(Default::default()).unwrap();
    assert_eq!(
        nbt,
        NbtTag::Compound(NbtCompound::from_values(vec![
            ("text".into(), "Hello, ".into()),
            (
                "extra".into(),
                [NbtCompound::from_values(vec![
                    ("translate".into(), "entity.minecraft.cat".into()),
                    ("color".into(), "yellow".into())
                ])]
                .into()
            )
        ]))
    );
}
