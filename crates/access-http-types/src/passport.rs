use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// 会话id
    pub session_id: String,
    /// 验证码id
    pub captcha_id: String,
    /// 验证码
    pub captcha: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginReply {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 过期时间(时间戳)
    pub expires_at: i64,
}
