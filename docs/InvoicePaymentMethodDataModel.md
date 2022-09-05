# InvoicePaymentMethodDataModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_method** | Option<**String**> | Payment method available for the invoice (e.g., \"BTC\" or \"BTC-LightningNetwork\" or \"BTC-LNURLPAY\") | [optional]
**crypto_code** | Option<**String**> | Crypto code of the payment method (e.g., \"BTC\" or \"LTC\") | [optional]
**destination** | Option<**String**> | The destination the payment must be made to | [optional]
**payment_link** | Option<**String**> | A payment link that helps pay to the payment destination | [optional]
**rate** | Option<**String**> | The rate between this payment method's currency and the invoice currency | [optional]
**payment_method_paid** | Option<**String**> | The amount paid by this payment method | [optional]
**total_paid** | Option<**String**> | The total amount paid by all payment methods to the invoice, converted to this payment method's currency | [optional]
**due** | Option<**String**> | The total amount left to be paid, converted to this payment method's currency (will be negative if overpaid) | [optional]
**amount** | Option<**String**> | The invoice amount, converted to this payment method's currency | [optional]
**network_fee** | Option<**String**> | The added merchant fee to pay for network costs of this payment method. | [optional]
**payments** | Option<[**Vec<crate::models::Payment>**](Payment.md)> | Payments made with this payment method. | [optional]
**activated** | Option<**bool**> | If the payment method is activated (when lazy payments option is enabled | [optional]
**additional_data** | Option<[**crate::models::InvoicePaymentMethodDataModelAdditionalData**](InvoicePaymentMethodDataModel_additionalData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


