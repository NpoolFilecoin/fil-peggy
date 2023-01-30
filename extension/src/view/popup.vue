<template>
  <div class="main_app">
    <headerComponent />
    <toolbarComponent />
    <RouterView />
  </div>
</template>

<script>
import headerComponent from '../components/header.vue'
import toolbarComponent from '../components/toolbar.vue'

export default {
  name: 'popupView',
  components: {
    headerComponent,
    toolbarComponent
  },
  data () {
    return {
    }
  }
}

</script>

<style scoped>
.main_app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  width: 360px;
  height: 572px;
  scrollbar-width: none; /* firefox */
  -ms-overflow-style: none; /* IE 10+ */
  overflow-x: hidden;
  overflow-y: auto;
}

.main_app::-webkit-scrollbar {
  display: none; /* Chrome Safari */
}
</style>
