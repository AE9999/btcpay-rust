# CustodianAccountData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique code of the customer's account with this custodian. The format depends on the custodian. | [optional]
**store_id** | Option<**String**> | The store ID. | [optional]
**custodian_code** | Option<**String**> | The code for the custodian. | [optional]
**name** | Option<**String**> | The name of the custodian account. | [optional]
**asset_balances** | Option<[**Vec<crate::models::AssetBalanceData>**](AssetBalanceData.md)> | A real-time loaded list of all assets (fiat and crypto) on this custodian and the quantity held in the account. Assets with qty 0 can be omitted. | [optional]
**config** | Option<[**serde_json::Value**](.md)> | The configuration of this custodian account. Specific contents depend on the custodian and your access permissions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


