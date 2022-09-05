# TradeResultData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_asset** | Option<**String**> | The asset to trade. | [optional]
**to_asset** | Option<**String**> | The asset you want. | [optional]
**ledger_entries** | Option<[**Vec<crate::models::LedgerEntryData>**](LedgerEntryData.md)> | The asset entries that were changed during the trade. This is an array of at least 2 items with the asset sold and the asset gained. It may also include ledger entries for the costs of the trade and possibly exchange tokens used. | [optional]
**trade_id** | Option<**String**> | The unique ID of the trade used by the exchange. This ID can be used to get the details of this trade at a later time. | [optional]
**account_id** | Option<**String**> | The unique ID of the custodian account used. | [optional]
**custodian_code** | Option<**String**> | The code of the custodian used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


