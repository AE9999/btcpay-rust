# CreateLightningInvoiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**String**> | Amount wrapped in a string, represented in a millistatoshi string. (1000 millisatoshi = 1 satoshi) | [optional]
**description** | Option<**String**> | Description of the invoice in the BOLT11 | [optional]
**description_hash** | Option<**String**> | Description hash of the invoice in the BOLT11 | [optional]
**expiry** | Option<[**crate::models::TimeSpanSeconds**](TimeSpanSeconds.md)> | Expiration time in seconds | [optional]
**private_route_hints** | Option<**bool**> | True if the invoice should include private route hints | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


