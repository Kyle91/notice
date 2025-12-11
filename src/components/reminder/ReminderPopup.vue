<template>
  <v-dialog v-model="dialogVisible" max-width="400" persistent>
    <v-card>
      <v-card-title class="text-h5 text-center pt-6">
        {{ reminder.title }}
      </v-card-title>

      <v-card-text class="text-center">
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
        <v-btn variant="text" @click="handleDismiss">关闭</v-btn>
        <v-btn variant="outlined" @click="showSnoozeMenu = true">稍后</v-btn>
        <v-btn color="primary" variant="tonal" @click="handleComplete">完成</v-btn>
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
  </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-shell'
import type { Reminder } from '../../types/reminder'

const props = defineProps<{
  reminder: Reminder
}>()

const emit = defineEmits<{
  complete: [uuid: string]
  snooze: [uuid: string, minutes: number]
  dismiss: [uuid: string]
}>()

const dialogVisible = ref(true)
const showSnoozeMenu = ref(false)

const snoozeOptions = [
  { value: 5, label: '5 分钟后' },
  { value: 10, label: '10 分钟后' },
  { value: 15, label: '15 分钟后' },
  { value: 30, label: '30 分钟后' },
  { value: 60, label: '1 小时后' },
]

async function openLink(url: string) {
  try {
    await open(url)
  } catch (error) {
    console.error('Failed to open link:', error)
  }
}

function handleComplete() {
  emit('complete', props.reminder.uuid)
  dialogVisible.value = false
}

function handleSnooze(minutes: number) {
  emit('snooze', props.reminder.uuid, minutes)
  showSnoozeMenu.value = false
  dialogVisible.value = false
}

function handleDismiss() {
  emit('dismiss', props.reminder.uuid)
  dialogVisible.value = false
}
</script>
