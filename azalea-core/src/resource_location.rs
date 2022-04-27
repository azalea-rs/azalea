//! A resource, like minecraft:stone

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct ResourceLocation {
    pub namespace: String,
    pub path: String,
}

static DEFAULT_NAMESPACE: &str = "minecraft";
// static REALMS_NAMESPACE: &str = "realms";

impl ResourceLocation {
    pub fn new(resource_string: &str) -> Result<ResourceLocation, String> {
        let sep_byte_position_option = resource_string.chars().position(|c| c == ':');
        let (namespace, path) = if let Some(sep_byte_position) = sep_byte_position_option {
            if sep_byte_position == 0 {
                (DEFAULT_NAMESPACE, &resource_string[1..])
            } else {
                (
                    &resource_string[..sep_byte_position],
                    &resource_string[sep_byte_position + 1..],
                )
            }
        } else {
            (DEFAULT_NAMESPACE, resource_string)
        };
        Ok(ResourceLocation {
            namespace: namespace.to_string(),
            path: path.to_string(),
        })
    }
}

impl std::fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
impl std::fmt::Debug for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_resource_location() {
        let r = ResourceLocation::new("abcdef:ghijkl").unwrap();
        assert_eq!(r.namespace, "abcdef");
        assert_eq!(r.path, "ghijkl");
    }
    #[test]
    fn no_namespace() {
        let r = ResourceLocation::new("azalea").unwrap();
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_start() {
        let r = ResourceLocation::new(":azalea").unwrap();
        assert_eq!(r.namespace, "minecraft");
        assert_eq!(r.path, "azalea");
    }
    #[test]
    fn colon_end() {
        let r = ResourceLocation::new("azalea:").unwrap();
        assert_eq!(r.namespace, "azalea");
        assert_eq!(r.path, "");
    }
}
