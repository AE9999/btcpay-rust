# PayoutData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the payout | [optional]
**revision** | Option<**i32**> | The revision number of the payout. This revision number is incremented when the payout amount or destination is modified before the approval. | [optional]
**pull_payment_id** | Option<**String**> | The id of the pull payment this payout belongs to | [optional]
**date** | Option<**String**> | The creation date of the payout as a unix timestamp | [optional]
**destination** | Option<**String**> | The destination of the payout (can be an address or a BIP21 url) | [optional]
**amount** | Option<**String**> | The amount of the payout in the currency of the pull payment (eg. USD). | [optional]
**payment_method** | Option<**String**> | The payment method of the payout (e.g., \"BTC\" or \"BTC_LightningLike\" | [optional]
**crypto_code** | Option<**String**> | Crypto code of the payment method of the payout (e.g., \"BTC\" or \"LTC\") | [optional]
**payment_method_amount** | Option<**String**> | The amount of the payout in the currency of the payment method (eg. BTC). This is only available from the `AwaitingPayment` state. | [optional]
**state** | Option<**String**> | The state of the payout (`AwaitingApproval`, `AwaitingPayment`, `InProgress`, `Completed`, `Cancelled`) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


