/**
 * 照片模块 — 包含照片、收藏夹、评论、时间线、点赞 API
 *
 * @example
 * ```ts
 * import { photo } from 'memory-seek-api'
 *
 * // 照片
 * await photo.uploadPhoto(file)
 * await photo.getPhotos({ size: 20 })
 *
 * // 收藏夹
 * await photo.collection.getCollectionList()
 *
 * // 评论
 * await photo.comment.getCommentList(photoId)
 *
 * // 时间线
 * await photo.timeline.getMonthlyStats()
 *
 * // 点赞
 * await photo.like.likePhoto(photoId)
 * await photo.like.unlikePhoto(photoId)
 * await photo.like.getLikedPhotos({ size: 20 })
 * ```
 */

import * as photoApi from './photo.js'
import * as collectionApi from './collection.js'
import * as commentApi from './comment.js'
import * as timelineApi from './timeline.js'
import * as likeApi from './like.js'

export const photo = {
  ...photoApi,
  collection: collectionApi,
  comment: commentApi,
  timeline: timelineApi,
  like: likeApi,
} as const
