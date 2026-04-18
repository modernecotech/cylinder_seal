//! Iraqi National Card (البطاقة الموحدة) format validation.
//!
//! The unified national card replaces the older civil-status (jinsiyya)
//! and information (hawiyyat al-ahwal al-madaniyya) cards. The card number
//! is 12 digits; the leading 2 digits encode the issuing governorate
//! (01–18 covering all 18 Iraqi governorates, including the three KRG
//! ones — Erbil 13, Sulaymaniyah 14, Duhok 15).
//!
//! We only enforce structural format here. There is no public checksum
//! specification we can rely on, and the canonical verification path is
//! a back-office check against the General Directorate of Nationality
//! registry — wired separately as an admin-side enrichment step.

use std::fmt;

/// Total digit count of an Iraqi unified national card number.
pub const IQ_NATIONAL_CARD_LEN: usize = 12;

/// Highest valid governorate prefix (01–18 inclusive).
pub const IQ_GOVERNORATE_MAX: u8 = 18;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IraqiIdError {
    Empty,
    WrongLength { actual: usize },
    NonDigit,
    UnknownGovernorate { prefix: u8 },
    AllZeroSerial,
}

impl fmt::Display for IraqiIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "id_number must not be empty"),
            Self::WrongLength { actual } => write!(
                f,
                "Iraqi National Card must be {IQ_NATIONAL_CARD_LEN} digits (got {actual})"
            ),
            Self::NonDigit => write!(f, "Iraqi National Card must contain only digits 0-9"),
            Self::UnknownGovernorate { prefix } => write!(
                f,
                "Iraqi National Card prefix {prefix:02} is not a valid governorate code (01–{IQ_GOVERNORATE_MAX})"
            ),
            Self::AllZeroSerial => write!(f, "Iraqi National Card serial portion must not be all zero"),
        }
    }
}

impl std::error::Error for IraqiIdError {}

/// Validate an Iraqi unified national card number. Accepts only ASCII
/// digits — Arabic-Indic digits should be folded by the caller before
/// reaching this function (the screening normaliser already does so).
///
/// Strips surrounding whitespace and any internal `-` / space separators
/// commonly inserted by data-entry operators (e.g. `1234 56 789012`).
pub fn validate_iraqi_national_card(s: &str) -> Result<String, IraqiIdError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(IraqiIdError::Empty);
    }
    let digits: String = trimmed
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect();

    if digits.len() != IQ_NATIONAL_CARD_LEN {
        return Err(IraqiIdError::WrongLength { actual: digits.len() });
    }
    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(IraqiIdError::NonDigit);
    }

    let prefix: u8 = digits[..2].parse().map_err(|_| IraqiIdError::NonDigit)?;
    if prefix == 0 || prefix > IQ_GOVERNORATE_MAX {
        return Err(IraqiIdError::UnknownGovernorate { prefix });
    }

    if digits[2..].chars().all(|c| c == '0') {
        return Err(IraqiIdError::AllZeroSerial);
    }

    Ok(digits)
}

/// Returns true if the given governorate prefix belongs to the Kurdistan
/// Regional Government (Erbil 13, Sulaymaniyah 14, Duhok 15). Used by the
/// region-routing layer to default new accounts to `region = 'krg'` when
/// onboarding via a KRG-issued card.
pub fn is_krg_governorate(prefix: u8) -> bool {
    matches!(prefix, 13 | 14 | 15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_well_formed_baghdad_card() {
        let id = validate_iraqi_national_card("011234567890").unwrap();
        assert_eq!(id, "011234567890");
    }

    #[test]
    fn accepts_card_with_separators() {
        let id = validate_iraqi_national_card("01-1234 567890").unwrap();
        assert_eq!(id, "011234567890");
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(validate_iraqi_national_card("   "), Err(IraqiIdError::Empty));
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(matches!(
            validate_iraqi_national_card("0112345"),
            Err(IraqiIdError::WrongLength { .. })
        ));
    }

    #[test]
    fn rejects_non_digit() {
        assert_eq!(
            validate_iraqi_national_card("01123456789X"),
            Err(IraqiIdError::NonDigit)
        );
    }

    #[test]
    fn rejects_unknown_governorate() {
        assert_eq!(
            validate_iraqi_national_card("991234567890"),
            Err(IraqiIdError::UnknownGovernorate { prefix: 99 })
        );
        assert_eq!(
            validate_iraqi_national_card("001234567890"),
            Err(IraqiIdError::UnknownGovernorate { prefix: 0 })
        );
    }

    #[test]
    fn rejects_all_zero_serial() {
        assert_eq!(
            validate_iraqi_national_card("010000000000"),
            Err(IraqiIdError::AllZeroSerial)
        );
    }

    #[test]
    fn detects_krg_governorates() {
        assert!(is_krg_governorate(13));
        assert!(is_krg_governorate(14));
        assert!(is_krg_governorate(15));
        assert!(!is_krg_governorate(1));
        assert!(!is_krg_governorate(18));
    }

    #[test]
    fn accepts_krg_erbil_card() {
        let id = validate_iraqi_national_card("131234567890").unwrap();
        assert_eq!(&id[..2], "13");
        let prefix: u8 = id[..2].parse().unwrap();
        assert!(is_krg_governorate(prefix));
    }
}
