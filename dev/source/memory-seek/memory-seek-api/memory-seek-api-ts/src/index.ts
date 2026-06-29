/**
 * memory-seek-api — Memory Seek TypeScript API 客户端
 *
 * 纯函数 API，与 memory-seek-backend-rs-new 对齐
 *
 * @example
 * ```ts
 * import { initApiConfig, auth, user, photo } from 'memory-seek-api'
 *
 * // 初始化
 * initApiConfig({ baseUrl: '/api' })
 *
 * // 登录
 * const { data } = await auth.login({ account: 'admin', password: '123456' })
 *
 * // 照片
 * const photos = await photo.getPhotos({ size: 20 })
 *
 * // 收藏夹
 * const collections = await photo.collection.getCollectionList()
 *
 * // 评论
 * const comments = await photo.comment.getCommentList(photoId)
 * ```
 */

// 配置
export { initApiConfig, getApiConfig, type ApiConfig } from './config.js'

// 存储
export { AuthStorage, setStorageAdapter } from './storage.js'

// HTTP 客户端
export {
  get,
  post,
  put,
  patch,
  del,
  setMessageHandler,
  type MessageHandler,
} from './client.js'

// API 模块
export * as auth from './api/auth.js'
export * as user from './api/user.js'
export { photo } from './api/photo/index.js'

// 类型
export type {
  // 通用
  R,
  CursorPage,
  // auth 请求
  LoginRequest,
  RegisterRequest,
  SendEmailCodeRequest,
  // auth 响应
  LoginResult,
  TokenRefreshResult,
  // user 请求
  ChangeNicknameParam,
  ChangePasswordParam,
  GetUserInfoBatchParam,
  // user 响应
  UserInfo,
  UserInfoResult,
  InviterCodeResult,
  // photo 请求
  PhotoCursorParam,
  Md5sExistParam,
  DeletePhotoParam,
  // photo 响应
  PhotoResult,
  MonthStat,
  // collection 请求
  CollectionCreateParam,
  CollectionUpdateParam,
  CollectionPhotoCursorPageParam,
  CollectionPhotoAddBatchParam,
  CollectionPhotoRemoveBatchParam,
  // collection 响应
  CollectionResult,
  CollectionPhotoAddBatchResult,
  CollectionPhotoRemoveBatchResult,
  PhotoCollectionResult,
  // comment 请求
  CommentPublishParam,
  CommentCursorPageParam,
  // comment 响应
  PhotoCommentResult,
  // 存储
  StorageAdapter,
} from './types/index.js'
