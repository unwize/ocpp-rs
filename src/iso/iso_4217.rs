//! # ISO 4217 Currency Library
//!
//! A Rust library for working with ISO 4217 currency codes.
//! Provides functionality to enumerate all currencies and validate currency identifiers.
//! Data current as of January 2024.

use std::collections::HashMap;
use std::fmt;

/// Represents an ISO 4217 currency
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    /// Three-letter alphabetic code (e.g., "USD")
    pub code: &'static str,
    /// Numeric code (e.g., 840 for USD)
    pub numeric: u16,
    /// Number of minor units (decimal places)
    pub minor_units: Option<u8>,
    /// Currency name
    pub name: &'static str,
    /// Countries/territories using this currency
    pub countries: &'static [&'static str],
    /// Whether this currency is currently active
    pub is_active: bool,
}

impl Currency {
    /// Creates a new Currency instance
    pub const fn new(
        code: &'static str,
        numeric: u16,
        minor_units: Option<u8>,
        name: &'static str,
        countries: &'static [&'static str],
        is_active: bool,
    ) -> Self {
        Self {
            code,
            numeric,
            minor_units,
            name,
            countries,
            is_active,
        }
    }

    /// Returns true if this currency has minor units (fractional parts)
    pub fn has_minor_units(&self) -> bool {
        self.minor_units.is_some() && self.minor_units != Some(0)
    }

    /// Returns the number of decimal places for this currency
    pub fn decimal_places(&self) -> u8 {
        self.minor_units.unwrap_or(0)
    }

    /// Returns true if this is a precious metal currency
    pub fn is_precious_metal(&self) -> bool {
        matches!(self.code, "XAU" | "XAG" | "XPT" | "XPD")
    }

    /// Returns true if this is a supranational currency
    pub fn is_supranational(&self) -> bool {
        matches!(self.code, "XCD" | "XAF" | "XOF" | "XPF" | "XDR" | "EUR")
    }

    /// Returns true if this is a testing or special purpose code
    pub fn is_special_purpose(&self) -> bool {
        matches!(self.code, "XTS" | "XXX")
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.code, self.name)
    }
}

/// ISO 4217 currency registry
pub struct CurrencyRegistry {
    by_code: HashMap<&'static str, &'static Currency>,
    by_numeric: HashMap<u16, &'static Currency>,
    currencies: &'static [Currency],
}

impl CurrencyRegistry {
    /// Creates a new currency registry with all ISO 4217 currencies
    pub fn new() -> Self {
        let currencies = ALL_CURRENCIES;
        let mut by_code = HashMap::new();
        let mut by_numeric = HashMap::new();

        for currency in currencies {
            by_code.insert(currency.code, currency);
            by_numeric.insert(currency.numeric, currency);
        }

        Self {
            by_code,
            by_numeric,
            currencies,
        }
    }

    /// Validates if a given alphabetic code is a valid currency
    pub fn is_valid_code(&self, code: &str) -> bool {
        // Validate format first
        if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
            return false;
        }
        self.by_code.contains_key(code)
    }

    /// Validates if a given numeric code is a valid currency
    pub fn is_valid_numeric(&self, numeric: u16) -> bool {
        // Valid numeric codes are in range 1-999
        if numeric == 0 || numeric > 999 {
            return false;
        }
        self.by_numeric.contains_key(&numeric)
    }

    /// Gets a currency by its alphabetic code
    pub fn get_by_code(&self, code: &str) -> Option<&Currency> {
        self.by_code.get(code).copied()
    }

    /// Gets a currency by its numeric code
    pub fn get_by_numeric(&self, numeric: u16) -> Option<&Currency> {
        self.by_numeric.get(&numeric).copied()
    }

    /// Returns an iterator over all currencies
    pub fn iter(&self) -> impl Iterator<Item = &Currency> {
        self.currencies.iter()
    }

    /// Returns an iterator over only active currencies
    pub fn active_currencies(&self) -> impl Iterator<Item = &Currency> {
        self.currencies.iter().filter(|c| c.is_active)
    }

    /// Returns all currencies as a slice
    pub fn all(&self) -> &[Currency] {
        self.currencies
    }

    /// Returns the number of currencies in the registry
    pub fn len(&self) -> usize {
        self.currencies.len()
    }

    /// Returns true if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.currencies.is_empty()
    }

    /// Finds currencies by exact country/territory name match
    pub fn find_by_country_exact(&self, country: &str) -> Vec<&Currency> {
        self.currencies
            .iter()
            .filter(|currency| currency.countries.contains(&country))
            .collect()
    }

    /// Finds currencies by country/territory name (case-insensitive substring match)
    pub fn find_by_country(&self, country: &str) -> Vec<&Currency> {
        let country_lower = country.to_lowercase();
        self.currencies
            .iter()
            .filter(|currency| {
                currency
                    .countries
                    .iter()
                    .any(|c| c.to_lowercase().contains(&country_lower))
            })
            .collect()
    }

    /// Returns all precious metal currencies
    pub fn precious_metals(&self) -> Vec<&Currency> {
        self.currencies
            .iter()
            .filter(|c| c.is_precious_metal())
            .collect()
    }

    /// Returns all supranational currencies
    pub fn supranational_currencies(&self) -> Vec<&Currency> {
        self.currencies
            .iter()
            .filter(|c| c.is_supranational())
            .collect()
    }
}

