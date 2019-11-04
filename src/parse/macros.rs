/// This macro defines other macros for easy parsing of YAML values
macro_rules! yaml_macro {
    ($macro_name:tt, $converter:tt, $type:expr) => {
        macro_rules! $macro_name {
            ($hash:ident, $key:ident, $context:expr) => {
                match $hash.get(&Yaml::String(stringify!($key).to_string())) {
                    Some(yaml) => {
                        match yaml.$converter() {
                            Some(value) => Some(value),
                            None => {
                                return Err(ParseError::InvalidType(format!(
                                    "Expected the value of key `{}` to be {}, instead it was {:?}",
                                    stringify!($key),
                                    $type,
                                    yaml,
                                ), $context.to_string()));
                            }
                        }
                    },
                    None => None
                }
            };
            (require: $hash:ident, $key:ident, $context:expr) => {
                match $macro_name!($hash, $key, $context) {
                    Some(value) => value,
                    None => {
                        return Err(ParseError::MissingRequiredKey(format!(
                            "Missing required key `{}`",
                            stringify!($key),
                        ), $context.to_string()));
                    }
                }
            };
        }
    }
}

// Define the various macros for easy parsing of YAML with suitable error messages
yaml_macro!(yaml_vec, as_vec, "an array (list)");
yaml_macro!(yaml_str, as_str, "a string (text)");
yaml_macro!(yaml_bool, as_bool, "a boolean (true or false)");
yaml_macro!(yaml_hash, as_hash, "a hash (key => value)");
