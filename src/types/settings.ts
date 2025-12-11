/** 窗口位置 */
export interface WindowPosition {
  x: number
  y: number
  width: number
  height: number
}

/** 应用设置 */
export interface AppSettings {
  serverchanDomain?: string
  serverchanSendkey?: string
  autoStart: boolean
  defaultSnoozeInterval: number
  windowPosition?: WindowPosition
}
