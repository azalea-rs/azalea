struct BoolArgumentType {
	// private static final Collection<String> EXAMPLES = Arrays.asList("true", "false");
	const EXAMPLES: &'static [&'static str] = &["true", "false"];
}

impl ArgumentType for BoolArgumentType {
	
}