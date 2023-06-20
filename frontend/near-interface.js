/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from 'near-api-js'

export class Contract {

  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getBeneficiary() {
    return await this.wallet.viewMethod({ contractId: this.contractId, method: "get_beneficiary" })
  }

  async latestDonations() {
    const number_of_donors = await this.wallet.viewMethod({ contractId: this.contractId, method: "number_of_donors" })
    const min = number_of_donors > 10 ? number_of_donors - 9 : 0

    let donations = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_donations", args: { from_index: min.toString(), limit: number_of_donors } })

    donations.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return donations
  }

  async latestDeposits() {
    const number_of_depositors = await this.wallet.viewMethod({ contractId: this.contractId, method: "number_of_depositors" })
    const min = number_of_depositors > 10 ? number_of_depositors - 9 : 0

    let deposits = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deposits", args: { from_index: min.toString(), limit: number_of_depositors } })

    deposits.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return deposits
  }

  async getDonationFromTransaction(txhash) {
    let donation_amount = await this.wallet.getTransactionResult(txhash);
    return utils.format.formatNearAmount(donation_amount);
  }

  async deposit(date,amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "deposit", args: { time: parseInt(date)}, deposit })
    return response
  }

  async depositstN(date,amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "depositst", args: { time: parseInt(date)}, deposit })
    return response
  }

  async payment(date,amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "payment", args: { time: parseInt(date), amount: deposit} })
    return response
  }

}