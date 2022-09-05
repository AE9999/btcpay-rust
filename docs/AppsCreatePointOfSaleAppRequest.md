# AppsCreatePointOfSaleAppRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_name** | Option<**String**> | The name of the app (shown in admin UI) | [optional]
**title** | Option<**String**> | The title of the app (shown to the user) | [optional]
**description** | Option<**String**> | The description of the app | [optional]
**template** | Option<**String**> | Template for items available in the app | [optional]
**default_view** | Option<**String**> | Template for items available in the app | [optional]
**currency** | Option<**String**> | Currency to use for the app. Defaults to the currency used by the store if not specified | [optional]
**show_custom_amount** | Option<**bool**> | Whether to include a special item in the store which allows user to input a custom payment amount | [optional][default to true]
**show_discount** | Option<**bool**> | Whether to allow user to input a discount amount. Applies to Cart view only. Not recommended for customer self-checkout | [optional][default to true]
**enable_tips** | Option<**bool**> | Whether to allow user to input a tip amount. Applies to Cart and Light views only | [optional][default to true]
**custom_amount_pay_button_text** | Option<**String**> | Payment button text which appears for items which allow user to input a custom amount | [optional][default to Pay]
**fixed_amount_pay_button_text** | Option<**String**> | Payment button text which appears for items which have a fixed price | [optional][default to Buy for {PRICE_HERE}]
**tip_text** | Option<**String**> | Prompt which appears next to the tip amount field if tipping is enabled | [optional][default to Do you want to leave a tip?]
**custom_css_link** | Option<**String**> | Link to a custom CSS stylesheet to be used in the app | [optional]
**embedded_css** | Option<**String**> | Custom CSS to embed into the app | [optional]
**notification_url** | Option<**String**> | Callback notification url to POST to once when invoice is paid for and once when there are enough blockchain confirmations | [optional]
**redirect_url** | Option<**String**> | URL to redirect user to once invoice is paid | [optional]
**redirect_automatically** | Option<**bool**> | Whether to redirect user to redirect URL automatically once invoice is paid. Defaults to what is set in the store settings | [optional]
**requires_refund_email** | Option<**bool**> | Whether to redirect user to redirect URL automatically once invoice is paid. Defaults to what is set in the store settings | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