impl Default for CurrencyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// All ISO 4217 currencies (current as of January 2024)
pub static ALL_CURRENCIES: &[Currency] = &[
    // Active currencies
    Currency::new("AED", 784, Some(2), "UAE Dirham", &["United Arab Emirates"], true),
    Currency::new("AFN", 971, Some(2), "Afghani", &["Afghanistan"], true),
    Currency::new("ALL", 8, Some(2), "Lek", &["Albania"], true),
    Currency::new("AMD", 51, Some(2), "Armenian Dram", &["Armenia"], true),
    Currency::new("ANG", 532, Some(2), "Netherlands Antillean Guilder", &["Curaçao", "Sint Maarten"], true),
    Currency::new("AOA", 973, Some(2), "Kwanza", &["Angola"], true),
    Currency::new("ARS", 32, Some(2), "Argentine Peso", &["Argentina"], true),
    Currency::new("AUD", 36, Some(2), "Australian Dollar", &["Australia", "Christmas Island", "Cocos Islands", "Heard Island and McDonald Islands", "Kiribati", "Nauru", "Norfolk Island", "Tuvalu"], true),
    Currency::new("AWG", 533, Some(2), "Aruban Florin", &["Aruba"], true),
    Currency::new("AZN", 944, Some(2), "Azerbaijan Manat", &["Azerbaijan"], true),
    Currency::new("BAM", 977, Some(2), "Convertible Mark", &["Bosnia and Herzegovina"], true),
    Currency::new("BBD", 52, Some(2), "Barbados Dollar", &["Barbados"], true),
    Currency::new("BDT", 50, Some(2), "Taka", &["Bangladesh"], true),
    Currency::new("BGN", 975, Some(2), "Bulgarian Lev", &["Bulgaria"], true),
    Currency::new("BHD", 48, Some(3), "Bahraini Dinar", &["Bahrain"], true),
    Currency::new("BIF", 108, Some(0), "Burundi Franc", &["Burundi"], true),
    Currency::new("BMD", 60, Some(2), "Bermudian Dollar", &["Bermuda"], true),
    Currency::new("BND", 96, Some(2), "Brunei Dollar", &["Brunei Darussalam"], true),
    Currency::new("BOB", 68, Some(2), "Boliviano", &["Bolivia"], true),
    Currency::new("BOV", 984, Some(2), "Mvdol", &["Bolivia"], true),
    Currency::new("BRL", 986, Some(2), "Brazilian Real", &["Brazil"], true),
    Currency::new("BSD", 44, Some(2), "Bahamian Dollar", &["Bahamas"], true),
    Currency::new("BTN", 64, Some(2), "Ngultrum", &["Bhutan"], true),
    Currency::new("BWP", 72, Some(2), "Pula", &["Botswana"], true),
    Currency::new("BYN", 933, Some(2), "Belarusian Ruble", &["Belarus"], true),
    Currency::new("BZD", 84, Some(2), "Belize Dollar", &["Belize"], true),
    Currency::new("CAD", 124, Some(2), "Canadian Dollar", &["Canada"], true),
    Currency::new("CDF", 976, Some(2), "Congolese Franc", &["Democratic Republic of the Congo"], true),
    Currency::new("CHE", 947, Some(2), "WIR Euro", &["Switzerland"], true),
    Currency::new("CHF", 756, Some(2), "Swiss Franc", &["Switzerland", "Liechtenstein"], true),
    Currency::new("CHW", 948, Some(2), "WIR Franc", &["Switzerland"], true),
    Currency::new("CLF", 990, Some(4), "Unidad de Fomento", &["Chile"], true),
    Currency::new("CLP", 152, Some(0), "Chilean Peso", &["Chile"], true),
    Currency::new("CNY", 156, Some(2), "Yuan Renminbi", &["China"], true),
    Currency::new("COP", 170, Some(2), "Colombian Peso", &["Colombia"], true),
    Currency::new("COU", 970, Some(2), "Unidad de Valor Real", &["Colombia"], true),
    Currency::new("CRC", 188, Some(2), "Costa Rican Colon", &["Costa Rica"], true),
    Currency::new("CUC", 931, Some(2), "Peso Convertible", &["Cuba"], true),
    Currency::new("CUP", 192, Some(2), "Cuban Peso", &["Cuba"], true),
    Currency::new("CVE", 132, Some(2), "Cabo Verde Escudo", &["Cabo Verde"], true),
    Currency::new("CZK", 203, Some(2), "Czech Koruna", &["Czech Republic"], true),
    Currency::new("DJF", 262, Some(0), "Djibouti Franc", &["Djibouti"], true),
    Currency::new("DKK", 208, Some(2), "Danish Krone", &["Denmark", "Faroe Islands", "Greenland"], true),
    Currency::new("DOP", 214, Some(2), "Dominican Peso", &["Dominican Republic"], true),
    Currency::new("DZD", 12, Some(2), "Algerian Dinar", &["Algeria"], true),
    Currency::new("EGP", 818, Some(2), "Egyptian Pound", &["Egypt"], true),
    Currency::new("ERN", 232, Some(2), "Nakfa", &["Eritrea"], true),
    Currency::new("ETB", 230, Some(2), "Ethiopian Birr", &["Ethiopia"], true),
    Currency::new("EUR", 978, Some(2), "Euro", &["Austria", "Belgium", "Cyprus", "Estonia", "Finland", "France", "Germany", "Greece", "Ireland", "Italy", "Latvia", "Lithuania", "Luxembourg", "Malta", "Netherlands", "Portugal", "Slovakia", "Slovenia", "Spain", "Andorra", "Monaco", "San Marino", "Vatican City", "Kosovo", "Montenegro"], true),
    Currency::new("FJD", 242, Some(2), "Fiji Dollar", &["Fiji"], true),
    Currency::new("FKP", 238, Some(2), "Falkland Islands Pound", &["Falkland Islands"], true),
    Currency::new("GBP", 826, Some(2), "Pound Sterling", &["United Kingdom", "Isle of Man", "Jersey", "Guernsey"], true),
    Currency::new("GEL", 981, Some(2), "Lari", &["Georgia"], true),
    Currency::new("GGP", 0, Some(2), "Guernsey Pound", &["Guernsey"], false), // Not official ISO code
    Currency::new("GHS", 936, Some(2), "Ghana Cedi", &["Ghana"], true),
    Currency::new("GIP", 292, Some(2), "Gibraltar Pound", &["Gibraltar"], true),
    Currency::new("GMD", 270, Some(2), "Dalasi", &["Gambia"], true),
    Currency::new("GNF", 324, Some(0), "Guinean Franc", &["Guinea"], true),
    Currency::new("GTQ", 320, Some(2), "Quetzal", &["Guatemala"], true),
    Currency::new("GYD", 328, Some(2), "Guyana Dollar", &["Guyana"], true),
    Currency::new("HKD", 344, Some(2), "Hong Kong Dollar", &["Hong Kong"], true),
    Currency::new("HNL", 340, Some(2), "Lempira", &["Honduras"], true),
    Currency::new("HTG", 332, Some(2), "Gourde", &["Haiti"], true),
    Currency::new("HUF", 348, Some(2), "Forint", &["Hungary"], true),
    Currency::new("IDR", 360, Some(2), "Rupiah", &["Indonesia"], true),
    Currency::new("ILS", 376, Some(2), "New Israeli Sheqel", &["Israel"], true),
    Currency::new("IMP", 0, Some(2), "Isle of Man Pound", &["Isle of Man"], false), // Not official ISO code
    Currency::new("INR", 356, Some(2), "Indian Rupee", &["India", "Bhutan"], true),
    Currency::new("IQD", 368, Some(3), "Iraqi Dinar", &["Iraq"], true),
    Currency::new("IRR", 364, Some(2), "Iranian Rial", &["Iran"], true),
    Currency::new("ISK", 352, Some(0), "Iceland Krona", &["Iceland"], true),
    Currency::new("JEP", 0, Some(2), "Jersey Pound", &["Jersey"], false), // Not official ISO code
    Currency::new("JMD", 388, Some(2), "Jamaican Dollar", &["Jamaica"], true),
    Currency::new("JOD", 400, Some(3), "Jordanian Dinar", &["Jordan"], true),
    Currency::new("JPY", 392, Some(0), "Yen", &["Japan"], true),
    Currency::new("KES", 404, Some(2), "Kenyan Shilling", &["Kenya"], true),
    Currency::new("KGS", 417, Some(2), "Som", &["Kyrgyzstan"], true),
    Currency::new("KHR", 116, Some(2), "Riel", &["Cambodia"], true),
    Currency::new("KMF", 174, Some(0), "Comorian Franc", &["Comoros"], true),
    Currency::new("KPW", 408, Some(2), "North Korean Won", &["North Korea"], true),
    Currency::new("KRW", 410, Some(0), "Won", &["South Korea"], true),
    Currency::new("KWD", 414, Some(3), "Kuwaiti Dinar", &["Kuwait"], true),
    Currency::new("KYD", 136, Some(2), "Cayman Islands Dollar", &["Cayman Islands"], true),
    Currency::new("KZT", 398, Some(2), "Tenge", &["Kazakhstan"], true),
    Currency::new("LAK", 418, Some(2), "Lao Kip", &["Laos"], true),
    Currency::new("LBP", 422, Some(2), "Lebanese Pound", &["Lebanon"], true),
    Currency::new("LKR", 144, Some(2), "Sri Lanka Rupee", &["Sri Lanka"], true),
    Currency::new("LRD", 430, Some(2), "Liberian Dollar", &["Liberia"], true),
    Currency::new("LSL", 426, Some(2), "Loti", &["Lesotho"], true),
    Currency::new("LYD", 434, Some(3), "Libyan Dinar", &["Libya"], true),
    Currency::new("MAD", 504, Some(2), "Moroccan Dirham", &["Morocco", "Western Sahara"], true),
    Currency::new("MDL", 498, Some(2), "Moldovan Leu", &["Moldova"], true),
    Currency::new("MGA", 969, Some(2), "Malagasy Ariary", &["Madagascar"], true),
    Currency::new("MKD", 807, Some(2), "Denar", &["North Macedonia"], true),
    Currency::new("MMK", 104, Some(2), "Kyat", &["Myanmar"], true),
    Currency::new("MNT", 496, Some(2), "Tugrik", &["Mongolia"], true),
    Currency::new("MOP", 446, Some(2), "Pataca", &["Macao"], true),
    Currency::new("MRU", 929, Some(2), "Ouguiya", &["Mauritania"], true),
    Currency::new("MUR", 480, Some(2), "Mauritius Rupee", &["Mauritius"], true),
    Currency::new("MVR", 462, Some(2), "Rufiyaa", &["Maldives"], true),
    Currency::new("MWK", 454, Some(2), "Malawi Kwacha", &["Malawi"], true),
    Currency::new("MXN", 484, Some(2), "Mexican Peso", &["Mexico"], true),
    Currency::new("MXV", 979, Some(2), "Mexican Unidad de Inversion", &["Mexico"], true),
    Currency::new("MYR", 458, Some(2), "Malaysian Ringgit", &["Malaysia"], true),
    Currency::new("MZN", 943, Some(2), "Mozambique Metical", &["Mozambique"], true),
    Currency::new("NAD", 516, Some(2), "Namibia Dollar", &["Namibia"], true),
    Currency::new("NGN", 566, Some(2), "Naira", &["Nigeria"], true),
    Currency::new("NIO", 558, Some(2), "Cordoba Oro", &["Nicaragua"], true),
    Currency::new("NOK", 578, Some(2), "Norwegian Krone", &["Norway", "Bouvet Island", "Svalbard and Jan Mayen"], true),
    Currency::new("NPR", 524, Some(2), "Nepalese Rupee", &["Nepal"], true),
    Currency::new("NZD", 554, Some(2), "New Zealand Dollar", &["New Zealand", "Cook Islands", "Niue", "Pitcairn", "Tokelau"], true),
    Currency::new("OMR", 512, Some(3), "Rial Omani", &["Oman"], true),
    Currency::new("PAB", 590, Some(2), "Balboa", &["Panama"], true),
    Currency::new("PEN", 604, Some(2), "Sol", &["Peru"], true),
    Currency::new("PGK", 598, Some(2), "Kina", &["Papua New Guinea"], true),
    Currency::new("PHP", 608, Some(2), "Philippine Peso", &["Philippines"], true),
    Currency::new("PKR", 586, Some(2), "Pakistan Rupee", &["Pakistan"], true),
    Currency::new("PLN", 985, Some(2), "Zloty", &["Poland"], true),
    Currency::new("PYG", 600, Some(0), "Guarani", &["Paraguay"], true),
    Currency::new("QAR", 634, Some(2), "Qatari Rial", &["Qatar"], true),
    Currency::new("RON", 946, Some(2), "Romanian Leu", &["Romania"], true),
    Currency::new("RSD", 941, Some(2), "Serbian Dinar", &["Serbia"], true),
    Currency::new("RUB", 643, Some(2), "Russian Ruble", &["Russia"], true),
    Currency::new("RWF", 646, Some(0), "Rwanda Franc", &["Rwanda"], true),
    Currency::new("SAR", 682, Some(2), "Saudi Riyal", &["Saudi Arabia"], true),
    Currency::new("SBD", 90, Some(2), "Solomon Islands Dollar", &["Solomon Islands"], true),
    Currency::new("SCR", 690, Some(2), "Seychelles Rupee", &["Seychelles"], true),
    Currency::new("SDG", 938, Some(2), "Sudanese Pound", &["Sudan"], true),
    Currency::new("SEK", 752, Some(2), "Swedish Krona", &["Sweden"], true),
    Currency::new("SGD", 702, Some(2), "Singapore Dollar", &["Singapore"], true),
    Currency::new("SHP", 654, Some(2), "Saint Helena Pound", &["Saint Helena", "Ascension and Tristan da Cunha"], true),
    Currency::new("SLE", 925, Some(2), "Leone", &["Sierra Leone"], true),
    Currency::new("SLL", 694, Some(2), "Leone", &["Sierra Leone"], false), // Old Leone, replaced by SLE
    Currency::new("SOS", 706, Some(2), "Somali Shilling", &["Somalia"], true),
    Currency::new("SRD", 968, Some(2), "Suriname Dollar", &["Suriname"], true),
    Currency::new("SSP", 728, Some(2), "South Sudanese Pound", &["South Sudan"], true),
    Currency::new("STN", 930, Some(2), "Dobra", &["Sao Tome and Principe"], true),
    Currency::new("SVC", 222, Some(2), "El Salvador Colon", &["El Salvador"], false), // Historical
    Currency::new("SYP", 760, Some(2), "Syrian Pound", &["Syria"], true),
    Currency::new("SZL", 748, Some(2), "Lilangeni", &["Eswatini"], true),
    Currency::new("THB", 764, Some(2), "Baht", &["Thailand"], true),
    Currency::new("TJS", 972, Some(2), "Somoni", &["Tajikistan"], true),
    Currency::new("TMT", 934, Some(2), "Turkmenistan New Manat", &["Turkmenistan"], true),
    Currency::new("TND", 788, Some(3), "Tunisian Dinar", &["Tunisia"], true),
    Currency::new("TOP", 776, Some(2), "Pa'anga", &["Tonga"], true),
    Currency::new("TRY", 949, Some(2), "Turkish Lira", &["Turkey"], true),
    Currency::new("TTD", 780, Some(2), "Trinidad and Tobago Dollar", &["Trinidad and Tobago"], true),
    Currency::new("TVD", 0, Some(2), "Tuvalu Dollar", &["Tuvalu"], false), // Not official ISO code
    Currency::new("TWD", 901, Some(2), "New Taiwan Dollar", &["Taiwan"], true),
    Currency::new("TZS", 834, Some(2), "Tanzanian Shilling", &["Tanzania"], true),
    Currency::new("UAH", 980, Some(2), "Hryvnia", &["Ukraine"], true),
    Currency::new("UGX", 800, Some(0), "Uganda Shilling", &["Uganda"], true),
    Currency::new("USD", 840, Some(2), "US Dollar", &["United States", "American Samoa", "British Indian Ocean Territory", "Ecuador", "El Salvador", "Guam", "Marshall Islands", "Micronesia", "Northern Mariana Islands", "Palau", "Panama", "Puerto Rico", "Timor-Leste", "Turks and Caicos Islands", "US Virgin Islands", "Wake Island"], true),
    Currency::new("USN", 997, Some(2), "US Dollar (Next day)", &["United States"], true),
    Currency::new("UYI", 940, Some(0), "Uruguay Peso en Unidades Indexadas", &["Uruguay"], true),
    Currency::new("UYU", 858, Some(2), "Peso Uruguayo", &["Uruguay"], true),
    Currency::new("UYW", 927, Some(4), "Unidad Previsional", &["Uruguay"], true),
    Currency::new("UZS", 860, Some(2), "Uzbekistan Sum", &["Uzbekistan"], true),
    Currency::new("VED", 926, Some(2), "Bolívar Soberano", &["Venezuela"], false), // Obsolete 2018-08-20
    Currency::new("VES", 928, Some(2), "Bolívar Soberano", &["Venezuela"], true), // Current Venezuelan currency
    Currency::new("VND", 704, Some(0), "Dong", &["Vietnam"], true),
    Currency::new("VUV", 548, Some(0), "Vatu", &["Vanuatu"], true),
    Currency::new("WST", 882, Some(2), "Tala", &["Samoa"], true),
    Currency::new("XAF", 950, Some(0), "CFA Franc BEAC", &["Cameroon", "Central African Republic", "Chad", "Republic of the Congo", "Equatorial Guinea", "Gabon"], true),
    Currency::new("XAG", 961, Some(2), "Silver", &["International"], true), // Precious metal
    Currency::new("XAU", 959, Some(2), "Gold", &["International"], true), // Precious metal
    Currency::new("XBA", 955, Some(2), "Bond Markets Unit European Composite Unit", &["International"], true),
    Currency::new("XBB", 956, Some(2), "Bond Markets Unit European Monetary Unit", &["International"], true),
    Currency::new("XBC", 957, Some(2), "Bond Markets Unit European Unit of Account 9", &["International"], true),
    Currency::new("XBD", 958, Some(2), "Bond Markets Unit European Unit of Account 17", &["International"], true),
    Currency::new("XCD", 951, Some(2), "East Caribbean Dollar", &["Anguilla", "Antigua and Barbuda", "Dominica", "Grenada", "Montserrat", "Saint Kitts and Nevis", "Saint Lucia", "Saint Vincent and the Grenadines"], true),
    Currency::new("XDR", 960, Some(2), "SDR (Special Drawing Right)", &["International Monetary Fund"], true),
    Currency::new("XOF", 952, Some(0), "CFA Franc BCEAO", &["Benin", "Burkina Faso", "Côte d'Ivoire", "Guinea-Bissau", "Mali", "Niger", "Senegal", "Togo"], true),
    Currency::new("XPD", 964, Some(2), "Palladium", &["International"], true), // Precious metal
    Currency::new("XPF", 953, Some(0), "CFP Franc", &["French Polynesia", "New Caledonia", "Wallis and Futuna"], true),
    Currency::new("XPT", 962, Some(2), "Platinum", &["International"], true), // Precious metal
    Currency::new("XSU", 994, Some(2), "Sucre", &["Sistema Unitario de Compensacion Regional de Pagos SUCRE"], true),
    Currency::new("XTS", 963, Some(2), "Codes specifically reserved for testing purposes", &["Testing"], true), // Testing
    Currency::new("XUA", 965, Some(2), "ADB Unit of Account", &["African Development Bank"], true),
    Currency::new("XXX", 999, None, "The codes assigned for transactions where no currency is involved", &["No currency"], true), // No currency
    Currency::new("YER", 886, Some(2), "Yemeni Rial", &["Yemen"], true),
    Currency::new("ZAR", 710, Some(2), "Rand", &["South Africa", "Lesotho", "Namibia"], true),
    Currency::new("ZMW", 967, Some(2), "Zambian Kwacha", &["Zambia"], true),
    Currency::new("ZWL", 932, Some(2), "Zimbabwe Dollar", &["Zimbabwe"], false), // Not actively used

    // Historical currencies (marked as inactive)
    Currency::new("HRK", 191, Some(2), "Kuna", &["Croatia"], false), // Replaced by EUR in 2023
];

