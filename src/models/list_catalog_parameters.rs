//! Query parameters for the List Catalog API

use super::enums::CatalogObjectType;

/// This is a model struct for ListCatalogParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListCatalogParameters {
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
    /// An optional case-insensitive, comma-separated list of object types to retrieve.
    ///
    /// The valid values are defined in the [CatalogObjectType] enum, for example, `ITEM`,
    /// `ITEM_VARIATION`, `CATEGORY`, `DISCOUNT`, `TAX`, `MODIFIER`, `MODIFIER_LIST`, `IMAGE`, etc.
    ///
    /// If this is unspecified, the operation returns objects of all the top level types at the
    /// version of the Square API used to make the request. Object types that are nested onto other
    /// object types are not included in the defaults.
    ///
    /// At the current API version the default object types are: ITEM, CATEGORY, TAX, DISCOUNT,
    /// MODIFIER_LIST, DINING_OPTION, TAX_EXEMPTION, SERVICE_CHARGE, PRICING_RULE, PRODUCT_SET,
    /// TIME_PERIOD, MEASUREMENT_UNIT, SUBSCRIPTION_PLAN, ITEM_OPTION, CUSTOM_ATTRIBUTE_DEFINITION,
    /// QUICK_AMOUNT_SETTINGS.
    pub types: Option<Vec<CatalogObjectType>>,
    /// The specific version of the catalog objects to be included in the response. This allows you
    /// to retrieve historical versions of objects. The specified version value is matched against
    /// the [CatalogObject]s' `version` attribute. If not included, results will be from the current
    /// version of the catalog.
    pub catalog_version: Option<i64>,
}

impl ListCatalogParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListCatalogParameters> for String {
    fn from(list_catalog_parameters: ListCatalogParameters) -> Self {
        list_catalog_parameters.to_string()
    }
}

impl ToString for ListCatalogParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(types) = &self.types {
            if !types.is_empty() {
                let string_types: Vec<String> =
                    types.iter().map(|t| serde_json::to_string(&t).unwrap()).collect();
                params.push(format!("types={}", string_types.join(",")));
            }
        }

        if let Some(catalog_version) = &self.catalog_version {
            params.push(format!("catalog_version={}", catalog_version));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
