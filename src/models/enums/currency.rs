//! Model for Currency enum

use serde::{Deserialize, Serialize};

/// Indicates the associated currency for an amount of money.
///
/// Values correspond to [ISO 4217](https://wikipedia.org/wiki/ISO_4217).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    /// Unknown currency
    UnknownCurrency,
    /// United Arab Emirates dirham
    Aed,
    /// Afghan afghani
    Afn,
    /// Albanian lek
    All,
    /// Armenian dram
    Amd,
    /// Netherlands Antillean guilder
    Ang,
    /// Angolan kwanza
    Aoa,
    /// Argentine peso
    Ars,
    /// Australian dollar
    Aud,
    /// Aruban florin
    Awg,
    /// Azerbaijani manat
    Azn,
    /// Bosnia and Herzegovina convertible mark
    Bam,
    /// Barbados dollar
    Bbd,
    /// Bangladeshi taka
    Bdt,
    /// Bulgarian lev
    Bgn,
    /// Bahraini dinar
    Bhd,
    /// Burundian franc
    Bif,
    /// Bermudian dollar
    Bmd,
    /// Brunei dollar
    Bnd,
    /// Boliviano
    Bob,
    /// Bolivian Mvdol
    Bov,
    /// Brazilian real
    Brl,
    /// Bahamian dollar
    Bsd,
    /// Bhutanese ngultrum
    Btn,
    /// Botswana pula
    Bwp,
    /// Belarusian ruble
    Byr,
    /// Belize dollar
    Bzd,
    /// Canadian dollar
    Cad,
    /// Congolese franc
    Cdf,
    /// WIR Euro
    Che,
    /// Swiss franc
    Chf,
    /// WIR Franc
    Chw,
    /// Unidad de Fomento
    Clf,
    /// Chilean peso
    Clp,
    /// Chinese yuan
    Cny,
    /// Colombian peso
    Cop,
    /// Unidad de Valor Real
    Cou,
    /// Costa Rican colon
    Crc,
    /// Cuban convertible peso
    Cuc,
    /// Cuban peso
    Cup,
    /// Cape Verdean escudo
    Cve,
    /// Czech koruna
    Czk,
    /// Djiboutian franc
    Djf,
    /// Danish krone
    Dkk,
    /// Dominican peso
    Dop,
    /// Algerian dinar
    Dzd,
    /// Egyptian pound
    Egp,
    /// Eritrean nakfa
    Ern,
    /// Ethiopian birr
    Etb,
    /// Euro
    Eur,
    /// Fiji dollar
    Fjd,
    /// Falkland Islands pound
    Fkp,
    /// Pound sterling
    Gbp,
    /// Georgian lari
    Gel,
    /// Ghanaian cedi
    Ghs,
    /// Gibraltar pound
    Gip,
    /// Gambian dalasi
    Gmd,
    /// Guinean franc
    Gnf,
    /// Guatemalan quetzal
    Gtq,
    /// Guyanese dollar
    Gyd,
    /// Hong Kong dollar
    Hkd,
    /// Honduran lempira
    Hnl,
    /// Croatian kuna
    Hrk,
    /// Haitian gourde
    Htg,
    /// Hungarian forint
    Huf,
    /// Indonesian rupiah
    Idr,
    /// Israeli new shekel
    Ils,
    /// Indian rupee
    Inr,
    /// Iraqi dinar
    Iqd,
    /// Iranian rial
    Irr,
    /// Icelandic króna
    Isk,
    /// Jamaican dollar
    Jmd,
    /// Jordanian dinar
    Jod,
    /// Japanese yen
    Jpy,
    /// Kenyan shilling
    Kes,
    /// Kyrgyzstani som
    Kgs,
    /// Cambodian riel
    Khr,
    /// Comoro franc
    Kmf,
    /// North Korean won
    Kpw,
    /// South Korean won
    Krw,
    /// Kuwaiti dinar
    Kwd,
    /// Cayman Islands dollar
    Kyd,
    /// Kazakhstani tenge
    Kzt,
    /// Lao kip
    Lak,
    /// Lebanese pound
    Lbp,
    /// Sri Lankan rupee
    Lkr,
    /// Liberian dollar
    Lrd,
    /// Lesotho loti
    Lsl,
    /// Lithuanian litas
    Ltl,
    /// Latvian lats
    Lvl,
    /// Libyan dinar
    Lyd,
    /// Moroccan dirham
    Mad,
    /// Moldovan leu
    Mdl,
    /// Malagasy ariary
    Mga,
    /// Macedonian denar
    Mkd,
    /// Myanmar kyat
    Mmk,
    /// Mongolian tögrög
    Mnt,
    /// Macanese pataca
    Mop,
    /// Mauritanian ouguiya
    Mro,
    /// Mauritian rupee
    Mur,
    /// Maldivian rufiyaa
    Mvr,
    /// Malawian kwacha
    Mwk,
    /// Mexican peso
    Mxn,
    /// Mexican Unidad de Inversion
    Mxv,
    /// Malaysian ringgit
    Myr,
    /// Mozambican metical
    Mzn,
    /// Namibian dollar
    Nad,
    /// Nigerian naira
    Ngn,
    /// Nicaraguan córdoba
    Nio,
    /// Norwegian krone
    Nok,
    /// Nepalese rupee
    Npr,
    /// New Zealand dollar
    Nzd,
    /// Omani rial
    Omr,
    /// Panamanian balboa
    Pab,
    /// Peruvian sol
    Pen,
    /// Papua New Guinean kina
    Pgk,
    /// Philippine peso
    Php,
    /// Pakistani rupee
    Pkr,
    /// Polish złoty
    Pln,
    /// Paraguayan guaraní
    Pyg,
    /// Qatari riyal
    Qar,
    /// Romanian leu
    Ron,
    /// Serbian dinar
    Rsd,
    /// Russian ruble
    Rub,
    /// Rwandan franc
    Rwf,
    /// Saudi riyal
    Sar,
    /// Solomon Islands dollar
    Sbd,
    /// Seychelles rupee
    Scr,
    /// Sudanese pound
    Sdg,
    /// Swedish krona
    Sek,
    /// Singapore dollar
    Sgd,
    /// Saint Helena pound
    Shp,
    /// Sierra Leonean leone
    Sll,
    /// Somali shilling
    Sos,
    /// Surinamese dollar
    Srd,
    /// South Sudanese pound
    Ssp,
    /// São Tomé and Príncipe dobra
    Std,
    /// Salvadoran colón
    Svc,
    /// Syrian pound
    Syp,
    /// Swazi lilangeni
    Szl,
    /// Thai baht
    Thb,
    /// Tajikstani somoni
    Tjs,
    /// Turkmenistan manat
    Tmt,
    /// Tunisian dinar
    Tnd,
    /// Tongan pa'anga
    Top,
    /// Turkish lira
    Try,
    /// Trinidad and Tobago dollar
    Ttd,
    /// New Taiwan dollar
    Twd,
    /// Tanzanian shilling
    Tzs,
    /// Ukrainian hryvnia
    Uah,
    /// Ugandan shilling
    Ugx,
    /// United States dollar
    Usd,
    /// United States dollar (next day)
    Usn,
    /// United States dollar (same day)
    Uss,
    /// Uruguay Peso en Unidedades Indexadas
    Uyi,
    /// Uruguyan peso
    Uyu,
    /// Uzbekistan som
    Uzs,
    /// Venezuelan bolívar soberano
    Vef,
    /// Vietnamese đồng
    Vnd,
    /// Vanuatu vatu
    Vuv,
    /// Samoan tala
    Wst,
    /// CFA franc BEAC
    Xaf,
    /// Silver
    Xag,
    /// Gold
    Xau,
    /// European Composite Unit
    Xba,
    /// European Monetary Unit
    Xbb,
    /// European Unit of Account 9
    Xbc,
    /// European Unit of Account 17
    Xbd,
    /// East Caribbean dollar
    Xcd,
    /// Special drawing rights (International Monetary Fund)
    Xdr,
    /// CFA franc BCEAO
    Xof,
    /// Palladium
    Xpd,
    /// CFP franc
    Xpf,
    /// Platinum
    Xpt,
    /// Code reserved for testing
    Xts,
    /// No currency
    Xxx,
    /// Yemeni rial
    Yer,
    /// South African rand
    Zar,
    /// Zambian kwacha
    Zmk,
    /// Zambian kwacha
    Zmw,
    /// Bitcoin
    Btc,
}

impl Default for Currency {
    fn default() -> Self {
        Self::UnknownCurrency
    }
}
