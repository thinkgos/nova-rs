//!
//! 应用错误定义, 只用于http, 用于其它层转换为http错误响应
//!

use std::result;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use sea_orm::DbErr;
use thiserror::Error;

use types::responded::Reply;

pub type Result<T> = result::Result<Reply<T>, AppError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub(crate) enum AppError {
    #[error("服务器异常")]
    InternalServer,
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            Self::ValidationError => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::PermissionForbidden => StatusCode::FORBIDDEN,
            Self::AnyhowError(_) | Self::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };
        let reply = Reply {
            code: status_code.as_u16().into(),
            message: self.to_string(),
            detail: self.to_string(),
            data: (),
        };
        let mut response = (status_code, Json(reply)).into_response();
        response.extensions_mut().insert(Arc::new(self));
        response
    }
}
