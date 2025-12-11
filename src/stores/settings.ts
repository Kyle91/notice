import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings } from '../types/settings'
import * as settingsService from '../services/settings'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({
    autoStart: false,
    defaultSnoozeInterval: 5,
  })
  const loading = ref(false)

  async function loadSettings() {
    loading.value = true
    try {
      settings.value = await settingsService.getSettings()
    } catch (error) {
      console.error('Failed to load settings:', error)
    } finally {
      loading.value = false
    }
  }

  async function updateSettings(newSettings: AppSettings) {
    await settingsService.updateSettings(newSettings)
    settings.value = newSettings
  }

  async function testServerchan(domain: string | undefined, sendkey: string) {
    return await settingsService.testServerchan(domain, sendkey)
  }

  async function setAutoStart(enabled: boolean) {
    await settingsService.setAutoStart(enabled)
    settings.value.autoStart = enabled
  }

  return {
    settings,
    loading,
    loadSettings,
    updateSettings,
    testServerchan,
    setAutoStart,
  }
})
