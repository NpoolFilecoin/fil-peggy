<template>
  <div class='item'>
    <img class='icon' :src='icon' />
    <div class='inner'>
      <div class='title'>{{ title }}</div>
      <div v-if='subtitle' class='subtitle'>{{ subtitle }}</div>

      <div class='content'>
        <div class='left'>
          <div class='line'>Power:</div>
          <div class='line'>State:</div>
          <div v-if='contract' class='line'>Type:</div>
          <div v-if='contract' class='line'>{{ valueLabel }}:</div>
          <div class='line'>Daily Reward:</div>
        </div>
        <div class='right'>
          <div class='line'>{{ rawPower }}/{{ adjPower }}</div>
          <div class='line'>{{ subtitle ? "Custody Miner" : "Free Miner" }}</div>
          <div v-if='contract' class='line'>{{ custodyType }}</div>
          <div v-if='contract' class='line'>{{ value }}%</div>
          <div class='line'>{{ estimateDailyReward }} FIL</div>
        </div>
      </div>

      <div v-if='!validCustodyContract' class='error'>Invalid custody contract</div>
    </div>
    <img class='right-arrow' :src='rightArrow' />
  </div>
</template>

<script>
import { CustodyTypes } from '../const/contract_types'
import { powerDisplay } from '../utils/power_display'
import { checkPeggy } from '../web3/peggy'

export default {
  name: 'minerItem',
  props: {
    title: {
      type: String,
      required: true
    },
    subtitle: {
      type: String,
      required: false
    },
    rawPowerBytes: {
      // eslint-disable-next-line no-undef
      type: BigInt,
      required: true
    },
    adjPowerBytes: {
      // eslint-disable-next-line no-undef
      type: BigInt,
      required: true
    },
    estimateDailyReward: {
      type: Number,
      required: true
    }
  },
  data () {
    return {
      rightArrow: '../assets/icons/right-arrow-24x24.png',
      icon: '../assets/icons/miner-64x64.png',
      validCustodyContract: true
    }
  },
  mounted () {
    if (!this.subtitle) {
      return
    }

    let network = this.$store.getters.selectedNetwork
    if (!network) {
      return
    }

    checkPeggy(network.RpcEndpoint, this.subtitle)
      .then()
      .catch(() => {
        this.validCustodyContract = false
      })
  },
  computed: {
    contract () {
      return this.$store.getters.contractByAddress(this.subtitle)
    },
    valueLabel () {
      if (!this.contract) {
        return
      }
      switch (this.contract.custodyType) {
      case CustodyTypes.FixedIncome:
        return 'APY'
      case CustodyTypes.FixedFeeRate:
        return 'Fee Rate'
      default:
        return 'APY'
      }
    },
    value () {
      if (!this.contract) {
        return
      }
      return this.contract.value
    },
    rawPower () {
      return powerDisplay(this.rawPowerBytes)
    },
    adjPower () {
      return powerDisplay(this.adjPowerBytes)
    },
    custodyType () {
      if (!this.contract) {
        return
      }
      return this.contract.custodyType
    }
  }
}
</script>

<style scoped>
.item {
  display: flex;
  height: 100%;
  width: 100%;
  padding: 16px 0 16px 0;
  cursor: pointer;
}

.item .icon {
  height: 64px;
  width: 64px;
  margin-right: 10px;
}

.item .title {
  color: #8A8A8A;
  font-size: 16px;
  font-weight: bold;
}

.item .subtitle {
  margin-top: 2px;
  font-size: 8px;
  color: #0D99FF;
  max-width: 200px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.item .right-arrow {
  margin-left: auto;
  height: 24px;
  width: 24px;
}

.item .inner {
  width: 240px;
}

.item .inner .content {
  display: flex;
  margin-top: 8px;
  color: #A5A5A5;
  font-size: 11px;
}

.item .inner .content .left {
  width: 40%;
}

.item .inner .content .right {
  width: 60%;
}

.item .inner .content .line {
  line-height: 16px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.item .inner .content .miner {
  color: #0D99FF;
}

.item .inner .error {
  color: red;
  margin-top: 10px;
}
</style>
