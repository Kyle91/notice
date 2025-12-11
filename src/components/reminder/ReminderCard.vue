<template>
  <v-hover v-slot="{ isHovering, props }">
    <v-card
      v-bind="props"
      class="reminder-card mb-3 transition-swing"
      :elevation="isHovering ? 3 : 0"
      :variant="reminder.isEnabled ? 'elevated' : 'flat'"
      :color="reminder.isEnabled ? 'surface' : 'surface-container-high'"
      :class="{ 'border-primary': reminder.isEnabled }"
      border
      rounded="xl"
    >
      <div class="d-flex align-center pa-4">
        <!-- 左侧：状态指示条 -->
        <div
          class="status-indicator mr-4 rounded-pill"
          :class="reminder.isEnabled ? 'bg-primary' : 'bg-outline-variant'"
        ></div>

        <!-- 图标 -->
        <v-avatar
          :color="reminder.isEnabled ? 'primary-container' : 'surface-variant'"
          size="48"
          class="mr-4"
        >
          <v-icon
            :color="reminder.isEnabled ? 'primary' : 'on-surface-variant'"
            size="24"
          >
            mdi-bell-ring-outline
          </v-icon>
        </v-avatar>

        <!-- 中间：内容 -->
        <div class="flex-grow-1 min-width-0">
          <div class="d-flex align-center mb-1">
            <span
              class="text-body-1 font-weight-bold text-truncate"
              :class="reminder.isEnabled ? 'text-on-surface' : 'text-on-surface-variant'"
            >
              {{ reminder.title }}
            </span>
          </div>
          
          <div class="d-flex align-center flex-wrap gap-2">
            <!-- 时间胶囊 -->
            <v-chip
              size="small"
              :color="reminder.isEnabled ? 'primary' : 'outline'"
              variant="tonal"
              class="font-weight-medium"
            >
              <v-icon start size="14">mdi-clock-outline</v-icon>
              {{ reminder.remindTime }}
            </v-chip>

            <!-- 类型文本 -->
            <span class="text-caption text-on-surface-variant">
              {{ remindTypeText }}
            </span>

            <!-- 高级设置图标组 -->
            <div v-if="hasAdvancedSettings" class="d-flex align-center ml-1 opacity-60">
              <v-icon v-if="reminder.repeatOnClose" size="16" color="secondary" class="mr-1" title="关闭后重复">mdi-repeat</v-icon>
              <v-icon v-if="reminder.isLoop" size="16" color="secondary" class="mr-1" title="循环提醒">mdi-sync</v-icon>
              <v-icon v-if="reminder.notifyOnTrigger || reminder.notifyOnComplete" size="16" color="tertiary" class="mr-1" title="Server酱通知">mdi-bell-ring</v-icon>
              <v-icon v-if="reminder.notifyOnTimeout" size="16" color="error" title="超时提醒">mdi-timer-alert</v-icon>
            </div>
          </div>
        </div>

        <!-- 右侧：操作按钮 -->
        <div class="d-flex align-center ml-2">
          <v-switch
            :model-value="reminder.isEnabled"
            color="primary"
            hide-details
            density="compact"
            inset
            class="mr-2"
            @update:model-value="emit('toggle', reminder.uuid)"
          ></v-switch>
          
          <v-menu location="bottom end">
            <template v-slot:activator="{ props }">
              <v-btn
                icon
                variant="text"
                density="comfortable"
                color="on-surface-variant"
                v-bind="props"
              >
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>
            <v-list density="compact" rounded="lg" elevation="2">
              <v-list-item @click="emit('edit', reminder)">
                <template v-slot:prepend>
                  <v-icon size="small">mdi-pencil</v-icon>
                </template>
                <v-list-item-title>编辑</v-list-item-title>
              </v-list-item>
              <v-list-item @click="showDeleteDialog = true" color="error">
                <template v-slot:prepend>
                  <v-icon size="small" color="error">mdi-delete</v-icon>
                </template>
                <v-list-item-title class="text-error">删除</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </div>
      </div>

      <!-- 删除确认对话框 -->
      <v-dialog v-model="showDeleteDialog" max-width="320">
        <v-card rounded="xl">
          <v-card-title class="text-h6 pt-4 px-4">确认删除</v-card-title>
          <v-card-text class="px-4 py-2 text-body-2 text-on-surface-variant">
            确定要删除「{{ reminder.title }}」吗？此操作无法撤销。
          </v-card-text>
          <v-card-actions class="px-4 pb-4">
            <v-spacer></v-spacer>
            <v-btn variant="text" color="on-surface-variant" @click="showDeleteDialog = false">取消</v-btn>
            <v-btn color="error" variant="flat" @click="handleDelete">删除</v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </v-card>
  </v-hover>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Reminder } from '../../types/reminder'

const props = defineProps<{
  reminder: Reminder
}>()

const emit = defineEmits<{
  toggle: [uuid: string]
  edit: [reminder: Reminder]
  delete: [uuid: string]
}>()

const showDeleteDialog = ref(false)

const remindTypeText = computed(() => {
  switch (props.reminder.remindType) {
    case 'daily':
      return '每天'
    case 'once':
      return '单次'
    case 'weekday':
      return formatWeekdays(props.reminder.weekdays)
    case 'monthly':
      return formatMonthdays(props.reminder.monthdays)
    default:
      return '每天'
  }
})

const hasAdvancedSettings = computed(() => {
  const r = props.reminder
  return r.repeatOnClose || r.isLoop || r.notifyOnTrigger || r.notifyOnComplete || r.notifyOnTimeout
})

function formatWeekdays(weekdays?: number[]): string {
  if (!weekdays || weekdays.length === 0) return '每周'
  const dayNames = ['', '周一', '周二', '周三', '周四', '周五', '周六', '周日']
  return weekdays.map(d => dayNames[d]).join('、')
}

function formatMonthdays(monthdays?: number[]): string {
  if (!monthdays || monthdays.length === 0) return '每月'
  if (monthdays.length > 3) {
    return `每月${monthdays.slice(0, 3).join('、')}等${monthdays.length}天`
  }
  return `每月${monthdays.join('、')}日`
}

function handleDelete() {
  emit('delete', props.reminder.uuid)
  showDeleteDialog.value = false
}
</script>

<style scoped>
.status-indicator {
  width: 4px;
  height: 24px;
}
</style>
