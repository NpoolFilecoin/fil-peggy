<template>
  <div class="main_app">
    <h1>Hello {{msg}}</h1>
  </div>
</template>

<script>
export default {
  name: 'devtoolsView',
  data () {
    return {
      msg: 'devdools'
    }
  }
}

</script>

<style scoped>
.main_app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
