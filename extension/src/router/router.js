import { createRouter, createWebHashHistory } from 'vue-router'
import popupView from '../view/popup.vue'

const routes = [
  { path: '/', component: popupView }
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})
