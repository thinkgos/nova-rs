use std::collections::HashMap;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
struct Reply {
    code: usize,
    message: String,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("未经授权,请先登陆")]
    Unauthorized,
    #[error("没有权限,拒绝防问")]
    PermissionForbidden,
    #[error("请求参数错误")]
    ValidationError,
    #[error("业务处理失败, 请稍后再试")]
    DbError(#[from] DbErr),
    #[error("业务处理失败, 请稍后再试")]
    AnyhowError(#[from] anyhow::Error),
    #[error("用户名或密码错误")]
    UserOrPassword,
    #[error("用户名或密码错误次数超过限制")]
    UserOrPasswordOverTime,
    #[error("用户不存在")]
    UserNotExists,
    #[error("用户已存在")]
    UserAlreadyExists,
    #[error("用户未设置密码, 不能用密码登录")]
    UserNoPasswd,
    #[error("用户密码已存在, 请直接登录")]
    UserPasswdExist,
    #[error("旧密码不能与新密码相同")]
    SamePasswd,
    #[error("密码错误")]
    PasswordIncorrect,
    #[error("密码已失效")]
    PasswdExpired,
    #[error("密码错误次数超过限制")]
    PasswordOverQuota,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            Error::ValidationError => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::PermissionForbidden => StatusCode::FORBIDDEN,
            Error::AnyhowError(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::OK,
        };
        let body = Json(Reply {
            code: status_code.as_u16().into(),
            message: self.to_string(),
            detail: self.to_string(),
            metadata: None,
        });
        (status_code, body).into_response()
    }
}
