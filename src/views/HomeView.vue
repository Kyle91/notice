<template>
  <v-layout class="home-container">
    <!-- 自定义标题栏 -->
    <TitleBar title="叮咚">
      <template #append>
        <v-btn icon size="small" variant="text" color="on-surface-variant" @click="goToSettings">
          <v-icon>mdi-cog-outline</v-icon>
        </v-btn>
      </template>
    </TitleBar>

    <!-- 主内容区域 -->
    <v-main class="bg-background pt-10">
      <v-container class="py-6 px-4" style="max-width: 900px;">
        
        <!-- 下次提醒卡片 (如果存在) -->
        <v-slide-y-transition>
          <div v-if="nextReminder" class="mb-6">
            <div class="text-subtitle-2 text-primary font-weight-bold mb-2 px-1">
              <v-icon size="small" class="mr-1">mdi-clock-fast</v-icon>
              即将到来
            </div>
            <v-card color="primary-container" variant="flat" class="rounded-xl px-4 py-3 d-flex align-center">
              <div class="flex-grow-1">
                 <div class="text-h6 text-on-primary-container font-weight-medium">{{ nextReminder.remindTime }}</div>
                 <div class="text-body-2 text-on-primary-container opacity-80">{{ nextReminder.title }}</div>
              </div>
              <v-icon size="40" color="primary" class="opacity-20">mdi-alarm</v-icon>
            </v-card>
          </div>
        </v-slide-y-transition>

        <!-- 提醒列表标题 -->
        <div class="d-flex align-center justify-space-between mb-3 px-1">
          <div class="text-h6 font-weight-bold text-on-surface">所有提醒</div>
          <div class="text-caption text-on-surface-variant">{{ reminderStore.reminders.length }} 个提醒</div>
        </div>

        <!-- 加载状态 -->
        <div v-if="reminderStore.loading" class="d-flex justify-center py-12">
          <v-progress-circular indeterminate color="primary" size="48"></v-progress-circular>
        </div>

        <!-- 空状态 -->
        <div v-else-if="reminderStore.reminders.length === 0" class="text-center py-16">
          <v-avatar color="surface-variant" size="120" class="mb-6">
            <v-icon size="64" color="outline">mdi-bell-sleep-outline</v-icon>
          </v-avatar>
          <h3 class="text-h5 font-weight-bold text-on-surface mb-2">还没有提醒</h3>
          <p class="text-body-1 text-on-surface-variant" style="max-width: 300px; margin: 0 auto;">
            点击右下角的加号按钮，创建一个新的提醒吧
          </p>
        </div>

        <!-- 提醒列表 -->
        <ReminderList v-else :reminders="reminderStore.reminders" @edit="handleEdit" @delete="handleDelete" />
        
        <!-- 底部垫高，防止被 FAB 遮挡 -->
        <div style="height: 88px;"></div>
      </v-container>
    </v-main>

    <!-- FAB 添加按钮 -->
    <v-btn
      icon
      size="x-large"
      color="primary"
      elevation="4"
      class="fab-add"
      @click="showAddDialog = true"
    >
      <v-icon size="32">mdi-plus</v-icon>
    </v-btn>

    <!-- 添加/编辑对话框 -->
    <ReminderForm
      v-model="showAddDialog"
      :reminder="editingReminder"
      @save="handleSave"
      @close="handleCloseForm"
    />
  </v-layout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useReminderStore } from '../stores/reminder'
import ReminderList from '../components/reminder/ReminderList.vue'
import ReminderForm from '../components/reminder/ReminderForm.vue'
import TitleBar from '../components/common/TitleBar.vue'
import type { Reminder, CreateReminderRequest, UpdateReminderRequest } from '../types/reminder'
import { getNextReminder } from '../services/settings'

const router = useRouter()
const reminderStore = useReminderStore()

const showAddDialog = ref(false)
const editingReminder = ref<Reminder | null>(null)
const nextReminder = ref<Reminder | null>(null)

onMounted(async () => {
  await loadNextReminder()
})

async function loadNextReminder() {
  try {
    nextReminder.value = await getNextReminder()
  } catch (error) {
    console.error('Failed to load next reminder:', error)
  }
}

function goToSettings() {
  router.push('/settings')
}

async function handleSave(data: CreateReminderRequest | UpdateReminderRequest) {
  try {
    if ('uuid' in data) {
      await reminderStore.updateReminder(data as UpdateReminderRequest)
    } else {
      await reminderStore.createReminder(data as CreateReminderRequest)
    }
    showAddDialog.value = false
    editingReminder.value = null
    await loadNextReminder()
  } catch (error) {
    console.error('Failed to save reminder:', error)
  }
}

function handleCloseForm() {
  showAddDialog.value = false
  editingReminder.value = null
}

function handleEdit(reminder: Reminder) {
  editingReminder.value = reminder
  showAddDialog.value = true
}

async function handleDelete() {
  await loadNextReminder()
}
</script>

<style scoped>
.home-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.home-container :deep(.v-main) {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.fab-settings {
  position: fixed;
  bottom: 120px;
  right: 16px;
  z-index: 1000;
}

.fab-add {
  position: fixed;
  bottom: 56px;
  right: 16px;
  z-index: 1000;
}
</style>
