# PayLightningInvoiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bolt11** | Option<**String**> | The BOLT11 of the invoice to pay | [optional]
**amount** | Option<**String**> | Optional explicit payment amount in millisatoshi (if specified, it overrides the BOLT11 amount) | [optional]
**max_fee_percent** | Option<**f32**> | The fee limit expressed as a percentage of the payment amount | [optional]
**max_fee_flat** | Option<**String**> | The fee limit expressed as a fixed amount in satoshi | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


