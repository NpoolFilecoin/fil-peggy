<template>
  <div class='abrev'>
    <div class='abrev1'>{{ totalFilAmount }}/{{ power }}</div>
    <div class='abrev2'>{{ contract.CustodyType }} ({{ contract.Value }}%)</div>
    <div class='actions'>
      <div class='action'>
        <img src='../assets/icons/add-40x40.png' />
        <div>Deposit</div>
      </div>
      <div class='action'>
        <img src='../assets/icons/benefit-40x40.png' />
        <div>Benefit</div>
      </div>
      <div class='action'>
        <img src='../assets/icons/withdraw-40x40.png' />
        <div>Withdraw</div>
      </div>
      <div class='action'>
        <img src='../assets/icons/redeem-40x40.png' />
        <div>Redeem</div>
      </div>
    </div>
  </div>
</template>

<script>
import { amountDisplay } from '../utils/amount_display'
import { powerDisplay } from '../utils/power_display'

export default {
  name: 'myContract',
  data () {
    return {
      contract: {}
    }
  },
  mounted () {
    this.contract = this.$store.getters.contractById(this.$route.query.contractId)
    this.$store.commit('setToolbarShowAddBtn', false)
    this.$store.commit('setToolbarTitle', this.contract.Title)
  },
  computed: {
    totalFilAmount () {
      return amountDisplay(this.contract.TotalAttoFilAmount)
    },
    power () {
      return powerDisplay(this.contract.AdjPowerBytes)
    }
  }
}
</script>

<style scoped>
.abrev {
  padding: 24px 64px 24px 64px;
  text-align: center;
  border-bottom: 1px solid #D6D9DC;
}

.abrev .abrev1 {
  color: #535A61;
  font-size: 20px;
  font-weight: bold;
}

.abrev .abrev2 {
  color: #535A61;
  font-size: 16px;
  margin-top: 8px;
}

.actions {
  display: flex;
  margin-top: 16px;
}

.actions .action {
  color: #0D99FF;
  width: 25%;
  text-align: center;
}
</style>
