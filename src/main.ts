import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import './assets/styles/main.css'

import App from './App.vue'
import router from './router'

const vuetify = createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'light',
    themes: {
      light: {
        colors: {
          primary: '#FB8C00', // Warm Orange - Vibrant & Friendly
          secondary: '#FFB74D', // Soft Orange - Complementary
          tertiary: '#5D4037', // Warm Brown - Accent
          error: '#D32F2F',
          surface: '#FFFFFF',
          background: '#FFFFFF', // Pure White
          'surface-variant': '#FFF3E0',
          'on-surface': '#4E342E', // Dark Brown/Grey - Softer than black
          'on-surface-variant': '#795548', // Medium Brown
          outline: '#BCAAA4',
          'outline-variant': '#D7CCC8',
          'primary-container': '#FFF3E0', // Cream
          'on-primary-container': '#E65100', // Deep Orange
          'secondary-container': '#FFF8E1',
          'on-secondary-container': '#E65100',
        },
      },
    },
  },
})

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(vuetify)

app.mount('#app')
