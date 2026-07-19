use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: Uuid,
    pub iat: usize, // issued at
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use super::Claims;
    use uuid::Uuid;

    #[test]
    fn claims_serde_roundtrip_preserves_every_field() {
        let claims = Claims {
            iss: "Observa".to_string(),
            sub: Uuid::new_v4(),
            iat: 1_700_000_000,
            exp: 1_700_003_600,
        };

        let json = serde_json::to_string(&claims).unwrap();
        let back: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(back.iss, claims.iss);
        assert_eq!(back.sub, claims.sub);
        assert_eq!(back.iat, claims.iat);
        assert_eq!(back.exp, claims.exp);
    }

    #[test]
    fn claims_reject_a_non_uuid_subject() {
        let json = r#"{"iss":"Observa","sub":"not-a-uuid","iat":1,"exp":2}"#;
        assert!(serde_json::from_str::<Claims>(json).is_err());
    }
}
