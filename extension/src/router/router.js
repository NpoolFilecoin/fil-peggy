import { createRouter, createWebHashHistory } from 'vue-router'
import homePage from '../pages/home.vue'
import storageProvidersPage from '../pages/storageproviders.vue'
import custodyContractsPage from '../pages/custodycontracts.vue'
import investmentSharesPage from '../pages/investmentshares.vue'
import filecoinAccountsPage from '../pages/filecoinaccounts.vue'

const routes = [
  { path: '/', component: homePage },
  { path: '/storageproviders', component: storageProvidersPage },
  { path: '/custodycontracts', component: custodyContractsPage },
  { path: '/investmentshares', component: investmentSharesPage },
  { path: '/filecoinaccounts', component: filecoinAccountsPage }
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})
