# CustodianData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | The unique code of the custodian. | [optional]
**label** | Option<**String**> | The name of the custodian. | [optional]
**depositable_payment_methods** | Option<**Vec<String>**> | A list of payment methods (crypto code + network) you can deposit to the custodian. | [optional]
**withdrawable_payment_methods** | Option<**Vec<String>**> | A list of payment methods (crypto code + network) you can withdraw from the custodian. | [optional]
**tradable_asset_pairs** | Option<[**Vec<crate::models::AssetPairData>**](AssetPairData.md)> | A list of tradable asset pair objects, or NULL if the custodian cannot trades/convert assets. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


