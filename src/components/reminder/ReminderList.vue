<template>
  <div class="reminder-list">
    <ReminderCard
      v-for="reminder in reminders"
      :key="reminder.uuid"
      :reminder="reminder"
      class="mb-2"
      @toggle="handleToggle"
      @edit="handleEdit"
      @delete="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import ReminderCard from './ReminderCard.vue'
import type { Reminder } from '../../types/reminder'
import { useReminderStore } from '../../stores/reminder'

defineProps<{
  reminders: Reminder[]
}>()

const emit = defineEmits<{
  edit: [reminder: Reminder]
  delete: [uuid: string]
}>()

const reminderStore = useReminderStore()

async function handleToggle(uuid: string) {
  await reminderStore.toggleReminder(uuid)
}

function handleEdit(reminder: Reminder) {
  emit('edit', reminder)
}

async function handleDelete(uuid: string) {
  await reminderStore.deleteReminder(uuid)
  emit('delete', uuid)
}
</script>

<style scoped>
.reminder-list {
  display: flex;
  flex-direction: column;
}
</style>
