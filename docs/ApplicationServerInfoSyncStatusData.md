# ApplicationServerInfoSyncStatusData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**crypto_code** | Option<**String**> | The CryptoCode of the crypto currency (eg. BTC) | [optional]
**node_information** | Option<[**crate::models::ApplicationServerInfoNodeStatusData**](ApplicationServerInfoNodeStatusData.md)> |  | [optional]
**chain_height** | Option<**i32**> | The height of the chain of header of the internal indexer | [optional]
**sync_height** | Option<**f32**> | The height of the latest indexed block of the internal indexer | [optional]
**available** | Option<**bool**> | True if the full node and the indexer are fully synchronized | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


