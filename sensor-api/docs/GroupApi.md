# \GroupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_members_by_uid**](GroupApi.md#add_group_members_by_uid) | **PUT** /api/v1/group/{group_uid}/members/add | Add Group Members By Uid
[**create_group**](GroupApi.md#create_group) | **POST** /api/v1/group/create | Create Group
[**get_all_group_uids**](GroupApi.md#get_all_group_uids) | **GET** /api/v1/group/ | Get All Group Uids
[**get_group_by_uid**](GroupApi.md#get_group_by_uid) | **GET** /api/v1/group/{group_uid} | Get Group By Uid
[**get_group_members_by_uid**](GroupApi.md#get_group_members_by_uid) | **GET** /api/v1/group/{group_uid}/members | Get Group Members By Uid
[**get_group_sensors_by_uid**](GroupApi.md#get_group_sensors_by_uid) | **GET** /api/v1/group/{group_uid}/sensors | Get Group Sensors By Uid
[**remove_group_members_by_uid**](GroupApi.md#remove_group_members_by_uid) | **PUT** /api/v1/group/{group_uid}/members/remove | Remove Group Members By Uid
[**update_group_by_uid**](GroupApi.md#update_group_by_uid) | **PUT** /api/v1/group/{group_uid} | Update Group By Uid
[**update_group_sensors_by_uid**](GroupApi.md#update_group_sensors_by_uid) | **PUT** /api/v1/group/{group_uid}/sensors/update/{sensor_uid} | Update Group Sensors By Uid



## add_group_members_by_uid

> models::GroupMember add_group_members_by_uid(group_uid, user_mail)
Add Group Members By Uid

Add a user to the group.  Args: ----     user_mail: The email address of the user to add.     group: The requested group.     current_active_user: A logged in and active user.  Returns: -------     set[GroupMember]: A set of group members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |
**user_mail** | [**UserMail**](UserMail.md) |  | [required] |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_group_by_uid

> models::Group get_group_by_uid(group_uid)
Get Group By Uid

Retrieve a group by uid.  Args: ----     group: The requested group.     current_active_user: A logged in and active user.  Returns: -------     Group: A schema of the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members_by_uid

> Vec<models::GroupMember> get_group_members_by_uid(group_uid)
Get Group Members By Uid

Retrieve a groups members by uid.  Args: ----     group: The requested group.     current_active_user: A logged in and active user.  Returns: -------     set[GroupMember]: A set of group members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |

### Return type

[**Vec<models::GroupMember>**](GroupMember.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_sensors_by_uid

> Vec<String> get_group_sensors_by_uid(group_uid, skip, limit)
Get Group Sensors By Uid

Retrieve a groups sensors by uid.  Args: ----     group: The requested group.     current_active_user: A logged in and active user.     skip (int, optional): Skip sensors for pagination. [Defaults to 0].     limit (int, optional): Limit sensors for pagination. [Defaults to 25].  Returns: -------     set[str]: A set of sensor uids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**Vec<String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_members_by_uid

> models::GroupMember remove_group_members_by_uid(group_uid, user_uid)
Remove Group Members By Uid

Remove a user from the group.  Args: ----     user_uid: The uid of the user to remove.     group: The requested group.     current_active_user: A logged in and active user.  Returns: -------     set[GroupMember]: A set of group members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |
**user_uid** | **String** |  | [required] |

### Return type

[**models::GroupMember**](GroupMember.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_by_uid

> models::Group update_group_by_uid(group_uid, group_update)
Update Group By Uid

Edit a group by uid.  Args: ----     group_update: The new data for the group.     group: The requested group.     current_active_user: A logged in and active user.  Returns: -------     Group: A schema of the updated group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |
**group_update** | [**GroupUpdate**](GroupUpdate.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_sensors_by_uid

> models::Permission update_group_sensors_by_uid(group_uid, sensor_uid, permissions)
Update Group Sensors By Uid

Update a groups permission on a sensor.  The permission literal exists, because fastapi (or another dependency) have issues with parsing the int flags.  Args: ----     group: The requested group.     sensor: The sensor to update the permissions of.     permissions: The permission that the group is given for the sensor.     current_active_user: A logged in and active user.  Returns: -------     str: The uid of the updated sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_uid** | **String** |  | [required] |
**sensor_uid** | **String** |  | [required] |
**permissions** | **String** |  | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

