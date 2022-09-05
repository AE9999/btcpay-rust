# StoreBaseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the store | [optional]
**website** | Option<**String**> | The absolute url of the store | [optional]
**default_currency** | Option<**String**> | The default currency of the store | [optional][default to USD]
**invoice_expiration** | Option<[**crate::models::TimeSpanSeconds**](TimeSpanSeconds.md)> | The time after which an invoice is considered expired if not paid. The value will be rounded down to a minute. | [optional]
**monitoring_expiration** | Option<[**crate::models::TimeSpanSeconds**](TimeSpanSeconds.md)> | The time after which an invoice which has been paid but not confirmed will be considered invalid. The value will be rounded down to a minute. | [optional]
**speed_policy** | Option<[**crate::models::SpeedPolicy**](SpeedPolicy.md)> |  | [optional]
**lightning_description_template** | Option<**String**> | The BOLT11 description of the lightning invoice in the checkout. You can use placeholders '{StoreName}', '{ItemDescription}' and '{OrderId}'. | [optional]
**payment_tolerance** | Option<**f64**> | Consider an invoice fully paid, even if the payment is missing 'x' % of the full amount. | [optional][default to 0]
**anyone_can_create_invoice** | Option<**bool**> | If true, then no authentication is needed to create invoices on this store. | [optional][default to false]
**requires_refund_email** | Option<**bool**> | If true, the checkout page will ask to enter an email address before accessing payment information. | [optional][default to false]
**receipt** | Option<[**crate::models::ReceiptOptions**](ReceiptOptions.md)> |  | [optional]
**lightning_amount_in_satoshi** | Option<**bool**> | If true, lightning payment methods show amount in satoshi in the checkout page. | [optional][default to false]
**lightning_private_route_hints** | Option<**bool**> | Should private route hints be included in the lightning payment of the checkout page. | [optional][default to false]
**on_chain_with_ln_invoice_fallback** | Option<**bool**> | Include lightning invoice fallback to on-chain BIP21 payment url. | [optional][default to false]
**redirect_automatically** | Option<**bool**> | After successfull payment, should the checkout page redirect the user automatically to the redirect URL of the invoice? | [optional][default to false]
**show_recommended_fee** | Option<**bool**> |  | [optional][default to true]
**recommended_fee_block_target** | Option<**i32**> | The fee rate recommendation in the checkout page for the on-chain payment to be confirmed after 'x' blocks. | [optional][default to 1]
**default_lang** | Option<**String**> | The default language to use in the checkout page. (The different translations available are listed [here](https://github.com/btcpayserver/btcpayserver/tree/master/BTCPayServer/wwwroot/locales) | [optional][default to en]
**custom_logo** | Option<**String**> | URL to a logo to include in the checkout page. | [optional]
**custom_css** | Option<**String**> | URL to a CSS stylesheet to include in the checkout page | [optional]
**html_title** | Option<**String**> | The HTML title of the checkout page (when you over the tab in your browser) | [optional]
**network_fee_mode** | Option<[**crate::models::NetworkFeeMode**](NetworkFeeMode.md)> |  | [optional]
**pay_join_enabled** | Option<**bool**> | If true, payjoin will be proposed in the checkout page if possible. ([More information](https://docs.btcpayserver.org/Payjoin/)) | [optional][default to false]
**lazy_payment_methods** | Option<**bool**> | If true, payment methods are enabled individually upon user interaction in the invoice | [optional][default to false]
**default_payment_method** | Option<**String**> | The default payment method to load when displaying an invoice. It can be in the format of `BTC_LightningNetwork` to specify Lightning to be the default or `BTC_OnChain`/ `BTC` for on-chain to be the default.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


