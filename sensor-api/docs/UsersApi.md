# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /api/v1/user/create | Create User
[**get_access_token_by_login**](UsersApi.md#get_access_token_by_login) | **POST** /api/v1/user/get_token | Get Access Token By Login
[**get_all_user_uids**](UsersApi.md#get_all_user_uids) | **GET** /api/v1/user/ | Get All User Uids
[**get_own_sensors**](UsersApi.md#get_own_sensors) | **GET** /api/v1/user/me/sensors | Get Own Sensors
[**get_own_user**](UsersApi.md#get_own_user) | **GET** /api/v1/user/me | Get Own User
[**get_refreshed_access_token**](UsersApi.md#get_refreshed_access_token) | **POST** /api/v1/user/refresh_token | Get Refreshed Access Token
[**get_sensors**](UsersApi.md#get_sensors) | **GET** /api/v1/user/me/sensors_full | Get Sensors
[**get_user_by_uid**](UsersApi.md#get_user_by_uid) | **GET** /api/v1/user/{user_uid} | Get User By Uid
[**resend_verification_email**](UsersApi.md#resend_verification_email) | **PUT** /api/v1/user/verify/resend | Resend Verification Email
[**reset_password**](UsersApi.md#reset_password) | **POST** /api/v1/user/reset_password | Reset Password
[**update_own_user**](UsersApi.md#update_own_user) | **PUT** /api/v1/user/me | Update Own User
[**update_user_by_uid**](UsersApi.md#update_user_by_uid) | **PUT** /api/v1/user/{user_uid} | Update User By Uid
[**verify_user**](UsersApi.md#verify_user) | **PUT** /api/v1/user/verify/{token} | Verify User



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


## get_access_token_by_login

> models::Token get_access_token_by_login(username, password, grant_type, scope, client_id, client_secret)
Get Access Token By Login

Retrieve an access token for the API by logging in.  Args: ----     form_data: The login form data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**password** | **String** |  | [required] |
**grant_type** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |[default to ]
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |

### Return type

[**models::Token**](Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
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


## get_own_sensors

> std::collections::HashMap<String, models::PermissionLevel> get_own_sensors()
Get Own Sensors

Retrieve own sensors.  Returns -------     list: A list of sensor uids.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::PermissionLevel>**](PermissionLevel.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_own_user

> models::User get_own_user()
Get Own User

Retrieve own user.  Returns -------     User: The own user schema.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_refreshed_access_token

> models::Token get_refreshed_access_token()
Get Refreshed Access Token

Refresh the current access token for an already signed-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Token**](Token.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sensors

> models::SensorListResponse get_sensors(skip, limit, include_modules)
Get Sensors

Return list of sensors for the active user.  Args: ----     skip (int, optional): Skip sensors for pagination. [Defaults to 0].     limit (int, optional): Limit sensors for pagination. [Defaults to 25].     include_modules (set[str], optional): List of module names to include.         If None, all are returned. Defaults to None.  Returns: -------     SensorListResponse: List of sensors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 25]
**include_modules** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::SensorListResponse**](SensorListResponse.md)

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


## resend_verification_email

> serde_json::Value resend_verification_email(user_mail)
Resend Verification Email

Resend a verification mail to a user.  Args: ----     mail_schema: The email of the user to resend the verification mail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_mail** | [**UserMail**](UserMail.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> serde_json::Value reset_password(user_mail)
Reset Password

Send a reset password mail to a user.  Args: ----     mail_schema: The email of the user to reset the password from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_mail** | [**UserMail**](UserMail.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_own_user

> models::User update_own_user(user_self_update)
Update Own User

Update own user.  Args: ----     user_update: The data to update.  Returns: -------     User: The own updated user schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_self_update** | [**UserSelfUpdate**](UserSelfUpdate.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
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


## verify_user

> models::User verify_user(token)
Verify User

Verify a user.  Args: ----     token: The verification token of the user to verify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

