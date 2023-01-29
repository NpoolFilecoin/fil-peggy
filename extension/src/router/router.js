import { createRouter, createWebHashHistory } from 'vue-router'
import homePage from '../pages/home.vue'
import storageProvidersPage from '../pages/storageproviders.vue'

const routes = [
  { path: '/', component: homePage },
  { path: '/storageproviders', component: storageProvidersPage }
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})
