import { createRouter, createWebHashHistory } from 'vue-router'
import homePage from '../pages/home.vue'

const routes = [
  { path: '/', component: homePage }
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})
