<template>
  <v-dialog v-model="dialogVisible" max-width="480" persistent scrollable>
    <v-card rounded="xl" class="d-flex flex-column" style="max-height: 90vh;">
      <v-card-title class="text-h6 px-4 pt-4 pb-2">
        {{ isEditing ? '编辑提醒' : '新建提醒' }}
      </v-card-title>

      <v-divider></v-divider>

      <v-card-text class="px-4 py-4" style="overflow-y: auto;">
        <v-form ref="form" @submit.prevent="handleSave">
          <!-- 标题 -->
          <v-text-field
            v-model="formData.title"
            label="提醒标题"
            variant="outlined"
            density="comfortable"
            color="primary"
            bg-color="surface"
            :rules="[v => !!v || '请输入标题']"
            class="mb-2"
          >
            <template v-slot:prepend-inner>
              <v-icon color="secondary">mdi-format-title</v-icon>
            </template>
          </v-text-field>

          <!-- 内容 -->
          <v-textarea
            v-model="formData.content"
            label="提醒内容（可选）"
            variant="outlined"
            density="comfortable"
            color="primary"
            bg-color="surface"
            rows="2"
            auto-grow
            class="mb-2"
          >
             <template v-slot:prepend-inner>
              <v-icon color="secondary" class="mt-1">mdi-text</v-icon>
            </template>
          </v-textarea>

          <!-- 链接 -->
          <div class="mb-4 pa-3 bg-surface-container-low rounded-lg border-thin">
            <div class="d-flex align-center justify-space-between mb-2">
              <div class="text-subtitle-2 font-weight-bold text-primary">快捷链接</div>
              <v-btn
                v-if="formData.links.length < 3"
                variant="text"
                size="small"
                color="primary"
                @click="addLink"
              >
                <v-icon start>mdi-plus</v-icon>
                添加
              </v-btn>
            </div>
            
            <div v-if="formData.links.length === 0" class="text-caption text-center text-disabled py-2">
              暂无链接
            </div>

            <div v-for="(link, index) in formData.links" :key="index" class="d-flex gap-2 mb-2 align-center">
              <v-text-field
                v-model="link.name"
                label="名称"
                variant="outlined"
                density="compact"
                hide-details
                bg-color="surface"
                style="max-width: 100px"
              ></v-text-field>
              <v-text-field
                v-model="link.url"
                label="URL"
                variant="outlined"
                density="compact"
                hide-details
                bg-color="surface"
                class="flex-grow-1"
              ></v-text-field>
              <v-btn icon size="x-small" variant="text" color="error" @click="removeLink(index)">
                <v-icon>mdi-close</v-icon>
              </v-btn>
            </div>
          </div>

          <!-- 提醒时间 -->
          <v-text-field
            v-model="formData.remindTime"
            label="提醒时间"
            variant="outlined"
            density="comfortable"
            color="primary"
            type="time"
            :rules="[v => !!v || '请选择时间']"
            class="mb-4"
          >
            <template v-slot:prepend-inner>
              <v-icon color="secondary">mdi-clock-outline</v-icon>
            </template>
          </v-text-field>

          <!-- 提醒类型 -->
          <div class="mb-4">
            <div class="text-subtitle-2 font-weight-bold mb-2">重复规则</div>
            <v-btn-toggle
              v-model="formData.remindType"
              mandatory
              color="primary"
              variant="outlined"
              divided
              class="d-flex width-100"
            >
              <v-btn value="daily" class="flex-grow-1">每天</v-btn>
              <v-btn value="once" class="flex-grow-1">单次</v-btn>
              <v-btn value="weekday" class="flex-grow-1">每周</v-btn>
              <v-btn value="monthly" class="flex-grow-1">每月</v-btn>
            </v-btn-toggle>
          </div>

          <!-- 周几选择 -->
          <div v-if="formData.remindType === 'weekday'" class="mb-4 pa-3 bg-surface-container-low rounded-lg">
            <div class="text-body-2 text-on-surface-variant mb-2">选择星期</div>
            <div class="weekday-chips d-flex flex-wrap gap-2">
              <v-chip
                v-for="day in weekdayOptions"
                :key="day.value"
                :color="formData.weekdays.includes(day.value) ? 'primary' : undefined"
                :variant="formData.weekdays.includes(day.value) ? 'flat' : 'outlined'"
                filter
                label
                size="small"
                @click="toggleWeekday(day.value)"
              >
                {{ day.label }}
              </v-chip>
            </div>
          </div>
          
          <!-- 每月几号选择 -->
          <div v-if="formData.remindType === 'monthly'" class="mb-4 pa-3 bg-surface-container-low rounded-lg">
             <div class="text-body-2 text-on-surface-variant mb-2">选择日期</div>
             <div class="monthday-grid">
               <div
                 v-for="day in 31"
                 :key="day"
                 class="monthday-item"
                 :class="{ active: formData.monthdays.includes(day) }"
                 @click="toggleMonthday(day)"
               >
                 {{ day }}
               </div>
             </div>
          </div>

          <v-divider class="my-3"></v-divider>

          <!-- 高级设置 -->
          <v-expansion-panels variant="accordion" class="mb-2">
            <v-expansion-panel elevation="0" bg-color="transparent">
              <v-expansion-panel-title class="px-0 py-0" style="min-height: 48px;">
                 <v-icon start color="secondary">mdi-tune</v-icon>
                 高级设置
              </v-expansion-panel-title>
              <v-expansion-panel-text class="px-0">
                <div class="d-flex flex-column gap-2 pt-2">
                  <v-checkbox
                     v-model="formData.repeatOnClose"
                     label="关闭提醒窗口后，第二天继续提醒"
                     density="compact"
                     hide-details
                     color="primary"
                  ></v-checkbox>
                  
                  <v-checkbox
                     v-model="formData.isLoop"
                     label="循环提醒（如每隔 30 分钟）"
                     density="compact"
                     hide-details
                     color="primary"
                  ></v-checkbox>
                  
                  <v-expand-transition>
                    <div v-if="formData.isLoop" class="pl-8 pt-2">
                      <v-text-field
                        v-model.number="formData.loopInterval"
                        label="循环间隔（分钟）"
                        type="number"
                        variant="outlined"
                        density="compact"
                        style="max-width: 200px"
                        hide-details
                      ></v-text-field>
                    </div>
                  </v-expand-transition>

                  <v-divider class="my-2"></v-divider>
                  <div class="text-subtitle-2 text-primary">Server酱通知</div>

                  <v-checkbox
                     v-model="formData.notifyOnTrigger"
                     label="提醒触发时通知"
                     density="compact"
                     hide-details
                     color="tertiary"
                  ></v-checkbox>
                  
                   <v-checkbox
                     v-model="formData.notifyOnComplete"
                     label="完成任务时通知"
                     density="compact"
                     hide-details
                     color="tertiary"
                  ></v-checkbox>
                  
                   <v-checkbox
                     v-model="formData.notifyOnTimeout"
                     label="任务超时未处理通知"
                     density="compact"
                     hide-details
                     color="error"
                  ></v-checkbox>
                </div>
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-expansion-panels>

        </v-form>
      </v-card-text>

      <v-divider></v-divider>

      <v-card-actions class="pa-4">
        <v-spacer></v-spacer>
        <v-btn variant="text" color="on-surface-variant" @click="dialogVisible = false">取消</v-btn>
        <v-btn color="primary" variant="flat" class="px-6" @click="handleSave">保存</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Reminder, CreateReminderRequest, UpdateReminderRequest, ReminderLink } from '../../types/reminder'

