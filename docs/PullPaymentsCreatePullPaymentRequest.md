# PullPaymentsCreatePullPaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the pull payment | [optional]
**description** | Option<**String**> | The description of the pull payment | [optional]
**amount** | Option<**String**> | The amount in `currency` of this pull payment as a decimal string | [optional]
**currency** | Option<**String**> | The currency of the amount. | [optional]
**period** | Option<**i32**> | The length of each period in seconds. | [optional]
**bolt11_expiration** | Option<**String**> | If lightning is activated, do not accept BOLT11 invoices with expiration less than … days | [optional][default to 30]
**auto_approve_claims** | Option<**bool**> | Any payouts created for this pull payment will skip the approval phase upon creation | [optional][default to false]
**starts_at** | Option<**i32**> | When this pull payment is effective. Already started if null or unspecified. | [optional]
**expires_at** | Option<**i32**> | When this pull payment expires. Never expires if null or unspecified. | [optional]
**payment_methods** | Option<**Vec<String>**> | The list of supported payment methods supported by this pull payment. Available options can be queried from the `StorePaymentMethods_GetStorePaymentMethods` endpoint | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


