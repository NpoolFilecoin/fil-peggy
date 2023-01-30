<template>
  <div>HHHHHHHHHHHHHHHHHHHHHH</div>
</template>

<script>
export default {
  name: 'myContract',
  data () {
    return {
    }
  },
  mounted () {
    console.log(this.$route.query)

    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarTitle', 'Home')
  },
  methods: {
    onMenuClick: function (menu) {
      this.$router.push(menu.path)
    }
  }
}
</script>

<style scoped>
</style>
