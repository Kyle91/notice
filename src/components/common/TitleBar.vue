<template>
  <div class="title-bar d-flex align-center" @mousedown="startDrag">
    <!-- 左侧区域 -->
    <div class="d-flex align-center pl-2" style="pointer-events: none; z-index: 1;">
      <div style="pointer-events: auto;">
        <slot name="prepend">
          <v-btn v-if="showBack" icon variant="text" size="small" density="compact" class="mr-2" @click.stop="$emit('back')">
            <v-icon>mdi-arrow-left</v-icon>
          </v-btn>
          <div v-else class="mr-2 d-flex align-center" style="pointer-events: none;">
             <AppLogo :size="28" />
          </div>
        </slot>
      </div>
      <div class="text-subtitle-2 font-weight-bold select-none text-high-emphasis ml-1">{{ title }}</div>
    </div>

    <v-spacer style="pointer-events: none;"></v-spacer>

    <!-- 右侧操作区 -->
    <div class="d-flex align-center mr-1" style="pointer-events: auto; z-index: 1;">
      <slot name="append"></slot>
    </div>

    <!-- 窗口控制按钮 -->
    <div class="window-controls d-flex align-center pr-2" style="pointer-events: auto; z-index: 1;">
      <div class="control-btn minimize d-flex align-center justify-center" @click.stop="minimize" title="最小化">
        <v-icon size="16">mdi-minus</v-icon>
      </div>
      <div class="control-btn close d-flex align-center justify-center" @click.stop="close" title="关闭">
        <v-icon size="16">mdi-close</v-icon>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import AppLogo from './AppLogo.vue'

defineProps<{
  title?: string
  showBack?: boolean
}>()

defineEmits(['back'])

async function startDrag(e: MouseEvent) {
  // 只有左键才能拖动
  if (e.button !== 0) return
  // 如果点击的是按钮或控制区域，不触发拖动
  const target = e.target as HTMLElement
  if (target.closest('.control-btn') || target.closest('button')) {
    return
  }
  await getCurrentWindow().startDragging()
}

async function minimize() {
  await getCurrentWindow().minimize()
}

async function close() {
  await getCurrentWindow().close()
}
</script>

<style scoped>
.title-bar {
  height: 40px;
  background: rgb(var(--v-theme-surface));
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 9999;
  border-bottom: 1px solid rgba(var(--v-theme-outline), 0.1);
}

.control-btn {
  width: 40px;
  height: 40px;
  cursor: pointer;
  transition: all 0.2s;
  color: rgb(var(--v-theme-on-surface-variant));
  border-radius: 0;
}

.control-btn:hover {
  background-color: rgba(var(--v-theme-on-surface), 0.08);
}

.control-btn.close:hover {
  background-color: rgb(var(--v-theme-error));
  color: white;
}

.select-none {
  user-select: none;
  cursor: default;
}
</style>