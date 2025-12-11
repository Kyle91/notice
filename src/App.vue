<template>
  <v-app>
    <router-view />
  </v-app>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useReminderStore } from './stores/reminder'

const reminderStore = useReminderStore()

onMounted(async () => {
  // 加载提醒列表
  await reminderStore.loadReminders()
})
</script>

<style>
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>
