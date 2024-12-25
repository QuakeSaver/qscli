# \EventApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_events**](EventApi.md#search_events) | **GET** /api/v1/event/search | Search Events



## search_events

> serde_json::Value search_events(start_time, end_time, min_magnitude, max_magnitude, catalogue)
Search Events

Get the requesters public IP.  Returns -------     str: The IP address of the requester.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<**String**> |  |  |
**end_time** | Option<**String**> |  |  |
**min_magnitude** | Option<**f64**> |  |  |
**max_magnitude** | Option<**f64**> |  |  |
**catalogue** | Option<**String**> |  |  |[default to geofon]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