/// Convenience functions for common operations
impl Currency {
    /// Returns true if this is a major world currency (commonly traded)
    pub fn is_major_currency(&self) -> bool {
        matches!(
            self.code,
            "USD" | "EUR" | "GBP" | "JPY" | "CHF" | "CAD" | "AUD" | "CNY" | "SEK" | "NOK" | "NZD" | "HKD" | "SGD"
        )
    }

    /// Returns true if this currency uses 3 decimal places
    pub fn uses_three_decimals(&self) -> bool {
        self.minor_units == Some(3)
    }

    /// Returns true if this currency has no fractional units
    pub fn is_whole_unit_currency(&self) -> bool {
        self.minor_units == Some(0) || self.minor_units.is_none()
    }
}

/// Quick validation functions
pub fn is_valid_currency_code(code: &str) -> bool {
    // Validate format first
    if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
        return false;
    }
    ALL_CURRENCIES.iter().any(|c| c.code == code)
}

pub fn is_valid_currency_numeric(numeric: u16) -> bool {
    // Valid numeric codes are in range 1-999
    if numeric == 0 || numeric > 999 {
        return false;
    }
    ALL_CURRENCIES.iter().any(|c| c.numeric == numeric)
}

pub fn get_currency_by_code(code: &str) -> Option<&'static Currency> {
    if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
        return None;
    }
    ALL_CURRENCIES.iter().find(|c| c.code == code)
}

