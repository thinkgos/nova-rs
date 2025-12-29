use strum::{Display, EnumCount, EnumString, VariantNames};

/// 应用程序运行时环境
#[derive(PartialEq, Eq, Debug, Display, EnumString, EnumCount, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Deploy {
    /// 本地
    Local,
    /// 开发
    Dev,
    /// 测试
    Test,
    /// 预发布
    Uat,
    /// 生产
    Prod,
}

#[cfg(test)]
mod tests {
    use super::Deploy;
    use strum::{EnumCount, VariantNames};

    #[test]
    fn test_deploy_display() {
        assert_eq!(Deploy::Local.to_string(), "local");
        assert_eq!(Deploy::Dev.to_string(), "dev");
        assert_eq!(Deploy::Test.to_string(), "test");
        assert_eq!(Deploy::Uat.to_string(), "uat");
        assert_eq!(Deploy::Prod.to_string(), "prod");
    }
    #[test]
    fn test_parser_str() {
        assert_eq!("dev".parse(), Ok(Deploy::Dev));
        assert_eq!("dev".try_into(), Ok(Deploy::Dev));
        assert_eq!(
            "a".parse::<Deploy>(),
            Err(strum::ParseError::VariantNotFound)
        );
    }
    #[test]
    fn test_deploy_const() {
        assert_eq!(["local", "dev", "test", "uat", "prod"], Deploy::VARIANTS);
        assert_eq!(5, Deploy::COUNT);
    }
}
