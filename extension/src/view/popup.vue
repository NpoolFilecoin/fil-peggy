<template>
  <div class="main_app">
    <headerComponent />
    <toolbarComponent />
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
}
</style>
