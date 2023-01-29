<template>
  <div>HHHHHHHHHHHHHHHHHHHHHHH</div>
</template>

<script>
export default {
  name: 'storageProviders',
  data () {
    return {
    }
  },
  mounted () {
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarTitle', 'Storage Providers')
  },
  methods: {
  }
}
</script>

<style scoped>
</style>