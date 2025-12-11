import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Reminder, CreateReminderRequest, UpdateReminderRequest } from '../types/reminder'
import * as reminderService from '../services/reminder'

export const useReminderStore = defineStore('reminder', () => {
  const reminders = ref<Reminder[]>([])
  const loading = ref(false)
  const triggeredReminder = ref<Reminder | null>(null)

  async function loadReminders() {
    loading.value = true
    try {
      reminders.value = await reminderService.getReminders()
    } catch (error) {
      console.error('Failed to load reminders:', error)
    } finally {
      loading.value = false
    }
  }

  async function createReminder(request: CreateReminderRequest) {
    const reminder = await reminderService.createReminder(request)
    reminders.value.unshift(reminder)
    return reminder
  }

  async function updateReminder(request: UpdateReminderRequest) {
    const reminder = await reminderService.updateReminder(request)
    const index = reminders.value.findIndex(r => r.uuid === reminder.uuid)
    if (index !== -1) {
      reminders.value[index] = reminder
    }
    return reminder
  }

  async function deleteReminder(uuid: string) {
    await reminderService.deleteReminder(uuid)
    reminders.value = reminders.value.filter(r => r.uuid !== uuid)
  }

  async function toggleReminder(uuid: string) {
    const reminder = await reminderService.toggleReminder(uuid)
    const index = reminders.value.findIndex(r => r.uuid === reminder.uuid)
    if (index !== -1) {
      reminders.value[index] = reminder
    }
    return reminder
  }

  async function completeReminder(uuid: string) {
    await reminderService.completeReminder(uuid)
    triggeredReminder.value = null
  }

  async function snoozeReminder(uuid: string, minutes: number) {
    await reminderService.snoozeReminder(uuid, minutes)
    triggeredReminder.value = null
  }

  async function dismissReminder(uuid: string) {
    await reminderService.dismissReminder(uuid)
    triggeredReminder.value = null
  }

  function setTriggeredReminder(reminder: Reminder) {
    triggeredReminder.value = reminder
  }

  return {
    reminders,
    loading,
    triggeredReminder,
    loadReminders,
    createReminder,
    updateReminder,
    deleteReminder,
    toggleReminder,
    completeReminder,
    snoozeReminder,
    dismissReminder,
    setTriggeredReminder,
  }
})
