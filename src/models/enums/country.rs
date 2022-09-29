//! Model for Country enum

use serde::{Deserialize, Serialize};

/// Indicates the country associated with another entity, such as a business.
///
/// Values are in
/// [ISO 3166-1-alpha-2 format](http://www.iso.org/iso/home/standards/country_codes.htm).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Country {
    /// Unknown
    Zz,
    /// Andorra
    Ad,
    /// United Arab Emirates
    Ae,
    /// Afghanistan
    Af,
    /// Antigua and Barbuda
    Ag,
    /// Anguilla
    Ai,
    /// Albania
    Al,
    /// Armenia
    Am,
    /// Angola
    Ao,
    /// Antartica
    Aq,
    /// Argentina
    Ar,
    /// American Samoa
    As,
    /// Austria
    At,
    /// Australia
    Au,
    /// Aruba
    Aw,
    /// Åland Islands
    Ax,
    /// Azerbaijan
    Az,
    /// Bosnia and Herzegovina
    Ba,
    /// Barbados
    Bb,
    /// Bangladesh
    Bd,
    /// Belgium
    Be,
    /// Burkina Faso
    Bf,
    /// Bulgaria
    Bg,
    /// Bahrain
    Bh,
    /// Burundi
    Bi,
    /// Benin
    Bj,
    /// Saint Barthélemy
    Bl,
    /// Bermuda
    Bm,
    /// Brunei
    Bn,
    /// Bolivia
    Bo,
    /// Bonaire
    Bq,
    /// Brazil
    Br,
    /// Bahamas
    Bs,
    /// Bhutan
    Bt,
    /// Bouvet Island
    Bv,
    /// Botswana
    Bw,
    /// Belarus
    By,
    /// Belize
    Bz,
    /// Canada
    Ca,
    /// Cocos Islands
    Cc,
    /// Democratic Republic of the Congo
    Cd,
    /// Central African Republic
    Cf,
    /// Congo
    Cg,
    /// Switzerland
    Ch,
    /// Ivory Coast
    Ci,
    /// Cook Islands
    Ck,
    /// Chile
    Cl,
    /// Cameroon
    Cm,
    /// China
    Cn,
    /// Colombia
    Co,
    /// Costa Rica
    Cr,
    /// Cuba
    Cu,
    /// Cabo Verde
    Cv,
    /// Curaçao
    Cw,
    /// Christmas Island
    Cx,
    /// Cyprus
    Cy,
    /// Czechia
    Cz,
    /// Germany
    De,
    /// Djibouti
    Dj,
    /// Denmark
    Dk,
    /// Dominica
    Dm,
    /// Dominican Republic
    Do,
    /// Algeria
    Dz,
    /// Ecuador
    Ec,
    /// Estonia
    Ee,
    /// Egypt
    Eg,
    /// Western Sahara
    Eh,
    /// Eritrea
    Er,
    /// Spain
    Es,
    /// Ethiopia
    Et,
    /// Finland
    Fi,
    /// Fiji
    Fj,
    /// Falkland Islands
    Fk,
    /// Federated States of Micronesia
    Fm,
    /// Faroe Islands
    Fo,
    /// France
    Fr,
    /// Gabon
    Ga,
    /// United Kingdom
    Gb,
    /// Grenada
    Gd,
    /// Georgia
    Ge,
    /// French Guiana
    Gf,
    /// Guernsey
    Gg,
    /// Ghana
    Gh,
    /// Gibraltar
    Gi,
    /// Greenland
    Gl,
    /// Gambia
    Gm,
    /// Guinea
    Gn,
    /// Guadeloupe
    Gp,
    /// Equatorial Guinea
    Gq,
    /// Greece
    Gr,
    /// South Georgia and the South Sandwich Islands
    Gs,
    /// Guatemala
    Gt,
    /// Guam
    Gu,
    /// Guinea-Bissau
    Gw,
    /// Guyana
    Gy,
    /// Hong Kong
    Hk,
    /// Heard Island and McDonald Islands
    Hm,
    /// Honduras
    Hn,
    /// Croatia
    Hr,
    /// Haiti
    Ht,
    /// Hungary
    Hu,
    /// Indonesia
    Id,
    /// Ireland
    Ie,
    /// Israel
    Il,
    /// Isle of Man
    Im,
    /// India
    In,
    /// British Indian Ocean Territory
    Io,
    /// Iraq
    Iq,
    /// Iran
    Ir,
    /// Iceland
    Is,
    /// Italy
    It,
    /// Jersey
    Je,
    /// Jamaica
    Jm,
    /// Jordan
    Jo,
    /// Japan
    Jp,
    /// Kenya
    Ke,
    /// Kyrgyzstan
    Kg,
    /// Cambodia
    Kh,
    /// Kiribati
    Ki,
    /// Comoros
    Km,
    /// Saint Kitts and Nevis
    Kn,
    /// Democratic People's Republic of Korea
    Kp,
    /// Republic of Korea
    Kr,
    /// Kuwait
    Kw,
    /// Cayman Islands
    Ky,
    /// Kazakhstan
    Kz,
    /// Lao People's Democratic Republic
    La,
    /// Lebanon
    Lb,
    /// Saint Lucia
    Lc,
    /// Liechtenstein
    Li,
    /// Sri Lanka
    Lk,
    /// Liberia
    Lr,
    /// Lesotho
    Ls,
    /// Lithuania
    Lt,
    /// Luxembourg
    Lu,
    /// Latvia
    Lv,
    /// Libya
    Ly,
    /// Morocco
    Ma,
    /// Monaco
    Mc,
    /// Moldova
    Md,
    /// Montenegro
    Me,
    /// Saint Martin
    Mf,
    /// Madagascar
    Mg,
    /// Marshall Islands
    Mh,
    /// North Macedonia
    Mk,
    /// Mali
    Ml,
    /// Myanmar
    Mm,
    /// Mongolia
    Mn,
    /// Macao
    Mo,
    /// Northern Mariana Islands
    Mp,
    /// Martinique
    Mq,
    /// Mauritania
    Mr,
    /// Montserrat
    Ms,
    /// Malta
    Mt,
    /// Mauritius
    Mu,
    /// Maldives
    Mv,
    /// Malawi
    Mw,
    /// Mexico
    Mx,
    /// Malaysia
    My,
    /// Mozambique
    Mz,
    /// Namibia
    Na,
    /// New Caledonia
    Nc,
    /// Niger
    Ne,
    /// Norfolk Island
    Nf,
    /// Nigeria
    Ng,
    /// Nicaragua
    Ni,
    /// Netherlands
    Nl,
    /// Norway
    No,
    /// Nepal
    Np,
    /// Nauru
    Nr,
    /// Niue
    Nu,
    /// New Zealand
    Nz,
    /// Oman
    Om,
    /// Panama
    Pa,
    /// Peru
    Pe,
    /// French Polynesia
    Pf,
    /// Papua New Guinea
    Pg,
    /// Philippines
    Ph,
    /// Pakistan
    Pk,
    /// Poland
    Pl,
    /// Saint Pierre and Miquelon
    Pm,
    /// Pitcairn
    Pn,
    /// Puerto Rico
    Pr,
    /// Palestine
    Ps,
    /// Portugal
    Pt,
    /// Palau
    Pw,
    /// Paraguay
    Py,
    /// Qatar
    Qa,
    /// Réunion
    Re,
    /// Romania
    Ro,
    /// Serbia
    Rs,
    /// Russia
    Ru,
    /// Rwanda
    Rw,
    /// Saudi Arabia
    Sa,
    /// Solomon Islands
    Sb,
    /// Seychelles
    Sc,
    /// Sudan
    Sd,
    /// Sweden
    Se,
    /// Singapore
    Sg,
    /// Saint Helena, Ascension and Tristan da Cunha
    Sh,
    /// Slovenia
    Si,
    /// Svalbard and Jan Mayen
    Sj,
    /// Slovakia
    Sk,
    /// Sierra Leone
    Sl,
    /// San Marino
    Sm,
    /// Senegal
    Sn,
    /// Somalia
    So,
    /// Suriname
    Sr,
    /// South Sudan
    Ss,
    /// Sao Tome and Principe
    St,
    /// El Salvador
    Sv,
    /// Sint Maarten
    Sx,
    /// Syrian Arab Republic
    Sy,
    /// Eswatini
    Sz,
    /// Turks and Caicos Islands
    Tc,
    /// Chad
    Td,
    /// French Southern Territories
    Tf,
    /// Togo
    Tg,
    /// Thailand
    Th,
    /// Tajikistan
    Tj,
    /// Tokelau
    Tk,
    /// Timor-Leste
    Tl,
    /// Turkmenistan
    Tm,
    /// Tunisia
    Tn,
    /// Tonga
    To,
    /// Turkey
    Tr,
    /// Trinidad and Tobago
    Tt,
    /// Tuvalu
    Tv,
    /// Taiwan
    Tw,
    /// Tanzania
    Tz,
    /// Ukraine
    Ua,
    /// Uganda
    Ug,
    /// United States Minor Outlying Islands
    Um,
    /// United States of America
    Us,
    /// Uruguay
    Uy,
    /// Uzbekistan
    Uz,
    /// Vatican City
    Va,
    /// Saint Vincent and the Grenadines
    Vc,
    /// Venezuela
    Ve,
    /// British Virgin Islands
    Vg,
    /// U.S. Virgin Islands
    Vi,
    /// Vietnam
    Vn,
    /// Vanuatu
    Vu,
    /// Wallis and Futuna
    Wf,
    /// Samoa
    Ws,
    /// Yemen
    Ye,
    /// Mayotte
    Yt,
    /// South Africa
    Za,
    /// Zambia
    Zm,
    /// Zimbabwe
    Zw,
}
