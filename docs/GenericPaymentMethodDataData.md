# GenericPaymentMethodDataData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**derivation_scheme** | Option<**String**> | The derivation scheme | [optional]
**label** | Option<**String**> | A label that will be shown in the UI | [optional]
**account_key_path** | Option<**String**> | The wallet fingerprint followed by the keypath to derive the account key used for signing operation or creating PSBTs | [optional]
**connection_string** | Option<**String**> | The lightning connection string. Set to 'Internal Node' to use the internal node. (See [this doc](https://github.com/btcpayserver/BTCPayServer.Lightning/blob/master/README.md#examples) for some example) | [optional]
**disable_bolt11_payment_option** | Option<**bool**> | Whether to disable generation of bolt11 invoices. Useful when wanting to only use LNURL Pay exclusively. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


