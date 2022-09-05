# CreateOnChainTransactionRequestDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<**String**> | A wallet address or a BIP21 payment link | [optional]
**amount** | Option<**String**> | The amount to send. If `destination` is a BIP21 link, the amount must be the same or null. | [optional]
**subtract_from_amount** | Option<**bool**> | Whether to subtract the transaction fee from the provided amount. This makes the receiver receive less, or in other words: he or she pays the transaction fee. Also useful if you want to clear out your wallet. Must be false if `destination` is a BIP21 link | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


