//! Model for Timezone enum

use serde::{Deserialize, Serialize};

/// An [IANA time zone](https://www.iana.org/time-zones) identifier for a time zone.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Timezone {
    #[serde(rename = "Africa/Abidjan")]
    AfricaAbidjan,
    #[serde(rename = "Africa/Accra")]
    AfricaAccra,
    #[serde(rename = "Africa/Addis_Ababa")]
    AfricaAddisAbaba,
    #[serde(rename = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(rename = "Africa/Asmara")]
    AfricaAsmara,
    #[serde(rename = "Africa/Asmera")]
    AfricaAsmera,
    #[serde(rename = "Africa/Bamako")]
    AfricaBamako,
    #[serde(rename = "Africa/Bangui")]
    AfricaBangui,
    #[serde(rename = "Africa/Banjul")]
    AfricaBanjul,
    #[serde(rename = "Africa/Bissau")]
    AfricaBissau,
    #[serde(rename = "Africa/Blantyre")]
    AfricaBlantyre,
    #[serde(rename = "Africa/Brazzaville")]
    AfricaBrazzaville,
    #[serde(rename = "Africa/Bujumbura")]
    AfricaBujumbura,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(rename = "Africa/Ceuta")]
    AfricaCeuta,
    #[serde(rename = "Africa/Conakry")]
    AfricaConakry,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Dar_es_Salaam")]
    AfricaDarEsSalaam,
    #[serde(rename = "Africa/Djibouti")]
    AfricaDjibouti,
    #[serde(rename = "Africa/Douala")]
    AfricaDouala,
    #[serde(rename = "Africa/El_Aaiun")]
    AfricaElAaiun,
    #[serde(rename = "Africa/Freetown")]
    AfricaFreetown,
    #[serde(rename = "Africa/Gaborone")]
    AfricaGaborone,
    #[serde(rename = "Africa/Harare")]
    AfricaHarare,
    #[serde(rename = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "Africa/Juba")]
    AfricaJuba,
    #[serde(rename = "Africa/Kampala")]
    AfricaKampala,
    #[serde(rename = "Africa/Khartoum")]
    AfricaKhartoum,
    #[serde(rename = "Africa/Kigali")]
    AfricaKigali,
    #[serde(rename = "Africa/Kinshasa")]
    AfricaKinshasa,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "Africa/Libreville")]
    AfricaLibreville,
    #[serde(rename = "Africa/Lome")]
    AfricaLome,
    #[serde(rename = "Africa/Luanda")]
    AfricaLuanda,
    #[serde(rename = "Africa/Lubumbashi")]
    AfricaLubumbashi,
    #[serde(rename = "Africa/Lusaka")]
    AfricaLusaka,
    #[serde(rename = "Africa/Malabo")]
    AfricaMalabo,
    #[serde(rename = "Africa/Maputo")]
    AfricaMaputo,
    #[serde(rename = "Africa/Maseru")]
    AfricaMaseru,
    #[serde(rename = "Africa/Mbabane")]
    AfricaMbabane,
    #[serde(rename = "Africa/Mogadishu")]
    AfricaMogadishu,
    #[serde(rename = "Africa/Monrovia")]
    AfricaMonrovia,
    #[serde(rename = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(rename = "Africa/Ndjamena")]
    AfricaNdjamena,
    #[serde(rename = "Africa/Niamey")]
    AfricaNiamey,
    #[serde(rename = "Africa/Nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "Africa/Ouagadougou")]
    AfricaOuagadougou,
    #[serde(rename = "Africa/Porto-Novo")]
    AfricaPortoNovo,
    #[serde(rename = "Africa/Sao_Tome")]
    AfricaSaoTome,
    #[serde(rename = "Africa/Timbuktu")]
    AfricaTimbuktu,
    #[serde(rename = "Africa/Tripoli")]
    AfricaTripoli,
    #[serde(rename = "Africa/Tunis")]
    AfricaTunis,
    #[serde(rename = "Africa/Windhoek")]
    AfricaWindhoek,
    #[serde(rename = "America/Adak")]
    AmericaAdak,
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Anguilla")]
    AmericaAnguilla,
    #[serde(rename = "America/Antigua")]
    AmericaAntigua,
    #[serde(rename = "America/Araguaina")]
    AmericaAraguaina,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Argentina/Catamarca")]
    AmericaArgentinaCatamarca,
    #[serde(rename = "America/Argentina/ComodRivadavia")]
    AmericaArgentinaComodRivadavia,
    #[serde(rename = "America/Argentina/Cordoba")]
    AmericaArgentinaCordoba,
    #[serde(rename = "America/Argentina/Jujuy")]
    AmericaArgentinaJujuy,
    #[serde(rename = "America/Argentina/La_Rioja")]
    AmericaArgentinaLaRioja,
    #[serde(rename = "America/Argentina/Mendoza")]
    AmericaArgentinaMendoza,
    #[serde(rename = "America/Argentina/Rio_Gallegos")]
    AmericaArgentinaRioGallegos,
    #[serde(rename = "America/Argentina/Salta")]
    AmericaArgentinaSalta,
    #[serde(rename = "America/Argentina/San_Juan")]
    AmericaArgentinaSanJuan,
    #[serde(rename = "America/Argentina/San_Luis")]
    AmericaArgentinaSanLuis,
    #[serde(rename = "America/Argentina/Tucuman")]
    AmericaArgentinaTucuman,
    #[serde(rename = "America/Argentina/Ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(rename = "America/Aruba")]
    AmericaAruba,
    #[serde(rename = "America/Asuncion")]
    AmericaAsuncion,
    #[serde(rename = "America/Atikokan")]
    AmericaAtikokan,
    #[serde(rename = "America/Atka")]
    AmericaAtka,
    #[serde(rename = "America/Bahia")]
    AmericaBahia,
    #[serde(rename = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "America/Barbados")]
    AmericaBarbados,
    #[serde(rename = "America/Belem")]
    AmericaBelem,
    #[serde(rename = "America/Belize")]
    AmericaBelize,
    #[serde(rename = "America/Blanc-Sablon")]
    AmericaBlancSablon,
    #[serde(rename = "America/Boa_Vista")]
    AmericaBoaVista,
    #[serde(rename = "America/Bogota")]
    AmericaBogota,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Buenos_Aires")]
    AmericaBuenosAires,
    #[serde(rename = "America/Cambridge_Bay")]
    AmericaCambridgeBay,
    #[serde(rename = "America/Campo_Grande")]
    AmericaCampoGrande,
    #[serde(rename = "America/Cancun")]
    AmericaCancun,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Catamarca")]
    AmericaCatamarca,
    #[serde(rename = "America/Cayenne")]
    AmericaCayenne,
    #[serde(rename = "America/Cayman")]
    AmericaCayman,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Chihuahua")]
    AmericaChihuahua,
    #[serde(rename = "America/Coral_Harbour")]
    AmericaCoralHarbour,
    #[serde(rename = "America/Cordoba")]
    AmericaCordoba,
    #[serde(rename = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(rename = "America/Creston")]
    AmericaCreston,
    #[serde(rename = "America/Cuiaba")]
    AmericaCuiaba,
    #[serde(rename = "America/Curacao")]
    AmericaCuracao,
    #[serde(rename = "America/Danmarkshavn")]
    AmericaDanmarkshavn,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Dawson_Creek")]
    AmericaDawsonCreek,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Dominica")]
    AmericaDominica,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/Eirunepe")]
    AmericaEirunepe,
    #[serde(rename = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(rename = "America/Ensenada")]
    AmericaEnsenada,
    #[serde(rename = "America/Fort_Nelson")]
    AmericaFortNelson,
    #[serde(rename = "America/Fort_Wayne")]
    AmericaFortWayne,
    #[serde(rename = "America/Fortaleza")]
    AmericaFortaleza,
    #[serde(rename = "America/Glace_Bay")]
    AmericaGlaceBay,
    #[serde(rename = "America/Godthab")]
    AmericaGodthab,
    #[serde(rename = "America/Goose_Bay")]
    AmericaGooseBay,
    #[serde(rename = "America/Grand_Turk")]
    AmericaGrandTurk,
    #[serde(rename = "America/Grenada")]
    AmericaGrenada,
    #[serde(rename = "America/Guadeloupe")]
    AmericaGuadeloupe,
    #[serde(rename = "America/Guatemala")]
    AmericaGuatemala,
    #[serde(rename = "America/Guayaquil")]
    AmericaGuayaquil,
    #[serde(rename = "America/Guyana")]
    AmericaGuyana,
    #[serde(rename = "America/Halifax")]
    AmericaHalifax,
    #[serde(rename = "America/Havana")]
    AmericaHavana,
    #[serde(rename = "America/Hermosillo")]
    AmericaHermosillo,
    #[serde(rename = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "America/Indiana/Knox")]
    AmericaIndianaKnox,
    #[serde(rename = "America/Indiana/Marengo")]
    AmericaIndianaMarengo,
    #[serde(rename = "America/Indiana/Petersburg")]
    AmericaIndianaPetersburg,
    #[serde(rename = "America/Indiana/Tell_City")]
    AmericaIndianaTellCity,
    #[serde(rename = "America/Indiana/Vevay")]
    AmericaIndianaVevay,
    #[serde(rename = "America/Indiana/Vincennes")]
    AmericaIndianaVincennes,
    #[serde(rename = "America/Indiana/Winamac")]
    AmericaIndianaWinamac,
    #[serde(rename = "America/Indianapolis")]
    AmericaIndianapolis,
    #[serde(rename = "America/Inuvik")]
    AmericaInuvik,
    #[serde(rename = "America/Iqaluit")]
    AmericaIqaluit,
    #[serde(rename = "America/Jamaica")]
    AmericaJamaica,
    #[serde(rename = "America/Jujuy")]
    AmericaJujuy,
    #[serde(rename = "America/Juneau")]
    AmericaJuneau,
    #[serde(rename = "America/Kentucky/Louisville")]
    AmericaKentuckyLouisville,
    #[serde(rename = "America/Kentucky/Monticello")]
    AmericaKentuckyMonticello,
    #[serde(rename = "America/Knox_IN")]
    AmericaKnoxIn,
    #[serde(rename = "America/Kralendijk")]
    AmericaKralendijk,
    #[serde(rename = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Louisville")]
    AmericaLouisville,
    #[serde(rename = "America/Lower_Princes")]
    AmericaLowerPrinces,
    #[serde(rename = "America/Maceio")]
    AmericaMaceio,
    #[serde(rename = "America/Managua")]
    AmericaManagua,
    #[serde(rename = "America/Manaus")]
    AmericaManaus,
    #[serde(rename = "America/Marigot")]
    AmericaMarigot,
    #[serde(rename = "America/Martinique")]
    AmericaMartinique,
    #[serde(rename = "America/Matamoros")]
    AmericaMatamoros,
    #[serde(rename = "America/Mazatlan")]
    AmericaMazatlan,
    #[serde(rename = "America/Mendoza")]
    AmericaMendoza,
    #[serde(rename = "America/Menominee")]
    AmericaMenominee,
    #[serde(rename = "America/Merida")]
    AmericaMerida,
    #[serde(rename = "America/Metlakatla")]
    AmericaMetlakatla,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/Miquelon")]
    AmericaMiquelon,
    #[serde(rename = "America/Moncton")]
    AmericaMoncton,
    #[serde(rename = "America/Monterrey")]
    AmericaMonterrey,
    #[serde(rename = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(rename = "America/Montreal")]
    AmericaMontreal,
    #[serde(rename = "America/Montserrat")]
    AmericaMontserrat,
    #[serde(rename = "America/Nassau")]
    AmericaNassau,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Nipigon")]
    AmericaNipigon,
    #[serde(rename = "America/Nome")]
    AmericaNome,
    #[serde(rename = "America/Noronha")]
    AmericaNoronha,
    #[serde(rename = "America/North_Dakota/Beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(rename = "America/North_Dakota/Center")]
    AmericaNorthDakotaCenter,
    #[serde(rename = "America/North_Dakota/New_Salem")]
    AmericaNorthDakotaNewSalem,
    #[serde(rename = "America/Nuuk")]
    AmericaNuuk,
    #[serde(rename = "America/Ojinaga")]
    AmericaOjinaga,
    #[serde(rename = "America/Panama")]
    AmericaPanama,
    #[serde(rename = "America/Pangnirtung")]
    AmericaPangnirtung,
    #[serde(rename = "America/Paramaribo")]
    AmericaParamaribo,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Port-au-Prince")]
    AmericaPortAuPrince,
    #[serde(rename = "America/Port_of_Spain")]
    AmericaPortOfSpain,
    #[serde(rename = "America/Porto_Acre")]
    AmericaPortoAcre,
    #[serde(rename = "America/Porto_Velho")]
    AmericaPortoVelho,
    #[serde(rename = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(rename = "America/Punta_Arenas")]
    AmericaPuntaArenas,
    #[serde(rename = "America/Rainy_River")]
    AmericaRainyRiver,
    #[serde(rename = "America/Rankin_Inlet")]
    AmericaRankinInlet,
    #[serde(rename = "America/Recife")]
    AmericaRecife,
    #[serde(rename = "America/Regina")]
    AmericaRegina,
    #[serde(rename = "America/Resolute")]
    AmericaResolute,
    #[serde(rename = "America/Rio_Branco")]
    AmericaRioBranco,
    #[serde(rename = "America/Rosario")]
    AmericaRosario,
    #[serde(rename = "America/Santa_Isabel")]
    AmericaSantaIsabel,
    #[serde(rename = "America/Santarem")]
    AmericaSantarem,
    #[serde(rename = "America/Santiago")]
    AmericaSantiago,
    #[serde(rename = "America/Santo_Domingo")]
    AmericaSantoDomingo,
    #[serde(rename = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "America/Scoresbysund")]
    AmericaScoresbysund,
    #[serde(rename = "America/Shiprock")]
    AmericaShiprock,
    #[serde(rename = "America/Sitka")]
    AmericaSitka,
    #[serde(rename = "America/St_Barthelemy")]
    AmericaStBarthelemy,
    #[serde(rename = "America/St_Johns")]
    AmericaStJohns,
    #[serde(rename = "America/St_Kitts")]
    AmericaStKitts,
    #[serde(rename = "America/St_Lucia")]
    AmericaStLucia,
    #[serde(rename = "America/St_Thomas")]
    AmericaStThomas,
    #[serde(rename = "America/St_Vincent")]
    AmericaStVincent,
    #[serde(rename = "America/Swift_Current")]
    AmericaSwiftCurrent,
    #[serde(rename = "America/Tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(rename = "America/Thule")]
    AmericaThule,
    #[serde(rename = "America/Thunder_Bay")]
    AmericaThunderBay,
    #[serde(rename = "America/Tijuana")]
    AmericaTijuana,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Tortola")]
    AmericaTortola,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "America/Virgin")]
    AmericaVirgin,
    #[serde(rename = "America/Whitehorse")]
    AmericaWhitehorse,
    #[serde(rename = "America/Winnipeg")]
    AmericaWinnipeg,
    #[serde(rename = "America/Yakutat")]
    AmericaYakutat,
    #[serde(rename = "America/Yellowknife")]
    AmericaYellowknife,
    #[serde(rename = "Antarctica/Casey")]
    AntarcticaCasey,
    #[serde(rename = "Antarctica/Davis")]
    AntarcticaDavis,
    #[serde(rename = "Antarctica/DumontDUrville")]
    AntarcticaDumontDUrville,
    #[serde(rename = "Antarctica/Macquarie")]
    AntarcticaMacquarie,
    #[serde(rename = "Antarctica/Mawson")]
    AntarcticaMawson,
    #[serde(rename = "Antarctica/McMurdo")]
    AntarcticaMcMurdo,
    #[serde(rename = "Antarctica/Palmer")]
    AntarcticaPalmer,
    #[serde(rename = "Antarctica/Rothera")]
    AntarcticaRothera,
    #[serde(rename = "Antarctica/South_Pole")]
    AntarcticaSouthPole,
    #[serde(rename = "Antarctica/Syowa")]
    AntarcticaSyowa,
    #[serde(rename = "Antarctica/Troll")]
    AntarcticaTroll,
    #[serde(rename = "Antarctica/Vostok")]
    AntarcticaVostok,
    #[serde(rename = "Arctic/Longyearbyen")]
    ArcticLongyearbyen,
    #[serde(rename = "Asia/Aden")]
    AsiaAden,
    #[serde(rename = "Asia/Almaty")]
    AsiaAlmaty,
    #[serde(rename = "Asia/Amman")]
    AsiaAmman,
    #[serde(rename = "Asia/Anadyr")]
    AsiaAnadyr,
    #[serde(rename = "Asia/Aqtau")]
    AsiaAqtau,
    #[serde(rename = "Asia/Aqtobe")]
    AsiaAqtobe,
    #[serde(rename = "Asia/Ashgabat")]
    AsiaAshgabat,
    #[serde(rename = "Asia/Ashkhabad")]
    AsiaAshkhabad,
    #[serde(rename = "Asia/Atyrau")]
    AsiaAtyrau,
    #[serde(rename = "Asia/Baghdad")]
    AsiaBaghdad,
    #[serde(rename = "Asia/Bahrain")]
    AsiaBahrain,
    #[serde(rename = "Asia/Baku")]
    AsiaBaku,
    #[serde(rename = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(rename = "Asia/Barnaul")]
    AsiaBarnaul,
    #[serde(rename = "Asia/Beirut")]
    AsiaBeirut,
    #[serde(rename = "Asia/Bishkek")]
    AsiaBishkek,
    #[serde(rename = "Asia/Brunei")]
    AsiaBrunei,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Chita")]
    AsiaChita,
    #[serde(rename = "Asia/Choibalsan")]
    AsiaChoibalsan,
    #[serde(rename = "Asia/Chongqing")]
    AsiaChongqing,
    #[serde(rename = "Asia/Chungking")]
    AsiaChungking,
    #[serde(rename = "Asia/Colombo")]
    AsiaColombo,
    #[serde(rename = "Asia/Dacca")]
    AsiaDacca,
    #[serde(rename = "Asia/Damascus")]
    AsiaDamascus,
    #[serde(rename = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(rename = "Asia/Dili")]
    AsiaDili,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Dushanbe")]
    AsiaDushanbe,
    #[serde(rename = "Asia/Famagusta")]
    AsiaFamagusta,
    #[serde(rename = "Asia/Gaza")]
    AsiaGaza,
    #[serde(rename = "Asia/Harbin")]
    AsiaHarbin,
    #[serde(rename = "Asia/Hebron")]
    AsiaHebron,
    #[serde(rename = "Asia/Ho_Chi_Minh")]
    AsiaHoChiMinh,
    #[serde(rename = "Asia/Hong_Kong")]
    AsiaHongKong,
    #[serde(rename = "Asia/Hovd")]
    AsiaHovd,
    #[serde(rename = "Asia/Irkutsk")]
    AsiaIrkutsk,
    #[serde(rename = "Asia/Istanbul")]
    AsiaIstanbul,
    #[serde(rename = "Asia/Jakarta")]
    AsiaJakarta,
    #[serde(rename = "Asia/Jayapura")]
    AsiaJayapura,
    #[serde(rename = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Kamchatka")]
    AsiaKamchatka,
    #[serde(rename = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(rename = "Asia/Kashgar")]
    AsiaKashgar,
    #[serde(rename = "Asia/Kathmandu")]
    AsiaKathmandu,
    #[serde(rename = "Asia/Katmandu")]
    AsiaKatmandu,
    #[serde(rename = "Asia/Khandyga")]
    AsiaKhandyga,
    #[serde(rename = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(rename = "Asia/Krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(rename = "Asia/Kuala_Lumpur")]
    AsiaKualaLumpur,
    #[serde(rename = "Asia/Kuching")]
    AsiaKuching,
    #[serde(rename = "Asia/Kuwait")]
    AsiaKuwait,
    #[serde(rename = "Asia/Macao")]
    AsiaMacao,
    #[serde(rename = "Asia/Macau")]
    AsiaMacau,
    #[serde(rename = "Asia/Magadan")]
    AsiaMagadan,
    #[serde(rename = "Asia/Makassar")]
    AsiaMakassar,
    #[serde(rename = "Asia/Manila")]
    AsiaManila,
    #[serde(rename = "Asia/Muscat")]
    AsiaMuscat,
    #[serde(rename = "Asia/Nicosia")]
    AsiaNicosia,
    #[serde(rename = "Asia/Novokuznetsk")]
    AsiaNovokuznetsk,
    #[serde(rename = "Asia/Novosibirsk")]
    AsiaNovosibirsk,
    #[serde(rename = "Asia/Omsk")]
    AsiaOmsk,
    #[serde(rename = "Asia/Oral")]
    AsiaOral,
    #[serde(rename = "Asia/Phnom_Penh")]
    AsiaPhnomPenh,
    #[serde(rename = "Asia/Pontianak")]
    AsiaPontianak,
    #[serde(rename = "Asia/Pyongyang")]
    AsiaPyongyang,
    #[serde(rename = "Asia/Qatar")]
    AsiaQatar,
    #[serde(rename = "Asia/Qostanay")]
    AsiaQostanay,
    #[serde(rename = "Asia/Qyzylorda")]
    AsiaQyzylorda,
    #[serde(rename = "Asia/Rangoon")]
    AsiaRangoon,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Saigon")]
    AsiaSaigon,
    #[serde(rename = "Asia/Sakhalin")]
    AsiaSakhalin,
    #[serde(rename = "Asia/Samarkand")]
    AsiaSamarkand,
    #[serde(rename = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(rename = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(rename = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(rename = "Asia/Srednekolymsk")]
    AsiaSrednekolymsk,
    #[serde(rename = "Asia/Taipei")]
    AsiaTaipei,
    #[serde(rename = "Asia/Tashkent")]
    AsiaTashkent,
    #[serde(rename = "Asia/Tbilisi")]
    AsiaTbilisi,
    #[serde(rename = "Asia/Tehran")]
    AsiaTehran,
    #[serde(rename = "Asia/Tel_Aviv")]
    AsiaTelAviv,
    #[serde(rename = "Asia/Thimbu")]
    AsiaThimbu,
    #[serde(rename = "Asia/Thimphu")]
    AsiaThimphu,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Asia/Tomsk")]
    AsiaTomsk,
    #[serde(rename = "Asia/Ujung_Pandang")]
    AsiaUjungPandang,
    #[serde(rename = "Asia/Ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(rename = "Asia/Ulan_Bator")]
    AsiaUlanBator,
    #[serde(rename = "Asia/Urumqi")]
    AsiaUrumqi,
    #[serde(rename = "Asia/Ust-Nera")]
    AsiaUstNera,
    #[serde(rename = "Asia/Vientiane")]
    AsiaVientiane,
    #[serde(rename = "Asia/Vladivostok")]
    AsiaVladivostok,
    #[serde(rename = "Asia/Yakutsk")]
    AsiaYakutsk,
    #[serde(rename = "Asia/Yangon")]
    AsiaYangon,
    #[serde(rename = "Asia/Yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(rename = "Asia/Yerevan")]
    AsiaYerevan,
    #[serde(rename = "Atlantic/Azores")]
    AtlanticAzores,
    #[serde(rename = "Atlantic/Bermuda")]
    AtlanticBermuda,
    #[serde(rename = "Atlantic/Canary")]
    AtlanticCanary,
    #[serde(rename = "Atlantic/Cape_Verde")]
    AtlanticCapeVerde,
    #[serde(rename = "Atlantic/Faeroe")]
    AtlanticFaeroe,
    #[serde(rename = "Atlantic/Faroe")]
    AtlanticFaroe,
    #[serde(rename = "Atlantic/Jan_Mayen")]
    AtlanticJanMayen,
    #[serde(rename = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(rename = "Atlantic/Reykjavik")]
    AtlanticReykjavik,
    #[serde(rename = "Atlantic/South_Georgia")]
    AtlanticSouthGeorgia,
    #[serde(rename = "Atlantic/St_Helena")]
    AtlanticStHelena,
    #[serde(rename = "Atlantic/Stanley")]
    AtlanticStanley,
    #[serde(rename = "Australia/ACT")]
    AustraliaACT,
    #[serde(rename = "Australia/Adelaide")]
    AustraliaAdelaide,
    #[serde(rename = "Australia/Brisbane")]
    AustraliaBrisbane,
    #[serde(rename = "Australia/Broken_Hill")]
    AustraliaBrokenHill,
    #[serde(rename = "Australia/Canberra")]
    AustraliaCanberra,
    #[serde(rename = "Australia/Currie")]
    AustraliaCurrie,
    #[serde(rename = "Australia/Darwin")]
    AustraliaDarwin,
    #[serde(rename = "Australia/Eucla")]
    AustraliaEucla,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/LHI")]
    AustraliaLHI,
    #[serde(rename = "Australia/Lindeman")]
    AustraliaLindeman,
    #[serde(rename = "Australia/Lord_Howe")]
    AustraliaLordHowe,
    #[serde(rename = "Australia/Melbourne")]
    AustraliaMelbourne,
    #[serde(rename = "Australia/North")]
    AustraliaNorth,
    #[serde(rename = "Australia/NSW")]
    AustraliaNSW,
    #[serde(rename = "Australia/Perth")]
    AustraliaPerth,
    #[serde(rename = "Australia/Queensland")]
    AustraliaQueensland,
    #[serde(rename = "Australia/South")]
    AustraliaSouth,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(rename = "Australia/Tasmania")]
    AustraliaTasmania,
    #[serde(rename = "Australia/Victoria")]
    AustraliaVictoria,
    #[serde(rename = "Australia/West")]
    AustraliaWest,
    #[serde(rename = "Australia/Yancowinna")]
    AustraliaYancowinna,
    #[serde(rename = "Brazil/Acre")]
    BrazilAcre,
    #[serde(rename = "Brazil/DeNoronha")]
    BrazilDeNoronha,
    #[serde(rename = "Brazil/East")]
    BrazilEast,
    #[serde(rename = "Brazil/West")]
    BrazilWest,
    #[serde(rename = "Canada/Atlantic")]
    CanadaAtlantic,
    #[serde(rename = "Canada/Central")]
    CanadaCentral,
    #[serde(rename = "Canada/Eastern")]
    CanadaEastern,
    #[serde(rename = "Canada/Mountain")]
    CanadaMountain,
    #[serde(rename = "Canada/Newfoundland")]
    CanadaNewfoundland,
    #[serde(rename = "Canada/Pacific")]
    CanadaPacific,
    #[serde(rename = "Canada/Saskatchewan")]
    CanadaSaskatchewan,
    #[serde(rename = "Canada/Yukon")]
    CanadaYukon,
    #[serde(rename = "CET")]
    CET,
    #[serde(rename = "Chile/Continental")]
    ChileContinental,
    #[serde(rename = "Chile/EasterIsland")]
    ChileEasterIsland,
    #[serde(rename = "CST6CDT")]
    CST6CDT,
    #[serde(rename = "Cuba")]
    Cuba,
    #[serde(rename = "EET")]
    EET,
    #[serde(rename = "Egypt")]
    Egypt,
    #[serde(rename = "Eire")]
    Eire,
    #[serde(rename = "EST")]
    EST,
    #[serde(rename = "EST5EDT")]
    EST5EDT,
    #[serde(rename = "Etc/GMT")]
    EtcGMT,
    #[serde(rename = "Etc/GMT+0")]
    EtcGMTPlus0,
    #[serde(rename = "Etc/GMT+1")]
    EtcGMTPlus1,
    #[serde(rename = "Etc/GMT+10")]
    EtcGMTPlus10,
    #[serde(rename = "Etc/GMT+11")]
    EtcGMTPlus11,
    #[serde(rename = "Etc/GMT+12")]
    EtcGMTPlus12,
    #[serde(rename = "Etc/GMT+2")]
    EtcGMTPlus2,
    #[serde(rename = "Etc/GMT+3")]
    EtcGMTPlus3,
    #[serde(rename = "Etc/GMT+4")]
    EtcGMTPlus4,
    #[serde(rename = "Etc/GMT+5")]
    EtcGMTPlus5,
    #[serde(rename = "Etc/GMT+6")]
    EtcGMTPlus6,
    #[serde(rename = "Etc/GMT+7")]
    EtcGMTPlus7,
    #[serde(rename = "Etc/GMT+8")]
    EtcGMTPlus8,
    #[serde(rename = "Etc/GMT+9")]
    EtcGMTPlus9,
    #[serde(rename = "Etc/GMT-0")]
    EtcGMTMinus0,
    #[serde(rename = "Etc/GMT-1")]
    EtcGMTMinus1,
    #[serde(rename = "Etc/GMT-10")]
    EtcGMTMinus10,
    #[serde(rename = "Etc/GMT-11")]
    EtcGMTMinus11,
    #[serde(rename = "Etc/GMT-12")]
    EtcGMTMinus12,
    #[serde(rename = "Etc/GMT-13")]
    EtcGMTMinus13,
    #[serde(rename = "Etc/GMT-14")]
    EtcGMTMinus14,
    #[serde(rename = "Etc/GMT-2")]
    EtcGMTMinus2,
    #[serde(rename = "Etc/GMT-3")]
    EtcGMTMinus3,
    #[serde(rename = "Etc/GMT-4")]
    EtcGMTMinus4,
    #[serde(rename = "Etc/GMT-5")]
    EtcGMTMinus5,
    #[serde(rename = "Etc/GMT-6")]
    EtcGMTMinus6,
    #[serde(rename = "Etc/GMT-7")]
    EtcGMTMinus7,
    #[serde(rename = "Etc/GMT-8")]
    EtcGMTMinus8,
    #[serde(rename = "Etc/GMT-9")]
    EtcGMTMinus9,
    #[serde(rename = "Etc/GMT0")]
    EtcGMT0,
    #[serde(rename = "Etc/Greenwich")]
    EtcGreenwich,
    #[serde(rename = "Etc/UCT")]
    EtcUCT,
    #[serde(rename = "Etc/Universal")]
    EtcUniversal,
    #[serde(rename = "Etc/UTC")]
    EtcUTC,
    #[serde(rename = "Etc/Zulu")]
    EtcZulu,
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Andorra")]
    EuropeAndorra,
    #[serde(rename = "Europe/Astrakhan")]
    EuropeAstrakhan,
    #[serde(rename = "Europe/Athens")]
    EuropeAthens,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Bratislava")]
    EuropeBratislava,
    #[serde(rename = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(rename = "Europe/Bucharest")]
    EuropeBucharest,
    #[serde(rename = "Europe/Budapest")]
    EuropeBudapest,
    #[serde(rename = "Europe/Busingen")]
    EuropeBusingen,
    #[serde(rename = "Europe/Chisinau")]
    EuropeChisinau,
    #[serde(rename = "Europe/Copenhagen")]
    EuropeCopenhagen,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/Gibraltar")]
    EuropeGibraltar,
    #[serde(rename = "Europe/Guernsey")]
    EuropeGuernsey,
    #[serde(rename = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(rename = "Europe/Isle_of_Man")]
    EuropeIsleOfMan,
    #[serde(rename = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(rename = "Europe/Jersey")]
    EuropeJersey,
    #[serde(rename = "Europe/Kaliningrad")]
    EuropeKaliningrad,
    #[serde(rename = "Europe/Kiev")]
    EuropeKiev,
    #[serde(rename = "Europe/Kirov")]
    EuropeKirov,
    #[serde(rename = "Europe/Lisbon")]
    EuropeLisbon,
    #[serde(rename = "Europe/Ljubljana")]
    EuropeLjubljana,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(rename = "Europe/Malta")]
    EuropeMalta,
    #[serde(rename = "Europe/Mariehamn")]
    EuropeMariehamn,
    #[serde(rename = "Europe/Minsk")]
    EuropeMinsk,
    #[serde(rename = "Europe/Monaco")]
    EuropeMonaco,
    #[serde(rename = "Europe/Moscow")]
    EuropeMoscow,
    #[serde(rename = "Europe/Nicosia")]
    EuropeNicosia,
    #[serde(rename = "Europe/Oslo")]
    EuropeOslo,
    #[serde(rename = "Europe/Paris")]
    EuropeParis,
    #[serde(rename = "Europe/Podgorica")]
    EuropePodgorica,
    #[serde(rename = "Europe/Prague")]
    EuropePrague,
    #[serde(rename = "Europe/Riga")]
    EuropeRiga,
    #[serde(rename = "Europe/Rome")]
    EuropeRome,
    #[serde(rename = "Europe/Samara")]
    EuropeSamara,
    #[serde(rename = "Europe/San_Marino")]
    EuropeSanMarino,
    #[serde(rename = "Europe/Sarajevo")]
    EuropeSarajevo,
    #[serde(rename = "Europe/Saratov")]
    EuropeSaratov,
    #[serde(rename = "Europe/Simferopol")]
    EuropeSimferopol,
    #[serde(rename = "Europe/Skopje")]
    EuropeSkopje,
    #[serde(rename = "Europe/Sofia")]
    EuropeSofia,
    #[serde(rename = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(rename = "Europe/Tallinn")]
    EuropeTallinn,
    #[serde(rename = "Europe/Tirane")]
    EuropeTirane,
    #[serde(rename = "Europe/Tiraspol")]
    EuropeTiraspol,
    #[serde(rename = "Europe/Ulyanovsk")]
    EuropeUlyanovsk,
    #[serde(rename = "Europe/Uzhgorod")]
    EuropeUzhgorod,
    #[serde(rename = "Europe/Vaduz")]
    EuropeVaduz,
    #[serde(rename = "Europe/Vatican")]
    EuropeVatican,
    #[serde(rename = "Europe/Vienna")]
    EuropeVienna,
    #[serde(rename = "Europe/Vilnius")]
    EuropeVilnius,
    #[serde(rename = "Europe/Volgograd")]
    EuropeVolgograd,
    #[serde(rename = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(rename = "Europe/Zagreb")]
    EuropeZagreb,
    #[serde(rename = "Europe/Zaporozhye")]
    EuropeZaporozhye,
    #[serde(rename = "Europe/Zurich")]
    EuropeZurich,
    #[serde(rename = "Factory")]
    Factory,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GB-Eire")]
    GBEire,
    #[serde(rename = "GMT")]
    GMT,
    #[serde(rename = "GMT+0")]
    GMTPlus0,
    #[serde(rename = "GMT-0")]
    GMTMinus0,
    #[serde(rename = "GMT0")]
    GMT0,
    #[serde(rename = "Greenwich")]
    Greenwich,
    #[serde(rename = "Hongkong")]
    Hongkong,
    #[serde(rename = "HST")]
    HST,
    #[serde(rename = "Iceland")]
    Iceland,
    #[serde(rename = "Indian/Antananarivo")]
    IndianAntananarivo,
    #[serde(rename = "Indian/Chagos")]
    IndianChagos,
    #[serde(rename = "Indian/Christmas")]
    IndianChristmas,
    #[serde(rename = "Indian/Cocos")]
    IndianCocos,
    #[serde(rename = "Indian/Comoro")]
    IndianComoro,
    #[serde(rename = "Indian/Kerguelen")]
    IndianKerguelen,
    #[serde(rename = "Indian/Mahe")]
    IndianMahe,
    #[serde(rename = "Indian/Maldives")]
    IndianMaldives,
    #[serde(rename = "Indian/Mauritius")]
    IndianMauritius,
    #[serde(rename = "Indian/Mayotte")]
    IndianMayotte,
    #[serde(rename = "Indian/Reunion")]
    IndianReunion,
    #[serde(rename = "Iran")]
    Iran,
    #[serde(rename = "Israel")]
    Israel,
    #[serde(rename = "Jamaica")]
    Jamaica,
    #[serde(rename = "Japan")]
    Japan,
    #[serde(rename = "Kwajalein")]
    Kwajalein,
    #[serde(rename = "Libya")]
    Libya,
    #[serde(rename = "MET")]
    MET,
    #[serde(rename = "Mexico/BajaNorte")]
    MexicoBajaNorte,
    #[serde(rename = "Mexico/BajaSur")]
    MexicoBajaSur,
    #[serde(rename = "Mexico/General")]
    MexicoGeneral,
    #[serde(rename = "MST")]
    MST,
    #[serde(rename = "MST7MDT")]
    MST7MDT,
    #[serde(rename = "Navajo")]
    Navajo,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "NZ-CHAT")]
    NzChat,
    #[serde(rename = "Pacific/Apia")]
    PacificApia,
    #[serde(rename = "Pacific/Auckland")]
    PacificAuckland,
    #[serde(rename = "Pacific/Bougainville")]
    PacificBougainville,
    #[serde(rename = "Pacific/Chatham")]
    PacificChatham,
    #[serde(rename = "Pacific/Chuuk")]
    PacificChuuk,
    #[serde(rename = "Pacific/Easter")]
    PacificEaster,
    #[serde(rename = "Pacific/Efate")]
    PacificEfate,
    #[serde(rename = "Pacific/Enderbury")]
    PacificEnderbury,
    #[serde(rename = "Pacific/Fakaofo")]
    PacificFakaofo,
    #[serde(rename = "Pacific/Fiji")]
    PacificFiji,
    #[serde(rename = "Pacific/Funafuti")]
    PacificFunafuti,
    #[serde(rename = "Pacific/Galapagos")]
    PacificGalapagos,
    #[serde(rename = "Pacific/Gambier")]
    PacificGambier,
    #[serde(rename = "Pacific/Guadalcanal")]
    PacificGuadalcanal,
    #[serde(rename = "Pacific/Guam")]
    PacificGuam,
    #[serde(rename = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(rename = "Pacific/Johnston")]
    PacificJohnston,
    #[serde(rename = "Pacific/Kanton")]
    PacificKanton,
    #[serde(rename = "Pacific/Kiritimati")]
    PacificKiritimati,
    #[serde(rename = "Pacific/Kosrae")]
    PacificKosrae,
    #[serde(rename = "Pacific/Kwajalein")]
    PacificKwajalein,
    #[serde(rename = "Pacific/Majuro")]
    PacificMajuro,
    #[serde(rename = "Pacific/Marquesas")]
    PacificMarquesas,
    #[serde(rename = "Pacific/Midway")]
    PacificMidway,
    #[serde(rename = "Pacific/Nauru")]
    PacificNauru,
    #[serde(rename = "Pacific/Niue")]
    PacificNiue,
    #[serde(rename = "Pacific/Norfolk")]
    PacificNorfolk,
    #[serde(rename = "Pacific/Noumea")]
    PacificNoumea,
    #[serde(rename = "Pacific/Pago_Pago")]
    PacificPagoPago,
    #[serde(rename = "Pacific/Palau")]
    PacificPalau,
    #[serde(rename = "Pacific/Pitcairn")]
    PacificPitcairn,
    #[serde(rename = "Pacific/Pohnpei")]
    PacificPohnpei,
    #[serde(rename = "Pacific/Ponape")]
    PacificPonape,
    #[serde(rename = "Pacific/Port_Moresby")]
    PacificPortMoresby,
    #[serde(rename = "Pacific/Rarotonga")]
    PacificRarotonga,
    #[serde(rename = "Pacific/Saipan")]
    PacificSaipan,
    #[serde(rename = "Pacific/Samoa")]
    PacificSamoa,
    #[serde(rename = "Pacific/Tahiti")]
    PacificTahiti,
    #[serde(rename = "Pacific/Tarawa")]
    PacificTarawa,
    #[serde(rename = "Pacific/Tongatapu")]
    PacificTongatapu,
    #[serde(rename = "Pacific/Truk")]
    PacificTruk,
    #[serde(rename = "Pacific/Wake")]
    PacificWake,
    #[serde(rename = "Pacific/Wallis")]
    PacificWallis,
    #[serde(rename = "Pacific/Yap")]
    PacificYap,
    #[serde(rename = "Poland")]
    Poland,
    #[serde(rename = "Portugal")]
    Portugal,
    #[serde(rename = "PRC")]
    Prc,
    #[serde(rename = "PST8PDT")]
    Pst8Pdt,
    #[serde(rename = "ROC")]
    Roc,
    #[serde(rename = "ROK")]
    Rok,
    #[serde(rename = "Singapore")]
    Singapore,
    #[serde(rename = "Turkey")]
    Turkey,
    #[serde(rename = "UCT")]
    Uct,
    #[serde(rename = "Universal")]
    Universal,
    #[serde(rename = "US/Alaska")]
    UsAlaska,
    #[serde(rename = "US/Aleutian")]
    UsAleutian,
    #[serde(rename = "US/Arizona")]
    UsArizona,
    #[serde(rename = "US/Central")]
    UsCentral,
    #[serde(rename = "US/East-Indiana")]
    UsEastIndiana,
    #[serde(rename = "US/Eastern")]
    UsEastern,
    #[serde(rename = "US/Hawaii")]
    UsHawaii,
    #[serde(rename = "US/Indiana-Starke")]
    UsIndianaStarke,
    #[serde(rename = "US/Michigan")]
    UsMichigan,
    #[serde(rename = "US/Mountain")]
    UsMountain,
    #[serde(rename = "US/Pacific")]
    UsPacific,
    #[serde(rename = "US/Samoa")]
    UsSamoa,
    #[serde(rename = "UTC")]
    Utc,
    #[serde(rename = "W-SU")]
    WSu,
    #[serde(rename = "WET")]
    Wet,
    #[serde(rename = "Zulu")]
    Zulu,
}
