//! https://www.sea-ql.org/SeaORM/docs/generate-entity/enumeration/
//! https://serde.rs/enum-representations.html

use sea_orm::prelude::{DeriveActiveEnum, StringLen};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize_repr,
    Deserialize_repr,
    EnumIter,
    EnumString,
    EnumMessage,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "u32", db_type = "Unsigned")]
#[serde(rename_all = "snake_case")]
#[repr(u32)]
pub enum Gender {
    /// 男
    #[strum(serialize = "1", message = "男")]
    Male = 1,
    /// 女
    #[strum(serialize = "2", message = "女")]
    Female = 2,
    /// 未知
    #[default]
    #[strum(serialize = "3", message = "未知")]
    Unknown = 3,
}

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    EnumMessage,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
#[serde(rename_all = "UPPERCASE")]
pub enum SysApiMethod {
    /// 查询
    #[default]
    #[serde(rename = "GET")]
    #[strum(serialize = "GET", message = "查询")]
    Get,
    /// 新增
    #[serde(rename = "POST")]
    #[strum(serialize = "POST", message = "新增")]
    Post,
    /// 更新
    #[serde(rename = "PUT")]
    #[strum(serialize = "PUT", message = "更新")]
    Put,
    /// 部分更新
    #[serde(rename = "PATCH")]
    #[strum(serialize = "PATCH", message = "部分更新")]
    Patch,
    /// 删除
    #[serde(rename = "DELETE")]
    #[strum(serialize = "DELETE", message = "删除")]
    Delete,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_number() {
        assert_eq!("1".parse::<Gender>(), Ok(Gender::Male));
        assert_eq!("2".parse::<Gender>(), Ok(Gender::Female));
        assert_eq!("3".parse::<Gender>(), Ok(Gender::Unknown));
        assert_eq!(Gender::Male.to_string(), "1");
    }
    #[test]
    fn test_enum_string() {
        assert_eq!("GET".parse::<SysApiMethod>(), Ok(SysApiMethod::Get));
        assert_eq!("POST".parse::<SysApiMethod>(), Ok(SysApiMethod::Post));
        assert_eq!("DELETE".parse::<SysApiMethod>(), Ok(SysApiMethod::Delete));
        assert_eq!(SysApiMethod::Get.to_string(), "GET");
    }
}
