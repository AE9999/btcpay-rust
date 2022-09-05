# LnurlPayPaymentMethodData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether the payment method is enabled. Note that this can only enabled when a Lightning Network payment method is available and enabled | [optional]
**crypto_code** | Option<**String**> | Crypto code of the payment method | [optional]
**use_bech32_scheme** | Option<**bool**> | Whether to use [LUD-01](https://github.com/fiatjaf/lnurl-rfc/blob/luds/01.md)'s bech32 format or to use [LUD-17](https://github.com/fiatjaf/lnurl-rfc/blob/luds/17.md) url formatting.  | [optional]
**enable_for_standard_invoices** | Option<**bool**> | Whether to allow this payment method to also be used for standard invoices and not just topup invoices. | [optional]
**lud12_enabled** | Option<**bool**> | Allow comments to be passed on via lnurl. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


