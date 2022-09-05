# CreatePayoutThroughStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<**String**> | The destination of the payout (can be an address or a BIP21 url) | [optional]
**amount** | Option<**String**> | The amount of the payout in the currency of the pull payment (eg. USD). | [optional]
**payment_method** | Option<**String**> | The payment method of the payout | [optional]
**pull_payment_id** | Option<**String**> | The pull payment to create this for. Optional. | [optional]
**approved** | Option<**bool**> | Whether to approve this payout automatically upon creation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

