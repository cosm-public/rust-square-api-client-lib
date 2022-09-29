//! Model struct for TaxIds type

use serde::{Deserialize, Serialize};

///  Identifiers for the location used by various governments for tax purposes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TaxIds {
    /// The NIF (Numero de Identificacion Fiscal) number is a nine-character tax identifier used in
    /// Spain. If it is present, it has been validated. For example, `73628495A`.
    pub es_nif: Option<String>,
    /// The EU VAT number for this location. For example, `IE3426675K`. If the EU VAT number is
    /// present, it is well-formed and has been validated with VIES, the VAT Information Exchange
    /// System.
    pub eu_vat: Option<String>,
    /// The French government uses the NAF (Nomenclature des Activités Françaises) to display and
    /// track economic statistical data. This is also called the APE (Activite Principale de
    /// l’Entreprise) code. For example, `6910Z`.
    pub fr_naf: Option<String>,
    /// The SIRET (Système d'Identification du Répertoire des Entreprises et de leurs
    /// Etablissements) number is a 14-digit code issued by the French INSEE. For example,
    /// `39922799000021`.
    pub fr_siret: Option<String>,
}
