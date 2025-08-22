use serde_json::{json, Value};
use std::collections::HashSet;

pub const MASK_PLACEHOLDER: &str = "***MASKED***";

const MAX_DEPTH: usize = 8;

pub const SENSITIVE_FIELDS: &[&str] = &[
    "password", "pass", "passwd", "pwd", "token", "access_token", "accessToken",
    "refresh_token", "refreshToken", "secret", "api_key", "apiKey", "authorization",
    "auth_token", "authToken", "jwt", "session_id", "sessionId", "sessionToken",
    "client_secret", "clientSecret", "private_key", "privateKey", "public_key",
    "publicKey", "key", "encryption_key", "encryptionKey", "credit_card",
    "creditCard", "card_number", "cardNumber", "cvv", "cvc", "ssn", "sin", "pin",
    "security_code", "securityCode", "bank_account", "bankAccount", "iban",
    "swift", "bic", "routing_number", "routingNumber", "license_key", "licenseKey",
    "otp", "mfa_code", "mfaCode", "phone_number", "phoneNumber", "email",
    "address", "dob", "tax_id", "taxId", "passport_number", "passportNumber",
    "driver_license", "driverLicense", "set-cookie", "cookie", "authorization",
    "proxyAuthorization",
];

pub const SENSITIVE_HEADERS: &[&str] = &[
    "set-cookie", "cookie", "authorization", "proxyAuthorization",
];

#[derive(Clone)]
pub struct MaskConfig {
    pub is_mask_body_enabled: bool,
    pub is_mask_headers_enabled: bool,
    pub mask_body_fields_list: Vec<String>,
    pub mask_headers_list: Vec<String>,
}

impl Default for MaskConfig {
    fn default() -> Self {
        Self {
            is_mask_body_enabled: true,
            is_mask_headers_enabled: true,
            mask_body_fields_list: SENSITIVE_FIELDS.iter().map(|s| s.to_string()).collect(),
            mask_headers_list: SENSITIVE_HEADERS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub fn mask_all(value: &Value, depth: usize) -> Value {
    if depth > MAX_DEPTH {
        return json!(null);
    }

    match value {
        Value::Array(arr) => {
            let masked_array: Vec<Value> = arr.iter()
                .map(|item| mask_all(item, depth + 1))
                .collect();
            Value::Array(masked_array)
        }
        Value::Object(obj) => {
            let mut masked_obj = serde_json::Map::new();
            for (key, val) in obj {
                masked_obj.insert(key.clone(), mask_all(val, depth + 1));
            }
            Value::Object(masked_obj)
        }
        Value::String(_) => Value::String(MASK_PLACEHOLDER.to_string()),
        _ => value.clone(),
    }
}

pub fn mask_selected(value: &Value, keys_to_mask: &HashSet<String>) -> Value {
    match value {
        Value::Array(arr) => {
            let masked_array: Vec<Value> = arr.iter()
                .map(|item| mask_selected(item, keys_to_mask))
                .collect();
            Value::Array(masked_array)
        }
        Value::Object(obj) => {
            let mut masked_obj = serde_json::Map::new();
            for (key, val) in obj {
                if keys_to_mask.contains(key) {
                    masked_obj.insert(key.clone(), Value::String(MASK_PLACEHOLDER.to_string()));
                } else {
                    masked_obj.insert(key.clone(), mask_selected(val, keys_to_mask));
                }
            }
            Value::Object(masked_obj)
        }
        _ => value.clone(),
    }
}

pub fn mask_json_string(json_str: &str, config: &MaskConfig) -> String {
    let keys_to_mask: HashSet<String> = config.mask_body_fields_list.iter()
        .map(|s| s.to_lowercase())
        .collect();

    match serde_json::from_str::<Value>(json_str) {
        Ok(json_value) => {
            let masked_data = if keys_to_mask.is_empty() {
                mask_all(&json_value, 0)
            } else {
                mask_selected(&json_value, &keys_to_mask)
            };
            
            match serde_json::to_string(&masked_data) {
                Ok(result) => result,
                Err(_) => json_str.to_string(),
            }
        }
        Err(_) => json_str.to_string(),
    }
}

pub fn mask_headers(headers: &[(String, String)], config: &MaskConfig) -> Vec<(String, String)> {
    if !config.is_mask_headers_enabled {
        return headers.to_vec();
    }

    let headers_to_mask: HashSet<String> = config.mask_headers_list.iter()
        .map(|s| s.to_lowercase())
        .collect();

    headers.iter()
        .map(|(key, value)| {
            let key_lower = key.to_lowercase();
            if headers_to_mask.contains(&key_lower) {
                (key.clone(), MASK_PLACEHOLDER.to_string())
            } else {
                (key.clone(), value.clone())
            }
        })
        .collect()
}

pub fn mask_body(body: &str, config: &MaskConfig) -> String {
    if !config.is_mask_body_enabled {
        return body.to_string();
    }

    // Try to parse as JSON and mask if successful
    if body.trim().starts_with('{') || body.trim().starts_with('[') {
        mask_json_string(body, config)
    } else {
        // For non-JSON content, return as-is
        body.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_all() {
        let input = json!({
            "name": "John",
            "password": "secret123",
            "data": {
                "email": "john@example.com",
                "token": "abc123"
            }
        });

        let result = mask_all(&input, 0);
        let expected = json!({
            "name": "[MASKED]",
            "password": "[MASKED]",
            "data": {
                "email": "[MASKED]",
                "token": "[MASKED]"
            }
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mask_selected() {
        let input = json!({
            "name": "John",
            "password": "secret123",
            "email": "john@example.com"
        });

        let keys_to_mask: HashSet<String> = vec!["password".to_string()].into_iter().collect();
        let result = mask_selected(&input, &keys_to_mask);
        let expected = json!({
            "name": "John",
            "password": "[MASKED]",
            "email": "john@example.com"
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mask_headers() {
        let headers = vec![
            ("content-type".to_string(), "application/json".to_string()),
            ("authorization".to_string(), "Bearer token123".to_string()),
            ("user-agent".to_string(), "Mozilla/5.0".to_string()),
        ];

        let config = MaskConfig::default();
        let result = mask_headers(&headers, &config);

        assert_eq!(result[0].1, "application/json"); // content-type not masked
        assert_eq!(result[1].1, "[MASKED]"); // authorization masked
        assert_eq!(result[2].1, "Mozilla/5.0"); // user-agent not masked
    }
}
