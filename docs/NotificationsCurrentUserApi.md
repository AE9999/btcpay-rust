# \NotificationsCurrentUserApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notifications_delete_notification**](NotificationsCurrentUserApi.md#notifications_delete_notification) | **DELETE** /api/v1/users/me/notifications/{id} | Remove Notification
[**notifications_get_notification**](NotificationsCurrentUserApi.md#notifications_get_notification) | **GET** /api/v1/users/me/notifications/{id} | Get notification
[**notifications_get_notifications**](NotificationsCurrentUserApi.md#notifications_get_notifications) | **GET** /api/v1/users/me/notifications | Get notifications
[**notifications_update_notification**](NotificationsCurrentUserApi.md#notifications_update_notification) | **PUT** /api/v1/users/me/notifications/{id} | Update notification



## notifications_delete_notification

> notifications_delete_notification(id)
Remove Notification

Removes the specified notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The notification to remove | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_get_notification

> crate::models::NotificationData notifications_get_notification(id)
Get notification

View information about the specified notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The notification to fetch | [required] |

### Return type

[**crate::models::NotificationData**](NotificationData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_get_notifications

> crate::models::NotificationData notifications_get_notifications(seen, skip, take)
Get notifications

View current user's notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seen** | Option<**String**> | filter by seen notifications |  |
**skip** | Option<**f32**> | Number of records to skip |  |
**take** | Option<**f32**> | Number of records returned in response |  |

### Return type

[**crate::models::NotificationData**](NotificationData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_update_notification

> crate::models::NotificationData notifications_update_notification(id, update_notification)
Update notification

Updates the notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The notification to update | [required] |
**update_notification** | [**UpdateNotification**](UpdateNotification.md) |  | [required] |

### Return type

[**crate::models::NotificationData**](NotificationData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

