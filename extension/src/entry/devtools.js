import { createApp } from 'vue'
import App from '../view/devtools.vue'
chrome.devtools.panels.create('FIL Peggy', '', 'devtools.html')
createApp(App).mount('#app')
