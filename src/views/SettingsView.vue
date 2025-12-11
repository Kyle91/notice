<template>
  <v-layout class="settings-container">
    <!-- 自定义标题栏 -->
    <TitleBar title="设置" show-back @back="goBack" />

    <!-- 主内容区域 -->
    <v-main class="bg-background pt-10">
      <v-container class="py-6 px-4" style="max-width: 800px;">
        <v-card variant="flat" color="surface" class="rounded-xl border">
          <v-list class="bg-transparent pa-2">
            <!-- Server酱配置 -->
            <v-list-subheader class="text-primary font-weight-bold">通知推送</v-list-subheader>
            
            <v-list-item class="px-4 py-2">
              <div class="text-caption font-weight-bold mb-1 ml-1 text-medium-emphasis">Server酱域名</div>
              <v-text-field
                v-model="settings.serverchanDomain"
                placeholder="https://sctapi.ftqq.com"
                variant="outlined"
                color="primary"
                density="compact"
                hide-details
                bg-color="surface"
                class="mb-4"
                @blur="saveSettings"
              >
              </v-text-field>
              
              <div class="text-caption font-weight-bold mb-1 ml-1 text-medium-emphasis">Server酱 SendKey</div>
              <v-text-field
                v-model="settings.serverchanSendkey" 
                placeholder="请输入 SendKey" 
                variant="outlined" 
                color="primary"
                density="compact"
                hide-details
                bg-color="surface"
                @blur="saveSettings"
              >
                <template #append-inner>
                  <v-btn size="small" variant="text" color="primary" :loading="testing" @click="testConnection">测试</v-btn>
                </template>
              </v-text-field>
            </v-list-item>

            <v-divider class="my-2"></v-divider>

            <!-- 系统设置 -->
            <v-list-subheader class="text-primary font-weight-bold">系统</v-list-subheader>
            
            <v-list-item class="px-4 py-2" @click="toggleAutoStart">
              <template #prepend>
                <v-avatar color="secondary-container" size="40" class="mr-3">
                  <v-icon color="secondary">mdi-power</v-icon>
                </v-avatar>
              </template>
              <v-list-item-title class="font-weight-medium">开机自启动</v-list-item-title>
              <v-list-item-subtitle>随系统启动自动运行</v-list-item-subtitle>
              <template #append>
                <v-switch v-model="settings.autoStart" color="primary" hide-details @update:model-value="handleAutoStartChange" @click.stop></v-switch>
              </template>
            </v-list-item>

            <v-list-item class="px-4 py-2">
              <template #prepend>
                <v-avatar color="secondary-container" size="40" class="mr-3">
                  <v-icon color="secondary">mdi-clock-outline</v-icon>
                </v-avatar>
              </template>
              <v-list-item-title class="font-weight-medium">默认延迟时间</v-list-item-title>
              <v-list-item-subtitle>提醒推迟时的默认时长</v-list-item-subtitle>
              <template #append>
                <v-select
                  v-model="settings.defaultSnoozeInterval"
                  :items="snoozeOptions"
                  variant="outlined"
                  density="compact"
                  color="primary"
                  hide-details
                  bg-color="surface-container-low"
                  style="width: 120px"
                  @update:model-value="saveSettings"
                ></v-select>
              </template>
            </v-list-item>

            <v-divider class="my-2"></v-divider>

            <!-- 关于 -->
            <v-list-subheader class="text-primary font-weight-bold">关于</v-list-subheader>
            <v-list-item class="px-4 py-2">
              <template #prepend>
                <v-avatar color="surface-variant" size="40" class="mr-3">
                  <v-icon color="on-surface-variant">mdi-information-outline</v-icon>
                </v-avatar>
              </template>
              <v-list-item-title class="font-weight-medium">版本信息</v-list-item-title>
              <v-list-item-subtitle>当前版本 v0.1.0</v-list-item-subtitle>
            </v-list-item>
          </v-list>
        </v-card>
      </v-container>
    </v-main>

    <!-- Snackbar -->
    <v-snackbar v-model="snackbar.show" :color="snackbar.color" location="top" rounded="pill">
      {{ snackbar.text }}
    </v-snackbar>
  </v-layout>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useSettingsStore } from "../stores/settings";
import TitleBar from "../components/common/TitleBar.vue";

const router = useRouter();
const settingsStore = useSettingsStore();

const settings = reactive({
  serverchanDomain: "",
  serverchanSendkey: "",
  autoStart: false,
  defaultSnoozeInterval: 5,
});

const testing = ref(false);
const snackbar = reactive({
  show: false,
  text: "",
  color: "success",
});

const snoozeOptions = [5, 10, 15, 30, 60];

onMounted(async () => {
  await settingsStore.loadSettings();
  Object.assign(settings, settingsStore.settings);
});

function goBack() {
  router.push("/");
}

function toggleAutoStart() {
  settings.autoStart = !settings.autoStart;
  handleAutoStartChange(settings.autoStart);
}

async function testConnection() {
  if (!settings.serverchanSendkey) {
    showSnackbar("请输入 SendKey", "error");
    return;
  }

  testing.value = true;
  try {
    await settingsStore.testServerchan(settings.serverchanDomain || undefined, settings.serverchanSendkey);
    showSnackbar("测试成功！", "success");
  } catch (error) {
    showSnackbar("测试失败，请检查配置", "error");
  } finally {
    testing.value = false;
  }
}

async function handleAutoStartChange(value: boolean | null) {
  if (value === null) return;
  try {
    await settingsStore.setAutoStart(value);
    await saveSettings();
  } catch (error) {
    console.error("Failed to set auto start:", error);
    // 回滚状态
    settings.autoStart = !value;
  }
}

async function saveSettings() {
  try {
    await settingsStore.updateSettings({
      serverchanDomain: settings.serverchanDomain || undefined,
      serverchanSendkey: settings.serverchanSendkey || undefined,
      autoStart: settings.autoStart,
      defaultSnoozeInterval: settings.defaultSnoozeInterval,
    });
  } catch (error) {
    console.error("Failed to save settings:", error);
  }
}

function showSnackbar(text: string, color: string) {
  snackbar.text = text;
  snackbar.color = color;
  snackbar.show = true;
}
</script>

<style scoped>
.settings-container {
  height: 100vh;
}
</style>
