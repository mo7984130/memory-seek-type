//! 照片相关类型定义

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 上传照片请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPhotoParam {
    /// 照片标题
    #[validate(length(min = 1, max = 100, message = "标题长度在 1 到 100 个字符"))]
    pub title: String,

    /// 照片描述
    #[validate(length(max = 500, message = "描述长度最多 500 个字符"))]
    pub description: Option<String>,

    /// 标签
    #[validate(length(max = 10, message = "标签数量最多 10 个"))]
    pub tags: Option<Vec<String>>,
}

/// 创建收藏夹请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionParam {
    /// 收藏夹名称
    #[validate(length(min = 1, max = 50, message = "名称长度在 1 到 50 个字符"))]
    pub name: String,

    /// 收藏夹描述
    #[validate(length(max = 200, message = "描述长度最多 200 个字符"))]
    pub description: Option<String>,
}

/// 添加评论请求参数
#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentParam {
    /// 评论内容
    #[validate(length(min = 1, max = 500, message = "评论长度在 1 到 500 个字符"))]
    pub content: String,
}

/// 照片信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoDTO {
    /// 照片 ID
    pub id: String,

    /// 照片标题
    pub title: String,

    /// 照片描述
    pub description: Option<String>,

    /// 照片 URL
    pub url: String,

    /// 缩略图 URL
    pub thumbnail_url: String,

    /// 标签
    pub tags: Vec<String>,

    /// 上传者 ID
    pub uploader_id: String,

    /// 创建时间
    pub created_at: String,
}

/// 收藏夹信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDTO {
    /// 收藏夹 ID
    pub id: String,

    /// 收藏夹名称
    pub name: String,

    /// 收藏夹描述
    pub description: Option<String>,

    /// 照片数量
    pub photo_count: i64,

    /// 创建者 ID
    pub creator_id: String,

    /// 创建时间
    pub created_at: String,
}

/// 评论信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDTO {
    /// 评论 ID
    pub id: String,

    /// 评论内容
    pub content: String,

    /// 评论者 ID
    pub commenter_id: String,

    /// 评论者昵称
    pub commenter_nickname: String,

    /// 评论者头像
    pub commenter_avatar: Option<String>,

    /// 创建时间
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UploadPhotoParam validation ====================

    #[test]
    fn test_upload_photo_param_valid() {
        let param = UploadPhotoParam {
            title: "Beautiful Sunset".to_string(),
            description: Some("A beautiful sunset photo".to_string()),
            tags: Some(vec!["sunset".to_string(), "nature".to_string()]),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_upload_photo_param_valid_minimal() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_upload_photo_param_invalid_empty_title() {
        let param = UploadPhotoParam {
            title: "".to_string(),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_long_title() {
        let param = UploadPhotoParam {
            title: "a".repeat(101),
            description: None,
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_long_description() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: Some("a".repeat(501)),
            tags: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_upload_photo_param_invalid_too_many_tags() {
        let param = UploadPhotoParam {
            title: "Photo".to_string(),
            description: None,
            tags: Some((0..11).map(|i| format!("tag{}", i)).collect()),
        };
        assert!(param.validate().is_err());
    }

    // ==================== CreateCollectionParam validation ====================

    #[test]
    fn test_create_collection_param_valid() {
        let param = CreateCollectionParam {
            name: "My Collection".to_string(),
            description: Some("A collection of photos".to_string()),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_create_collection_param_invalid_empty_name() {
        let param = CreateCollectionParam {
            name: "".to_string(),
            description: None,
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_create_collection_param_invalid_long_name() {
        let param = CreateCollectionParam {
            name: "a".repeat(51),
            description: None,
        };
        assert!(param.validate().is_err());
    }

    // ==================== AddCommentParam validation ====================

    #[test]
    fn test_add_comment_param_valid() {
        let param = AddCommentParam {
            content: "Great photo!".to_string(),
        };
        assert!(param.validate().is_ok());
    }

    #[test]
    fn test_add_comment_param_invalid_empty_content() {
        let param = AddCommentParam {
            content: "".to_string(),
        };
        assert!(param.validate().is_err());
    }

    #[test]
    fn test_add_comment_param_invalid_long_content() {
        let param = AddCommentParam {
            content: "a".repeat(501),
        };
        assert!(param.validate().is_err());
    }

    // ==================== DTO serialization ====================

    #[test]
    fn test_photo_dto_serializes_to_camel_case() {
        let photo = PhotoDTO {
            id: "123".to_string(),
            title: "Sunset".to_string(),
            description: Some("A sunset".to_string()),
            url: "https://example.com/photo.jpg".to_string(),
            thumbnail_url: "https://example.com/thumb.jpg".to_string(),
            tags: vec!["sunset".to_string()],
            uploader_id: "user123".to_string(),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&photo).unwrap();
        assert!(json.contains("\"thumbnailUrl\""));
        assert!(json.contains("\"uploaderId\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("thumbnail_url"));
        assert!(!json.contains("uploader_id"));
        assert!(!json.contains("created_at"));
    }

    #[test]
    fn test_collection_dto_serializes_to_camel_case() {
        let collection = CollectionDTO {
            id: "123".to_string(),
            name: "My Collection".to_string(),
            description: Some("A collection".to_string()),
            photo_count: 10,
            creator_id: "user123".to_string(),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&collection).unwrap();
        assert!(json.contains("\"photoCount\""));
        assert!(json.contains("\"creatorId\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("photo_count"));
        assert!(!json.contains("creator_id"));
        assert!(!json.contains("created_at"));
    }

    #[test]
    fn test_comment_dto_serializes_to_camel_case() {
        let comment = CommentDTO {
            id: "123".to_string(),
            content: "Great!".to_string(),
            commenter_id: "user123".to_string(),
            commenter_nickname: "Test User".to_string(),
            commenter_avatar: Some("https://example.com/avatar.jpg".to_string()),
            created_at: "2026-06-13T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("\"commenterId\""));
        assert!(json.contains("\"commenterNickname\""));
        assert!(json.contains("\"commenterAvatar\""));
        assert!(json.contains("\"createdAt\""));
        assert!(!json.contains("commenter_id"));
        assert!(!json.contains("commenter_nickname"));
        assert!(!json.contains("commenter_avatar"));
        assert!(!json.contains("created_at"));
    }
}
