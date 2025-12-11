<template>
  <v-app>
    <v-main class="popup-container">
      <!-- 加载状态 -->
      <div v-if="loading" class="d-flex justify-center align-center fill-height">
        <v-progress-circular indeterminate color="primary"></v-progress-circular>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="d-flex flex-column justify-center align-center fill-height">
        <v-icon size="48" color="error">mdi-alert-circle</v-icon>
        <p class="text-body-1 mt-4">{{ error }}</p>
        <v-btn variant="text" @click="closeWindow">关闭</v-btn>
      </div>

      <!-- 提醒内容 -->
      <v-card v-else-if="reminder" class="fill-height d-flex flex-column" elevation="0">
        <v-card-title class="text-h5 text-center pt-6">
          {{ reminder.title }}
        </v-card-title>

        <v-card-text class="text-center flex-grow-1">
          <p v-if="reminder.content" class="text-body-1 mb-4">
            {{ reminder.content }}
          </p>

          <!-- 链接 -->
          <div v-if="reminder.links && reminder.links.length > 0" class="d-flex justify-center flex-wrap gap-2 mb-4">
            <v-chip
              v-for="(link, index) in reminder.links"
              :key="index"
              color="primary"
              variant="tonal"
              @click="openLink(link.url)"
            >
              <v-icon start>mdi-link</v-icon>
              {{ link.name }}
            </v-chip>
          </div>
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions class="justify-center pa-4">
          <v-btn variant="text" @click="handleDismiss" :disabled="actionLoading">关闭</v-btn>
          <v-btn variant="outlined" @click="showSnoozeMenu = true" :disabled="actionLoading">稍后</v-btn>
          <v-btn color="primary" variant="tonal" @click="handleComplete" :loading="actionLoading">完成</v-btn>
        </v-card-actions>
      </v-card>

      <!-- 延迟菜单 -->
      <v-dialog v-model="showSnoozeMenu" max-width="300">
        <v-card>
          <v-card-title>延迟提醒</v-card-title>
          <v-list>
            <v-list-item
              v-for="option in snoozeOptions"
              :key="option.value"
              @click="handleSnooze(option.value)"
            >
              <v-list-item-title>{{ option.label }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card>
      </v-dialog>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { open } from '@tauri-apps/plugin-shell'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { Reminder } from '../types/reminder'
import { getReminder, completeReminder, snoozeReminder, dismissReminder } from '../services/reminder'

const route = useRoute()

const reminder = ref<Reminder | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)
const actionLoading = ref(false)
const showSnoozeMenu = ref(false)

const snoozeOptions = [
  { value: 5, label: '5 分钟后' },
  { value: 10, label: '10 分钟后' },
  { value: 15, label: '15 分钟后' },
  { value: 30, label: '30 分钟后' },
  { value: 60, label: '1 小时后' },
]

onMounted(async () => {
  const uuid = route.query.uuid as string
  if (!uuid) {
    error.value = '未找到提醒ID'
    loading.value = false
    return
  }

  try {
    reminder.value = await getReminder(uuid)
    if (!reminder.value) {
      error.value = '未找到提醒'
    }
  } catch (e) {
    console.error('Failed to load reminder:', e)
    error.value = '加载提醒失败'
  } finally {
    loading.value = false
  }
})

async function openLink(url: string) {
  try {
    await open(url)
  } catch (e) {
    console.error('Failed to open link:', e)
  }
}

async function closeWindow() {
  try {
    const window = getCurrentWindow()
    await window.close()
  } catch (e) {
    console.error('Failed to close window:', e)
  }
}

async function handleComplete() {
  if (!reminder.value) return
  actionLoading.value = true
  try {
    await completeReminder(reminder.value.uuid)
    await closeWindow()
  } catch (e) {
    console.error('Failed to complete reminder:', e)
  } finally {
    actionLoading.value = false
  }
}

async function handleSnooze(minutes: number) {
  if (!reminder.value) return
  showSnoozeMenu.value = false
  actionLoading.value = true
  try {
    await snoozeReminder(reminder.value.uuid, minutes)
    await closeWindow()
  } catch (e) {
    console.error('Failed to snooze reminder:', e)
  } finally {
    actionLoading.value = false
  }
}

async function handleDismiss() {
  if (!reminder.value) return
  actionLoading.value = true
  try {
    await dismissReminder(reminder.value.uuid)
    await closeWindow()
  } catch (e) {
    console.error('Failed to dismiss reminder:', e)
  } finally {
    actionLoading.value = false
  }
}
</script>

<style scoped>
.popup-container {
  height: 100vh;
  overflow: hidden;
}

.fill-height {
  height: 100%;
}
</style>
