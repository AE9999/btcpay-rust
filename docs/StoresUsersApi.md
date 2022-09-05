# \StoresUsersApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stores_add_store_user**](StoresUsersApi.md#stores_add_store_user) | **POST** /api/v1/stores/{storeId}/users | Add a store user
[**stores_get_store_users**](StoresUsersApi.md#stores_get_store_users) | **GET** /api/v1/stores/{storeId}/users | Get store users
[**stores_remove_store_user**](StoresUsersApi.md#stores_remove_store_user) | **DELETE** /api/v1/stores/{storeId}/users/{userId} | Remove Store User



## stores_add_store_user

> stores_add_store_user(store_id, store_user_data)
Add a store user

Add a store user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to add the user to | [required] |
**store_user_data** | [**StoreUserData**](StoreUserData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_get_store_users

> Vec<crate::models::StoreData> stores_get_store_users(store_id)
Get store users

View users of the specified store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**Vec<crate::models::StoreData>**](StoreData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_remove_store_user

> stores_remove_store_user(store_id, user_id)
Remove Store User

Removes the specified store user. If there is no other owner, this endpoint will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store | [required] |
**user_id** | **String** | The user | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

