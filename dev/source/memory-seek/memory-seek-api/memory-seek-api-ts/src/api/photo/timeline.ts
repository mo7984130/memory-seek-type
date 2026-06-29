/**
 * 时间线统计 API
 *
 * 路由前缀: /photo/timeline
 */

import { get } from '../../client.js'
import type { R, MonthStat } from '../../types/index.js'

/**
 * 获取每月照片统计
 * GET /photo/timeline/stats
 */
export async function getMonthlyStats(): Promise<R<MonthStat[]>> {
  return get<MonthStat[]>('/photo/timeline/stats')
}
