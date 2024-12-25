# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](AdminApi.md#create_group) | **POST** /api/v1/group/create | Create Group
[**create_user**](AdminApi.md#create_user) | **POST** /api/v1/user/create | Create User
[**get_all_group_uids**](AdminApi.md#get_all_group_uids) | **GET** /api/v1/group/ | Get All Group Uids
[**get_all_sensor_uids**](AdminApi.md#get_all_sensor_uids) | **GET** /api/v1/sensors/ | Get All Sensor Uids
[**get_all_user_uids**](AdminApi.md#get_all_user_uids) | **GET** /api/v1/user/ | Get All User Uids
[**get_user_by_uid**](AdminApi.md#get_user_by_uid) | **GET** /api/v1/user/{user_uid} | Get User By Uid
[**update_user_by_uid**](AdminApi.md#update_user_by_uid) | **PUT** /api/v1/user/{user_uid} | Update User By Uid



## create_group

> models::Group create_group(group_create)
Create Group

Create a new group.  Args: ----     group_to_create: The data to create a new group from.     current_active_user: A logged in and active user.  Returns: -------     User: The own updated user schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_create** | [**GroupCreate**](GroupCreate.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(user_create)
Create User

Create a new user.  Args: ----     user_to_create: The data to create a new user from.  Returns: -------     User: The own updated user schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create** | [**UserCreate**](UserCreate.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_group_uids

> Vec<String> get_all_group_uids()
Get All Group Uids

Retrieve a list of all existing group.  Args: ----     current_active_superuser: A logged in and active superuser.  Returns: -------     list: A list of all existing group IDs.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_sensor_uids

> Vec<String> get_all_sensor_uids()
Get All Sensor Uids

Get all sensor uids.  Args: ----     current_active_superuser: A logged in and active superuser.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_uids

> Vec<String> get_all_user_uids(skip, limit)
Get All User Uids

Retrieve a list of all existing users.  Args: ----     skip (int, optional): Skip sensors for pagination. [Defaults to 0]     limit (int, optional): Limit sensors for pagination. [Defaults to 0]  Returns: -------     list: A list of all existing user IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 0]

### Return type

**Vec<String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_uid

> models::User get_user_by_uid(user_uid)
Get User By Uid

Retrieve a user by uid.  Args: ----     user_uid: The uid of the user to get.  Returns: -------     User: A schema of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uid** | **String** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_by_uid

> models::User update_user_by_uid(user_uid, user_superuser_update)
Update User By Uid

Update user by uid.  Args: ----     user_uid: The uid of the user to update.     user_update: The data to update.  Returns: -------     User: The own updated user schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_uid** | **String** |  | [required] |
**user_superuser_update** | [**UserSuperuserUpdate**](UserSuperuserUpdate.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

