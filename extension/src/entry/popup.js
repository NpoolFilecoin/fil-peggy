import { createApp } from 'vue'
import { store } from '../store/store'
import App from '../view/popup.vue'

const app = createApp(App)

app.use(store)

app.mount('#app')
