# ApplicationServerInfoData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | BTCPay Server version | [optional]
**onion** | Option<**String**> | The Tor hostname | [optional]
**supported_payment_methods** | Option<**Vec<String>**> | The payment methods this server supports | [optional]
**fully_synched** | Option<**bool**> | True if the instance is fully synchronized, according to NBXplorer | [optional]
**sync_status** | Option<[**Vec<crate::models::ApplicationServerInfoSyncStatusData>**](ApplicationServerInfoSyncStatusData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


