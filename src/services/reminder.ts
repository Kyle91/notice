import { invoke } from '@tauri-apps/api/core'
import type { Reminder, CreateReminderRequest, UpdateReminderRequest, ReminderLog } from '../types/reminder'

export async function getReminders(): Promise<Reminder[]> {
  return invoke('get_reminders')
}

export async function getReminder(uuid: string): Promise<Reminder | null> {
  return invoke('get_reminder', { uuid })
}

export async function createReminder(request: CreateReminderRequest): Promise<Reminder> {
  return invoke('create_reminder', { request })
}

export async function updateReminder(request: UpdateReminderRequest): Promise<Reminder> {
  return invoke('update_reminder', { request })
}

export async function deleteReminder(uuid: string): Promise<void> {
  return invoke('delete_reminder', { uuid })
}

export async function toggleReminder(uuid: string): Promise<Reminder> {
  return invoke('toggle_reminder', { uuid })
}

export async function reorderReminders(uuids: string[]): Promise<void> {
  return invoke('reorder_reminders', { uuids })
}

export async function completeReminder(uuid: string): Promise<void> {
  return invoke('complete_reminder', { uuid })
}

export async function snoozeReminder(uuid: string, minutes: number): Promise<void> {
  return invoke('snooze_reminder', { uuid, minutes })
}

export async function dismissReminder(uuid: string): Promise<void> {
  return invoke('dismiss_reminder', { uuid })
}

export async function getReminderLogs(
  reminderUuid?: string,
  startDate?: string,
  endDate?: string,
  limit?: number
): Promise<ReminderLog[]> {
  return invoke('get_reminder_logs', { reminderUuid, startDate, endDate, limit })
}
