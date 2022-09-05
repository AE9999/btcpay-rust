# LightningPaymentData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The payment's ID | [optional]
**status** | Option<[**crate::models::LightningPaymentStatus**](LightningPaymentStatus.md)> |  | [optional]
**bolt11** | Option<**String**> | The BOLT11 representation of the payment | [optional]
**payment_hash** | Option<**String**> | The payment hash | [optional]
**preimage** | Option<**String**> | The payment preimage (available when status is complete) | [optional]
**created_at** | Option<**f32**> | The unix timestamp when the payment got created | [optional]
**total_amount** | Option<**String**> | The total amount (including fees) in millisatoshi | [optional]
**fee_amount** | Option<**String**> | The total fees in millisatoshi | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


