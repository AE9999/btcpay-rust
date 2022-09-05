# WithdrawalResultData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset** | Option<**String**> | The asset that is being withdrawn. | [optional]
**payment_method** | Option<**String**> | The payment method that is used (crypto code + network). | [optional]
**ledger_entries** | Option<[**Vec<crate::models::LedgerEntryData>**](LedgerEntryData.md)> | The asset entries that were changed during the withdrawal. The first item is always the withdrawal itself. It could also includes ledger entries for the costs and may include credits or exchange tokens to give a discount. | [optional]
**withdrawal_id** | Option<**String**> | The unique ID of the withdrawal used by the exchange. | [optional]
**account_id** | Option<**String**> | The unique ID of the custodian account used. | [optional]
**custodian_code** | Option<**String**> | The code of the custodian used. | [optional]
**status** | Option<**String**> | The status of the withdrawal: 'Queued', 'Complete', 'Failed' or 'Unknown'. | [optional]
**transaction_id** | Option<**String**> | The transaction ID on the blockchain once the withdrawal has been executed. | [optional]
**target_address** | Option<**String**> | The address where the funds were sent to once the withdrawal has been executed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


