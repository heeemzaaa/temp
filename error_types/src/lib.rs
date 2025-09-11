use chrono::Local;


#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    form_values: (&'static str, String),
    date: String,
    err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: "".to_string(),
            err: err
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError {
                form_values: ("name", self.name.clone()),
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty",
            });
        } else if self.password.len() < 8 {
            return Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long",
            });
        } else {
            let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
            let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
            let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

            if !(has_letter && has_digit && has_symbol) {
                return Err(FormError {
                    form_values: ("password", self.password.clone()),
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    err: "Password should be a combination of ASCII numbers, letters and symbols",
                });
            }
        }

        Ok(())
    }
}
