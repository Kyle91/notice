/** 提醒类型 */
export type RemindType = 'daily' | 'once' | 'weekday' | 'monthly'

/** 同步状态 */
export type SyncStatus = 'pending' | 'synced' | 'conflict'

/** 日志操作类型 */
export type LogAction = 'triggered' | 'completed' | 'dismissed' | 'snoozed'

/** 网站链接 */
export interface ReminderLink {
  name: string
  url: string
}

/** 提醒模型 */
export interface Reminder {
  id?: number
  uuid: string
  userId?: string

  // 基本信息
  title: string
  content: string
  links?: ReminderLink[]

  // 时间设置
  remindTime: string
  remindType: RemindType
  weekdays?: number[]
  monthdays?: number[]

  // 状态
  isEnabled: boolean

  // 重复设置
  repeatOnClose: boolean
  repeatInterval?: number
  isLoop: boolean
  loopInterval?: number

  // Server酱通知
  notifyOnTrigger: boolean
  notifyOnComplete: boolean
  notifyOnTimeout: boolean
  timeoutMinutes?: number

  // 运行状态
  lastTriggeredAt?: string
  lastCompletedAt?: string

  // 排序
  sortOrder: number

  // 时间戳
  createdAt: string
  updatedAt: string
  deletedAt?: string

  // 同步
  version: number
  syncStatus: SyncStatus
  syncAt?: string
}

/** 创建提醒请求 */
export interface CreateReminderRequest {
  title: string
  content?: string
  links?: ReminderLink[]
  remindTime: string
  remindType?: RemindType
  weekdays?: number[]
  monthdays?: number[]
  repeatOnClose?: boolean
  repeatInterval?: number
  isLoop?: boolean
  loopInterval?: number
  notifyOnTrigger?: boolean
  notifyOnComplete?: boolean
  notifyOnTimeout?: boolean
  timeoutMinutes?: number
}

/** 更新提醒请求 */
export interface UpdateReminderRequest {
  uuid: string
  title?: string
  content?: string
  links?: ReminderLink[]
  remindTime?: string
  remindType?: RemindType
  weekdays?: number[]
  monthdays?: number[]
  isEnabled?: boolean
  repeatOnClose?: boolean
  repeatInterval?: number
  isLoop?: boolean
  loopInterval?: number
  notifyOnTrigger?: boolean
  notifyOnComplete?: boolean
  notifyOnTimeout?: boolean
  timeoutMinutes?: number
  sortOrder?: number
}

/** 提醒日志 */
export interface ReminderLog {
  id?: number
  uuid: string
  reminderUuid: string
  action: LogAction
  triggeredAt: string
  actionAt?: string
  snoozeUntil?: string
  note?: string
  createdAt: string
  syncStatus: SyncStatus
  syncAt?: string
}
