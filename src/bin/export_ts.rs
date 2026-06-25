//! 导出 TypeScript 类型定义
//!
//! 运行 `cargo run --bin export-ts` 将生成 TypeScript 类型文件到 `bindings/` 目录

use ts_rs::TS;

// 导入所有需要导出的类型
use memory_seek_type::auth::models::{LoginRequest, LoginResponse, RegisterRequest, SendEmailCodeRequest};
use memory_seek_type::photo::models::{AddCommentParam, CollectionDTO, CommentDTO, CreateCollectionParam, PhotoDTO, UploadPhotoParam};
use memory_seek_type::user::models::{ChangePasswordParam, UpdateUserParam, UserDTO};

fn main() {
    println!("正在导出 TypeScript 类型定义...");

    let base_dir = "bindings";

    // 按模块创建输出目录
    let dirs = ["auth", "photo", "user"];
    for dir in &dirs {
        std::fs::create_dir_all(format!("{}/{}", base_dir, dir))
            .expect(&format!("无法创建 {}/{} 目录", base_dir, dir));
    }

    // 导出 auth 类型
    let auth_dir = format!("{}/auth", base_dir);
    LoginRequest::export_all_to(&auth_dir).expect("导出 LoginRequest 失败");
    RegisterRequest::export_all_to(&auth_dir).expect("导出 RegisterRequest 失败");
    SendEmailCodeRequest::export_all_to(&auth_dir).expect("导出 SendEmailCodeRequest 失败");
    LoginResponse::export_all_to(&auth_dir).expect("导出 LoginResponse 失败");

    // 导出 photo 类型
    let photo_dir = format!("{}/photo", base_dir);
    UploadPhotoParam::export_all_to(&photo_dir).expect("导出 UploadPhotoParam 失败");
    CreateCollectionParam::export_all_to(&photo_dir).expect("导出 CreateCollectionParam 失败");
    AddCommentParam::export_all_to(&photo_dir).expect("导出 AddCommentParam 失败");
    PhotoDTO::export_all_to(&photo_dir).expect("导出 PhotoDTO 失败");
    CollectionDTO::export_all_to(&photo_dir).expect("导出 CollectionDTO 失败");
    CommentDTO::export_all_to(&photo_dir).expect("导出 CommentDTO 失败");

    // 导出 user 类型
    let user_dir = format!("{}/user", base_dir);
    UpdateUserParam::export_all_to(&user_dir).expect("导出 UpdateUserParam 失败");
    ChangePasswordParam::export_all_to(&user_dir).expect("导出 ChangePasswordParam 失败");
    UserDTO::export_all_to(&user_dir).expect("导出 UserDTO 失败");

    println!("✅ TypeScript 类型已导出到 {}/ 目录（按模块分目录）", base_dir);
}
