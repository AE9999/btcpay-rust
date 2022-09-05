# LightningNetworkPaymentMethodData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_string** | Option<**String**> | The lightning connection string. Set to 'Internal Node' to use the internal node. (See [this doc](https://github.com/btcpayserver/BTCPayServer.Lightning/blob/master/README.md#examples) for some example) | [optional]
**disable_bolt11_payment_option** | Option<**bool**> | Whether to disable generation of bolt11 invoices. Useful when wanting to only use LNURL Pay exclusively. | [optional]
**enabled** | Option<**bool**> | Whether the payment method is enabled | [optional]
**crypto_code** | Option<**String**> | Crypto code of the payment method | [optional]
**payment_method** | Option<**String**> | The payment method | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


