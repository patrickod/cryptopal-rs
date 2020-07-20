use std::collections::HashMap;
use std::result::Result;
use std::str::FromStr;

pub struct Profile {
    email: String,
    uid: u64,
    role: String,
}

impl Profile {
    pub fn for_email(email: &str) -> Self {
        let sanitized_email: String = email.chars()
            .filter( |c| !['&', '='].contains(c) )
            .collect();

        Self {
            email: sanitized_email.to_owned(),
            uid: 100,
            role: "user".to_owned()
        }
    }

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

    pub fn serialize(&self) -> String {
        format!("email={email}&uid={uid}&role={role}",
             email=self.email,
             uid=self.uid,
             role=self.role
        )
    }
}

#[cfg(test)]
mod tests {
    use profile::Profile;

    #[test]
    fn test_profile_parse() {
        Profile::parse("email=p@trickod.com&uid=100&role=admin").expect("unable to parse");
    }

    #[test]
    fn test_profile_for_email() {
        let p = Profile::for_email("p@trickod.com");
        assert_eq!(p.role, "user".to_owned());
    }

    #[test]
    fn test_profile_for_email_invalid_characters() {
        let p = Profile::for_email("p@trickod.com&role=admin");
        assert_eq!(p.email, "p@trickod.comroleadmin".to_owned());
    }

    #[test]
    fn test_profile_serialize() {
        let p = Profile::for_email("p@trickod.com");
        assert_eq!("email=p@trickod.com&uid=100&role=user", p.serialize());
    }
}