const props = defineProps<{
  modelValue: boolean
  reminder?: Reminder | null
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [data: CreateReminderRequest | UpdateReminderRequest]
  close: []
}>()

const form = ref()
const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
})

const isEditing = computed(() => !!props.reminder)

const weekdayOptions = [
  { value: 1, label: '一' },
  { value: 2, label: '二' },
  { value: 3, label: '三' },
  { value: 4, label: '四' },
  { value: 5, label: '五' },
  { value: 6, label: '六' },
  { value: 7, label: '日' },
]

const formData = ref({
  title: '',
  content: '',
  remindTime: '09:00',
  remindType: 'daily' as 'daily' | 'once' | 'weekday' | 'monthly',
  weekdays: [] as number[],
  monthdays: [] as number[],
  links: [] as ReminderLink[],
  // 高级设置
  repeatOnClose: false,
  repeatInterval: 5,
  isLoop: false,
  loopInterval: 5,
  notifyOnTrigger: false,
  notifyOnComplete: false,
  notifyOnTimeout: false,
  timeoutMinutes: 30,
})

watch(() => props.modelValue, (visible) => {
  if (visible) {
    if (props.reminder) {
      formData.value = {
        title: props.reminder.title,
        content: props.reminder.content,
        remindTime: props.reminder.remindTime,
        remindType: props.reminder.remindType,
        weekdays: props.reminder.weekdays || [],
        monthdays: props.reminder.monthdays || [],
        links: props.reminder.links ? [...props.reminder.links] : [],
        // 高级设置
        repeatOnClose: props.reminder.repeatOnClose || false,
        repeatInterval: props.reminder.repeatInterval || 5,
        isLoop: props.reminder.isLoop || false,
        loopInterval: props.reminder.loopInterval || 5,
        notifyOnTrigger: props.reminder.notifyOnTrigger || false,
        notifyOnComplete: props.reminder.notifyOnComplete || false,
        notifyOnTimeout: props.reminder.notifyOnTimeout || false,
        timeoutMinutes: props.reminder.timeoutMinutes || 30,
      }
    } else {
      resetForm()
    }
  }
})

