import { ActivityTypes, ActivityDirs } from '../const/contract_types'

export const activityDir = (activity) => {
  switch (activity) {
  case ActivityTypes.Deposit:
  case ActivityTypes.Receive:
  case ActivityTypes.Reward:
  case ActivityTypes.WithdrawMinerBalance:
    return ActivityDirs.Incoming
  case ActivityTypes.Redeem:
  case ActivityTypes.Withdraw:
    return ActivityDirs.Outcoming
  }
}