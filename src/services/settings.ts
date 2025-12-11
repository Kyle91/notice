import { invoke } from '@tauri-apps/api/core'
import type { AppSettings } from '../types/settings'
import type { Reminder } from '../types/reminder'

export async function getSettings(): Promise<AppSettings> {
  return invoke('get_settings')
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke('update_settings', { settings })
}

export async function testServerchan(domain: string | undefined, sendkey: string): Promise<boolean> {
  return invoke('test_serverchan', { domain, sendkey })
}

export async function setAutoStart(enabled: boolean): Promise<void> {
  return invoke('set_auto_start', { enabled })
}

export async function getAutoStart(): Promise<boolean> {
  return invoke('get_auto_start')
}

export async function getNextReminder(): Promise<Reminder | null> {
  return invoke('get_next_reminder')
}
