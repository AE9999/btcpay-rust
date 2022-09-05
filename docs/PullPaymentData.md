# PullPaymentData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id of the pull payment | [optional]
**name** | Option<**String**> | Name given to pull payment when it was created | [optional]
**description** | Option<**String**> | Description given to pull payment when it was created | [optional]
**currency** | Option<**String**> | The currency of the pull payment's amount | [optional]
**amount** | Option<**String**> | The amount in the currency of this pull payment as a decimal string | [optional]
**period** | Option<**i32**> | The length of each period in seconds | [optional]
**bolt11_expiration** | Option<**String**> | If lightning is activated, do not accept BOLT11 invoices with expiration less than … days | [optional]
**auto_approve_claims** | Option<**bool**> | Any payouts created for this pull payment will skip the approval phase upon creation | [optional][default to false]
**archived** | Option<**bool**> | Whether this pull payment is archived | [optional]
**view_link** | Option<**String**> | The link to a page to claim payouts to this pull payment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


