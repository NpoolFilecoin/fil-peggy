import { createApp } from 'vue'
import { store } from '../store/store'
import { router } from '../router/router'
import App from '../view/popup.vue'

const app = createApp(App)

app.use(store)
app.use(router)

app.mount('#app')