pub fn get_currency_by_numeric(numeric: u16) -> Option<&'static Currency> {
    if numeric == 0 || numeric > 999 {
        return None;
    }
    ALL_CURRENCIES.iter().find(|c| c.numeric == numeric)
}

/// Returns only active currencies
pub fn get_active_currencies() -> impl Iterator<Item = &'static Currency> {
    ALL_CURRENCIES.iter().filter(|c| c.is_active)
}

/// Returns currencies by category
pub fn get_precious_metal_currencies() -> impl Iterator<Item = &'static Currency> {
    ALL_CURRENCIES.iter().filter(|c| c.is_precious_metal())
}

pub fn get_supranational_currencies() -> impl Iterator<Item = &'static Currency> {
    ALL_CURRENCIES.iter().filter(|c| c.is_supranational())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        let usd = Currency::new("USD", 840, Some(2), "US Dollar", &["United States"], true);
        assert_eq!(usd.code, "USD");
        assert_eq!(usd.numeric, 840);
        assert_eq!(usd.minor_units, Some(2));
        assert_eq!(usd.name, "US Dollar");
        assert!(usd.has_minor_units());
        assert_eq!(usd.decimal_places(), 2);
        assert!(usd.is_active);
    }

    #[test]
    fn test_currency_registry() {
        let registry = CurrencyRegistry::new();

        assert!(registry.is_valid_code("USD"));
        assert!(registry.is_valid_numeric(840));
        assert!(!registry.is_valid_code("XXY")); // Invalid code
        assert!(!registry.is_valid_code("usd")); // Lowercase invalid
        assert!(!registry.is_valid_code("US")); // Too short
        assert!(!registry.is_valid_numeric(0)); // Invalid numeric
        assert!(!registry.is_valid_numeric(1000)); // Out of range

        let usd = registry.get_by_code("USD").unwrap();
        assert_eq!(usd.code, "USD");
        assert_eq!(usd.numeric, 840);
        assert!(usd.is_active);

        let eur = registry.get_by_numeric(978).unwrap();
        assert_eq!(eur.code, "EUR");
        assert!(eur.is_active);
    }

    #[test]
    fn test_quick_validation() {
        assert!(is_valid_currency_code("USD"));
        assert!(is_valid_currency_code("EUR"));
        assert!(!is_valid_currency_code("XXY"));
        assert!(!is_valid_currency_code("usd")); // Case sensitive
        assert!(!is_valid_currency_code("US")); // Too short

        assert!(is_valid_currency_numeric(840)); // USD
        assert!(is_valid_currency_numeric(978)); // EUR
        assert!(!is_valid_currency_numeric(0)); // Invalid
        assert!(!is_valid_currency_numeric(1000)); // Out of range
    }

    #[test]
    fn test_currency_properties() {
        let jpy = get_currency_by_code("JPY").unwrap();
        assert!(jpy.is_whole_unit_currency());
        assert!(!jpy.has_minor_units());
        assert!(jpy.is_active);

        let usd = get_currency_by_code("USD").unwrap();
        assert!(usd.is_major_currency());
        assert!(usd.has_minor_units());
        assert_eq!(usd.decimal_places(), 2);
        assert!(usd.is_active);

        let kwd = get_currency_by_code("KWD").unwrap();
        assert!(kwd.uses_three_decimals());
        assert_eq!(kwd.decimal_places(), 3);
        assert!(kwd.is_active);

        // Test precious metals
        let gold = get_currency_by_code("XAU").unwrap();
        assert!(gold.is_precious_metal());
        assert!(gold.is_active);

        let silver = get_currency_by_code("XAG").unwrap();
        assert!(silver.is_precious_metal());
        assert!(silver.is_active);
    }

    #[test]
    fn test_historical_currencies() {
        // HRK is now historical (Croatia adopted EUR in 2023)
        let hrk = get_currency_by_code("HRK").unwrap();
        assert!(!hrk.is_active);

        // VED is obsolete, VES is current for Venezuela
        let ved = get_currency_by_code("VED");
        if let Some(ved_currency) = ved {
            assert!(!ved_currency.is_active);
        }

        let ves = get_currency_by_code("VES").unwrap();
        assert!(ves.is_active);
        assert_eq!(ves.numeric, 928);
    }

    #[test]
    fn test_find_by_country() {
        let registry = CurrencyRegistry::new();

        // Exact match
        let us_currencies = registry.find_by_country_exact("United States");
        assert!(!us_currencies.is_empty());
        assert!(us_currencies.iter().any(|c| c.code == "USD"));

        // Substring match
        let euro_currencies = registry.find_by_country("Germany");
        assert!(euro_currencies.iter().any(|c| c.code == "EUR"));
    }

    #[test]
    fn test_special_currencies() {
        let registry = CurrencyRegistry::new();

        // Test precious metals
        let precious_metals = registry.precious_metals();
        assert!(precious_metals.iter().any(|c| c.code == "XAU"));
        assert!(precious_metals.iter().any(|c| c.code == "XAG"));
        assert!(precious_metals.iter().any(|c| c.code == "XPT"));
        assert!(precious_metals.iter().any(|c| c.code == "XPD"));

        // Test supranational currencies
        let supranational = registry.supranational_currencies();
        assert!(supranational.iter().any(|c| c.code == "EUR"));
        assert!(supranational.iter().any(|c| c.code == "XCD"));
        assert!(supranational.iter().any(|c| c.code == "XAF"));

        // Test special purpose codes
        let xts = get_currency_by_code("XTS").unwrap();
        assert!(xts.is_special_purpose());

        let xxx = get_currency_by_code("XXX").unwrap();
        assert!(xxx.is_special_purpose());
    }

    #[test]
    fn test_registry_iteration() {
        let registry = CurrencyRegistry::new();
        let total_count = registry.iter().count();
        let active_count = registry.active_currencies().count();

        assert!(total_count > 150); // Should have many currencies
        assert!(active_count < total_count); // Some should be inactive
        assert_eq!(total_count, registry.len());

        // Test that we have the expected precious metals
        let precious_count = get_precious_metal_currencies().count();
        assert_eq!(precious_count, 4); // XAU, XAG, XPT, XPD
    }

    #[test]
    fn test_format_validation() {
        // Test that invalid formats are rejected
        assert!(!is_valid_currency_code(""));
        assert!(!is_valid_currency_code("U"));
        assert!(!is_valid_currency_code("US"));
        assert!(!is_valid_currency_code("USDD"));
        assert!(!is_valid_currency_code("US1"));
        assert!(!is_valid_currency_code("usd"));
        assert!(!is_valid_currency_code("Usd"));
    }

    #[test]
    fn test_euro_countries() {
        let eur = get_currency_by_code("EUR").unwrap();
        assert!(eur.countries.contains(&"Germany"));
        assert!(eur.countries.contains(&"France"));
        assert!(eur.countries.contains(&"Italy"));
        assert!(eur.countries.contains(&"Spain"));
        // Should not contain Croatia (uses EUR but separately tracked)
    }

    #[test]
    fn test_currency_categories() {
        let active_currencies: Vec<_> = get_active_currencies().collect();
        let precious_metals: Vec<_> = get_precious_metal_currencies().collect();
        let supranational: Vec<_> = get_supranational_currencies().collect();

        // Ensure we have reasonable numbers in each category
        assert!(active_currencies.len() > 100);
        assert_eq!(precious_metals.len(), 4);
        assert!(supranational.len() >= 6); // EUR, XCD, XAF, XOF, XPF, XDR at minimum

        // Ensure precious metals are all active
        for metal in precious_metals {
            assert!(metal.is_active);
        }
    }
}