<template>
  <div>
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
    <div class='detail'>
      <div class='inner'>
        <div class='line'>
          <span class='left'>Contract ID</span>
          <span class='right contract-id'>{{ contract.Title }}</span>
        </div>
        <div class='line'>
          <span class='left'>Contract Address</span>
          <span class='right contract-address'>{{ contract.Subtitle }}</span>
        </div>
        <div class='line'>
          <span class='left'>Investment Type</span>
          <span class='right'>{{ contract.CustodyType }}</span>
        </div>
        <div class='line'>
          <span class='left'>{{ valueLabel }}</span>
          <span class='right value'>{{ contract.Value }}%</span>
        </div>
        <div class='line'>
          <span class='left'>Total Power</span>
          <span class='right'>{{ totalPower }}</span>
        </div>
        <div class='line'>
          <span class='left'>Total Reward</span>
          <span class='right'>{{ totalReward }}</span>
        </div>
        <div class='line'>
          <span class='left'>Network Power</span>
          <span class='right'>{{ networkPower }}</span>
        </div>
        <div class='line'>
          <span class='left'>Estimate Daily Reward</span>
          <span class='right daily-reward'>{{ estimateDailyReward }}</span>
        </div>
        <div class='line'>
          <span class='left'>Total Slash Penalty</span>
          <span class='right penalty'>{{ totalSlashPenalty }}</span>
        </div>
        <div class='line'>
          <span class='left'>Average Power Lost Interval</span>
          <span class='right'>{{ avgPowerLostInterval }}</span>
        </div>
        <div class='line'>
          <span class='left'>Average Power Lost Recover</span>
          <span class='right'>{{ avgPowerLostRecover }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Month Power Increment</span>
          <span class='right'>{{ lastMonthPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Half Year Power Increment</span>
          <span class='right'>{{ lastHalfYearPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Last Year Power Increment</span>
          <span class='right'>{{ lastYearPowerIncrement }}</span>
        </div>
        <div class='line'>
          <span class='left'>Balance</span>
          <span class='right'>{{ balance }}</span>
        </div>
        <div class='line'>
          <span class='left'>Miners</span>
          <span class='right miners'>{{ miners }}</span>
        </div>
      </div>
    </div>
    <div class='activities'>
      <div
        v-for='(act, index) in contract.Activities'
        :key='index'
        class='activity'
      >
        <img class='icon' :src='activityIcon(act)' />
        <div class='content'>
          <div class='top'>
            <span class='activity1'>{{ act.Activity }}</span>
            <span class='amount'>{{ activityAmount(act) }}</span>
          </div>
          <div class='timestamp'>{{ activityDate(act) }}</div>
          <div class='target'>{{ activityTarget(act) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { amountDisplay } from '../utils/amount_display'
import { powerDisplay } from '../utils/power_display'
import { CustodyTypes } from '../const/contract_types'
import { durationDisplay } from '../utils/time_display'
import { activityDir } from '../utils/activity_dir'
import { ActivityDirs } from '../const/contract_types'

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
  methods: {
    activityIcon: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return '../assets/icons/incoming-32x32.png'
      case ActivityDirs.Outcoming:
        return '../assets/icons/outcoming-32x32.png'
      }
    },
    activityAmount: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return '+ ' + amountDisplay(act.AttoFilAmount)
      case ActivityDirs.Outcoming:
        return '- ' + amountDisplay(act.AttoFilAmount)
      }
    },
    activityDate: function (act) {
      let date = new Date(act.Timestamp * 1000)
      return date.getFullYear() + '/' + date.getMonth() + '/' + date.getDate()
    },
    activityTarget: function (act) {
      switch (activityDir(act.Activity)) {
      case ActivityDirs.Incoming:
        return act.Target
      case ActivityDirs.Outcoming:
        return act.Target
      }
    }
  },
  computed: {
    totalFilAmount () {
      return amountDisplay(this.contract.TotalAttoFilAmount)
    },
    power () {
      return powerDisplay(this.contract.AdjPowerBytes)
    },
    valueLabel () {
      switch (this.contract.custodyType) {
      case CustodyTypes.FixedIncome:
        return 'Annual Percentage Yield'
      case CustodyTypes.FixedFeeRate:
        return 'Management Fee Rate'
      default:
        return 'Annual Percentage Yield'
      }
    },
    totalPower () {
      return powerDisplay(this.contract.RawPowerBytes) + '/' + powerDisplay(this.contract.AdjPowerBytes)
    },
    totalReward () {
      return amountDisplay(this.contract.TotalRewardAttoFilAmount)
    },
    networkPower () {
      return powerDisplay(this.$store.getters.networkRawPowerBytes) + '/' + powerDisplay(this.$store.getters.networkAdjPowerBytes)
    },
    estimateDailyReward () {
      return this.contract.EstimateDailyReward + ' FIL'
    },
    totalSlashPenalty () {
      return amountDisplay(this.contract.TotalSlashPenaltyAttoFilAmount)
    },
    avgPowerLostInterval () {
      return durationDisplay(this.contract.AvgPowerLostIntervalSeconds)
    },
    avgPowerLostRecover () {
      return durationDisplay(this.contract.AvgPowerLostRecoverSeconds)
    },
    lastMonthPowerIncrement () {
      return powerDisplay(this.contract.lastMonthRawPowerBytesIncrement) + '/' + powerDisplay(this.contract.lastMonthAdjPowerBytesIncrement)
    },
    lastHalfYearPowerIncrement () {
      return powerDisplay(this.contract.lastHalfYearRawPowerBytesIncrement) + '/' + powerDisplay(this.contract.lastHalfYearAdjPowerBytesIncrement)
    },
    lastYearPowerIncrement () {
      return powerDisplay(this.contract.lastYearRawPowerBytesIncrement) + '/' + powerDisplay(this.contract.lastYearAdjPowerBytesIncrement)
    },
    balance () {
      return amountDisplay(this.contract.BalanceAttoFilAmount)
    },
    miners () {
      if (!this.contract.Miners) {
        return 'None'
      }
      return this.contract.Miners.join(' ')
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

.detail {
  padding: 10px;
  border-bottom: 1px solid #D6D9DC;
}

.detail .inner {
  background-color: #F2F4F6;
  border-radius: 4px;
  padding: 10px;
  color: #535A61;
}

.detail .inner .line {
  width: 100%;
  height: 16px;
  line-height: 16px;
}

.detail .inner .line .left {
  float: left;
}

.detail .inner .line .right {
  float: right;
  max-width: 70%;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.detail .inner .contract-id {
  color: #0D99FF;
  font-weight: bold;
}

.detail .inner .contract-address {
  color: #0D99FF;
}

.detail .inner .value {
  font-weight: bold;
}

.detail .inner .daily-reward {
  color: #28D90C;
}

.detail .inner .penalty {
  color: #FAC712;
  font-weight: bold;
}

.detail .inner .miners {
  color: #0D99FF;
  word-wrap: break-word;
}

.activities {
  padding: 0 16px 0 16px;
}

.activities .activity {
  display: flex;
  height: 48px;
  padding: 16px 0 16px 0;
  border-bottom: 1px solid #D6D9DC;
}

.activities .activity .icon {
  height: 32px;
  width: 32px;
}

.activities .activity .content {
  height: 32px;
  width: calc(100% - 42px);
  margin-left: 10px;
  display: block;
}

.activities .activity .content .top {
  height: 16px;
  margin-bottom: 8px;
}

.activities .activity .content .activity1 {
  float: left;
  color: #535A61;
  font-weight: bold;
}

.activities .activity .content .amount {
  float: right;
  color: #535A61;
}

.activities .activity .content .timestamp {
  color: #28D90C;
}

.activities .activity .content .target {
  color: #8A8A8A;
}
</style>
