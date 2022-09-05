# CheckoutOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**speed_policy** | Option<[**crate::models::SpeedPolicy**](SpeedPolicy.md)> |  | [optional]
**payment_methods** | Option<**Vec<String>**> | A specific set of payment methods to use for this invoice (ie. BTC, BTC-LightningNetwork). By default, select all payment methods enabled in the store. | [optional]
**default_payment_method** | Option<**String**> | Default payment type for the invoice (e.g., BTC, BTC-LightningNetwork). Default payment method set for the store is used if this parameter is not specified. | [optional]
**expiration_minutes** | Option<[**crate::models::TimeSpanMinutes**](TimeSpanMinutes.md)> | The number of minutes after which an invoice becomes expired. Defaults to the store's settings. (The default store settings is 15) | [optional]
**monitoring_minutes** | Option<[**crate::models::TimeSpanMinutes**](TimeSpanMinutes.md)> | The number of minutes after an invoice expired after which we are still monitoring for incoming payments. Defaults to the store's settings. (The default store settings is 1440, 1 day) | [optional]
**payment_tolerance** | Option<**f64**> | A percentage determining whether to count the invoice as paid when the invoice is paid within the specified margin of error. Defaults to the store's settings. (The default store settings is 100) | [optional]
**redirect_url** | Option<**String**> | When the customer has paid the invoice, the URL where the customer will be redirected when clicking on the `return to store` button. You can use placeholders `{InvoiceId}` or `{OrderId}` in the URL, BTCPay Server will replace those with this invoice `id` or `metadata.orderId` respectively. | [optional]
**redirect_automatically** | Option<**bool**> | When the customer has paid the invoice, and a `redirectURL` is set, the checkout is redirected to `redirectURL` automatically if `redirectAutomatically` is true. Defaults to the store's settings. (The default store settings is false) | [optional]
**requires_refund_email** | Option<**bool**> | Invoice will require user to provide a refund email if this option is set to `true`. Has no effect if `buyerEmail` metadata is set as there is no email to collect in this case. | [optional]
**default_language** | Option<**String**> | The language code (eg. en-US, en, fr-FR...) of the language presented to your customer in the checkout page. BTCPay Server tries to match the best language available. If null or not set, will fallback on the store's default language. You can see the list of language codes with [this operation](#operation/langCodes). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