function resetForm() {
  formData.value = {
    title: '',
    content: '',
    remindTime: '09:00',
    remindType: 'daily',
    weekdays: [],
    monthdays: [],
    links: [],
    // 高级设置
    repeatOnClose: false,
    repeatInterval: 5,
    isLoop: false,
    loopInterval: 5,
    notifyOnTrigger: false,
    notifyOnComplete: false,
    notifyOnTimeout: false,
    timeoutMinutes: 30,
  }
}

function addLink() {
  if (formData.value.links.length < 3) {
    formData.value.links.push({ name: '', url: '' })
  }
}

function removeLink(index: number) {
  formData.value.links.splice(index, 1)
}

function toggleWeekday(value: number) {
  const index = formData.value.weekdays.indexOf(value)
  if (index === -1) {
    formData.value.weekdays.push(value)
  } else {
    formData.value.weekdays.splice(index, 1)
  }
}

function toggleMonthday(value: number) {
  const index = formData.value.monthdays.indexOf(value)
  if (index === -1) {
    formData.value.monthdays.push(value)
  } else {
    formData.value.monthdays.splice(index, 1)
  }
}

async function handleSave() {
  const { valid } = await form.value.validate()
  if (!valid) return

  const links = formData.value.links.filter(l => l.name && l.url)

  if (props.reminder) {
    emit('save', {
      uuid: props.reminder.uuid,
      title: formData.value.title,
      content: formData.value.content,
      remindTime: formData.value.remindTime,
      remindType: formData.value.remindType,
      weekdays: formData.value.remindType === 'weekday' ? formData.value.weekdays : undefined,
      monthdays: formData.value.remindType === 'monthly' ? formData.value.monthdays : undefined,
      links: links.length > 0 ? links : undefined,
      // 高级设置
      repeatOnClose: formData.value.repeatOnClose,
      repeatInterval: formData.value.repeatOnClose ? formData.value.repeatInterval : undefined,
      isLoop: formData.value.isLoop,
      loopInterval: formData.value.isLoop ? formData.value.loopInterval : undefined,
      notifyOnTrigger: formData.value.notifyOnTrigger,
      notifyOnComplete: formData.value.notifyOnComplete,
      notifyOnTimeout: formData.value.notifyOnTimeout,
      timeoutMinutes: formData.value.notifyOnTimeout ? formData.value.timeoutMinutes : undefined,
    } as UpdateReminderRequest)
  } else {
    emit('save', {
      title: formData.value.title,
      content: formData.value.content || undefined,
      remindTime: formData.value.remindTime,
      remindType: formData.value.remindType,
      weekdays: formData.value.remindType === 'weekday' ? formData.value.weekdays : undefined,
      monthdays: formData.value.remindType === 'monthly' ? formData.value.monthdays : undefined,
      links: links.length > 0 ? links : undefined,
      // 高级设置
      repeatOnClose: formData.value.repeatOnClose,
      repeatInterval: formData.value.repeatOnClose ? formData.value.repeatInterval : undefined,
      isLoop: formData.value.isLoop,
      loopInterval: formData.value.isLoop ? formData.value.loopInterval : undefined,
      notifyOnTrigger: formData.value.notifyOnTrigger,
      notifyOnComplete: formData.value.notifyOnComplete,
      notifyOnTimeout: formData.value.notifyOnTimeout,
      timeoutMinutes: formData.value.notifyOnTimeout ? formData.value.timeoutMinutes : undefined,
    } as CreateReminderRequest)
  }
}

</script>

<style scoped>
.monthday-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.monthday-item {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: rgb(var(--v-theme-on-surface-variant));
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.monthday-item:hover {
  background-color: rgba(var(--v-theme-on-surface), 0.05);
}

.monthday-item.active {
  background-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  font-weight: bold;
}
</style>
