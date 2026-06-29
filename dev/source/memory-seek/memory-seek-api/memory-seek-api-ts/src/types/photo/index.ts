/**
 * 照片相关类型
 */

export * from './collection'
export * from './comment'

/** 照片结果 (后端 PhotoResult) */
export interface PhotoResult {
  id: string
  name: string
  width: number
  height: number
  size: number
  createdAt: string
  isCollected?: boolean
  isLiked?: boolean
  thumbnailToken?: string
  previewToken?: string
  originalToken?: string
}

/** 照片分页查询参数 */
export interface PhotoCursorParam {
  cursor?: string
  size?: number
  direction?: 'next' | 'prev'
  defaultCollectionId?: string
}

/** MD5 批量校验请求 */
export interface Md5sExistParam {
  md5s: string[]
}

/** 批量删除照片请求 */
export interface DeletePhotoParam {
  photoIds: string[]
}

/** 每月照片统计 (后端 MonthStat) */
export interface MonthStat {
  dateStr: string
  count: number
}
