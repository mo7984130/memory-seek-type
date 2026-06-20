//! 导出 TypeScript 类型定义
//!
//! 运行 `cargo run --bin export-ts` 将生成 TypeScript 类型文件到 `bindings/` 目录

use ts_rs::TS;

// 导入所有需要导出的类型
use memory_seek_type::auth::models::{AccessTokenResult, LoginParam, RegisterParam, SendEmailCodeParam};
use memory_seek_type::photo::models::{AddCommentParam, CollectionDTO, CommentDTO, CreateCollectionParam, PhotoDTO, UploadPhotoParam};
use memory_seek_type::user::models::{ChangePasswordParam, UpdateUserParam, UserDTO};

fn main() {
    println!("正在导出 TypeScript 类型定义...");

    // 导出所有类型
    // 注意: ts-rs 会自动处理 #[ts(export)] 标记的类型
    // 这里显式调用是为了触发导出
    let out_dir = "bindings";

    // 创建输出目录
    std::fs::create_dir_all(out_dir).expect("无法创建输出目录");

    // 导出 auth 类型
    LoginParam::export_all_to(out_dir).expect("导出 LoginParam 失败");
    RegisterParam::export_all_to(out_dir).expect("导出 RegisterParam 失败");
    SendEmailCodeParam::export_all_to(out_dir).expect("导出 SendEmailCodeParam 失败");
    AccessTokenResult::export_all_to(out_dir).expect("导出 AccessTokenResult 失败");

    // 导出 photo 类型
    UploadPhotoParam::export_all_to(out_dir).expect("导出 UploadPhotoParam 失败");
    CreateCollectionParam::export_all_to(out_dir).expect("导出 CreateCollectionParam 失败");
    AddCommentParam::export_all_to(out_dir).expect("导出 AddCommentParam 失败");
    PhotoDTO::export_all_to(out_dir).expect("导出 PhotoDTO 失败");
    CollectionDTO::export_all_to(out_dir).expect("导出 CollectionDTO 失败");
    CommentDTO::export_all_to(out_dir).expect("导出 CommentDTO 失败");

    // 导出 user 类型
    UpdateUserParam::export_all_to(out_dir).expect("导出 UpdateUserParam 失败");
    ChangePasswordParam::export_all_to(out_dir).expect("导出 ChangePasswordParam 失败");
    UserDTO::export_all_to(out_dir).expect("导出 UserDTO 失败");

    println!("✅ TypeScript 类型已导出到 {}/ 目录", out_dir);
}
