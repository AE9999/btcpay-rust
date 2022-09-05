# InvoiceMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | You can use this property to store the ID of an external system. We allow you to search in the invoice list based on this ID. | [optional]
**order_url** | Option<**String**> | You can use this property to store the URL to the order of an external system. This makes navigating to the order easier. | [optional]
**pos_data** | Option<[**crate::models::OneOfstringobject**](oneOf<string,object>.md)> |  | [optional]
**buyer_name** | Option<**String**> |  | [optional]
**buyer_email** | Option<**String**> |  | [optional]
**buyer_country** | Option<**String**> |  | [optional]
**buyer_zip** | Option<**String**> |  | [optional]
**buyer_state** | Option<**String**> |  | [optional]
**buyer_city** | Option<**String**> |  | [optional]
**buyer_address1** | Option<**String**> |  | [optional]
**buyer_address2** | Option<**String**> |  | [optional]
**buyer_phone** | Option<**String**> |  | [optional]
**item_desc** | Option<**String**> |  | [optional]
**item_code** | Option<**String**> |  | [optional]
**physical** | Option<**String**> |  | [optional]
**tax_included** | Option<**f32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


