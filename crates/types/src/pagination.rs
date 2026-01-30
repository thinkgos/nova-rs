use garde::Validate;
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_PER_PAGE: u64 = 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[garde(range(min = 1))]
    pub page: u64,
    #[serde(default = "default_per_page")]
    #[garde(range(min = 1, max = 500))]
    pub per_page: u64,
}

impl Pagination {
    pub fn to_pagination_reply<T>(&self) -> PaginationReply<T> {
        PaginationReply::default().set_pagination(self)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

const fn default_page() -> u64 {
    DEFAULT_PAGE
}
const fn default_per_page() -> u64 {
    DEFAULT_PER_PAGE
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationReply<T> {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub data: Vec<T>,
}

impl<T> PaginationReply<T> {
    pub fn new(page: u64, per_page: u64) -> Self {
        Self::default().set_page(page).set_per_page(per_page)
    }
    pub fn set_page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }
    pub fn set_per_page(mut self, per_page: u64) -> Self {
        self.per_page = per_page;
        self
    }
    pub fn set_total(mut self, total: u64) -> Self {
        self.total = total;
        self
    }
    pub fn set_data(mut self, data: Vec<T>) -> Self {
        self.data = data;
        self
    }
    pub fn set_pagination(mut self, p: &Pagination) -> Self {
        self.page = p.page;
        self.per_page = p.per_page;
        self
    }
}

impl<T> Default for PaginationReply<T> {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 0,
            total: 0,
            data: vec![],
        }
    }
}
