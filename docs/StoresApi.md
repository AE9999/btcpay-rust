# \StoresApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stores_create_store**](StoresApi.md#stores_create_store) | **POST** /api/v1/stores | Create a new store
[**stores_delete_store**](StoresApi.md#stores_delete_store) | **DELETE** /api/v1/stores/{storeId} | Remove Store
[**stores_get_store**](StoresApi.md#stores_get_store) | **GET** /api/v1/stores/{storeId} | Get store
[**stores_get_stores**](StoresApi.md#stores_get_stores) | **GET** /api/v1/stores | Get stores
[**stores_update_store**](StoresApi.md#stores_update_store) | **PUT** /api/v1/stores/{storeId} | Update store



## stores_create_store

> crate::models::StoreData stores_create_store(store_base_data)
Create a new store

Create a new store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_base_data** | [**StoreBaseData**](StoreBaseData.md) |  | [required] |

### Return type

[**crate::models::StoreData**](StoreData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_delete_store

> stores_delete_store(store_id)
Remove Store

Removes the specified store. If there is another user with access, only your access will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to remove | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_get_store

> crate::models::StoreData stores_get_store(store_id)
Get store

View information about the specified store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**crate::models::StoreData**](StoreData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_get_stores

> Vec<crate::models::StoreData> stores_get_stores()
Get stores

View information about the available stores

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::StoreData>**](StoreData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_update_store

> crate::models::StoreData stores_update_store(store_id, store_data)
Update store

Update the specified store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to update | [required] |
**store_data** | [**StoreData**](StoreData.md) |  | [required] |

### Return type

[**crate::models::StoreData**](StoreData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

