# \AppsApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_create_point_of_sale_app**](AppsApi.md#apps_create_point_of_sale_app) | **POST** /api/v1/stores/{storeId}/apps/pos | Create a new Point of Sale app
[**apps_delete_point_of_sale_app**](AppsApi.md#apps_delete_point_of_sale_app) | **DELETE** /api/v1/apps/{appId} | Delete app
[**apps_get_point_of_sale_app**](AppsApi.md#apps_get_point_of_sale_app) | **GET** /api/v1/apps/{appId} | Get basic app data



## apps_create_point_of_sale_app

> crate::models::PointOfSaleAppData apps_create_point_of_sale_app(store_id, apps_create_point_of_sale_app_request)
Create a new Point of Sale app

Point of Sale apps allows accepting payments for items in a virtual store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store ID | [required] |
**apps_create_point_of_sale_app_request** | Option<[**AppsCreatePointOfSaleAppRequest**](AppsCreatePointOfSaleAppRequest.md)> |  |  |

### Return type

[**crate::models::PointOfSaleAppData**](PointOfSaleAppData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_delete_point_of_sale_app

> apps_delete_point_of_sale_app(app_id)
Delete app

Deletes apps with specified ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_point_of_sale_app

> crate::models::BasicAppData apps_get_point_of_sale_app(app_id)
Get basic app data

Returns basic app data shared between all types of apps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**crate::models::BasicAppData**](BasicAppData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

