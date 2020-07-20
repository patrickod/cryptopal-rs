use std::collections::HashMap;
use std::result::Result;
use std::str::FromStr;

pub struct Profile {
    email: String,
    uid: u64,
    role: String,
}

impl Profile {
    pub fn parse(serialized: &str) -> Result<Self, &'static str> {
        let mut kv: HashMap<String, String> = HashMap::new();
        let pairs = serialized.split('&');

        for pair in pairs {
            let mut kvs = pair.split('=');
            kv.insert(
                String::from_str(kvs.next().unwrap()).unwrap(),
                String::from_str(kvs.next().unwrap_or("")).unwrap(),
            );
        }

        if !kv.contains_key("email") || !kv.contains_key("role") || !kv.contains_key("uid") {
            return Err("Missing required uid, email, or role key");
        }

        Ok(Self {
            email: kv.get("email").unwrap().to_owned(),
            uid: u64::from_str(&kv.get("uid").unwrap()).unwrap(),
            role: kv.get("role").unwrap().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use profile::Profile;

    #[test]
    fn test_profile_parse() {
        Profile::parse("email=p@trickod.com&uid=100&role=admin").expect("unable to parse");
    }
}
