# CreateOnChainTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destinations** | Option<[**Vec<crate::models::CreateOnChainTransactionRequestDestination>**](CreateOnChainTransactionRequestDestination.md)> | What and where to send money | [optional]
**feerate** | Option<**f32**> | Transaction fee. | [optional]
**proceed_with_payjoin** | Option<**bool**> | Whether to attempt to do a BIP78 payjoin if one of the destinations is a BIP21 with payjoin enabled | [optional][default to true]
**proceed_with_broadcast** | Option<**bool**> | Whether to broadcast the transaction after creating it or to simply return the transaction in hex format. | [optional][default to true]
**no_change** | Option<**bool**> | Whether to send all the spent coins to the destinations (THIS CAN COST YOU SIGNIFICANT AMOUNTS OF MONEY, LEAVE FALSE UNLESS YOU KNOW WHAT YOU ARE DOING). | [optional][default to false]
**rbf** | Option<**bool**> | Whether to enable RBF for the transaction. Leave blank to have it random (beneficial to privacy) | [optional]
**exclude_unconfirmed** | Option<**bool**> | Whether to exclude unconfirmed UTXOs from the transaction. | [optional][default to false]
**selected_inputs** | Option<**Vec<String>**> | Restrict the creation of the transactions from the outpoints provided ONLY (coin selection) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


