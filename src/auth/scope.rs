use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::LsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    AuditRead,

    ProductsRead,
    ProductsWrite,
    ProductsReadPriceBooks,
    ProductsWritePriceBooks,

    CustomersRead,
    CustomersWrite,

    SalesRead,
    SalesWrite,

    InventoryRead,
    InventoryWrite,

    // ...
    Webhooks,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scope = match self {
            Scope::AuditRead => "audit:read",

            Scope::ProductsRead => "products:read",
            Scope::ProductsWrite => "products:write",
            Scope::ProductsReadPriceBooks => "products:read:price_books",
            Scope::ProductsWritePriceBooks => "products:write:price_books",

            Scope::CustomersRead => "customers:read",
            Scope::CustomersWrite => "customers:write",

            Scope::SalesRead => "sales:read",
            Scope::SalesWrite => "sales:write",

            Scope::InventoryRead => "inventory:read",
            Scope::InventoryWrite => "inventory:write",

            Scope::Webhooks => "webhooks",
        };

        write!(f, "{scope}")
    }
}

impl Scope {
    pub fn join(scopes: &[Scope]) -> String {
        scopes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn parse(scopes: &str) -> Vec<Self> {
        scopes
            .split_whitespace()
            .filter_map(|scope| scope.parse().ok())
            .collect()
    }
}

impl FromStr for Scope {
    type Err = LsError;

    fn from_str(scope: &str) -> Result<Self, Self::Err> {
        match scope {
            "audit:read" => Ok(Self::AuditRead),

            "products:read" => Ok(Self::ProductsRead),
            "products:write" => Ok(Self::ProductsWrite),
            "products:read:price_books" => Ok(Self::ProductsReadPriceBooks),
            "products:write:price_books" => Ok(Self::ProductsWritePriceBooks),

            "customers:read" => Ok(Self::CustomersRead),
            "customers:write" => Ok(Self::CustomersWrite),

            "sales:read" => Ok(Self::SalesRead),
            "sales:write" => Ok(Self::SalesWrite),

            "webhooks" => Ok(Self::Webhooks),

            _ => Err(LsError::OAuth(format!("Unknown scope '{scope}'"))),
        }
    }
}
