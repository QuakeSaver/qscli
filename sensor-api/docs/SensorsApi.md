# \SensorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_to_sensor**](SensorsApi.md#add_group_to_sensor) | **POST** /api/v1/sensors/{sensor_uid}/add_group | Add Group To Sensor
[**delete_data_product_by_uid**](SensorsApi.md#delete_data_product_by_uid) | **DELETE** /api/v1/sensors/{sensor_uid}/data_products/{data_product_uid} | Delete Data Product By Uid
[**delete_sensor_by_uid**](SensorsApi.md#delete_sensor_by_uid) | **DELETE** /api/v1/sensors/{sensor_uid} | Delete Sensor By Uid
[**delete_warning**](SensorsApi.md#delete_warning) | **DELETE** /api/v1/sensors/{sensor_uid}/warning | Delete Warning
[**get_all_sensor_uids**](SensorsApi.md#get_all_sensor_uids) | **GET** /api/v1/sensors/ | Get All Sensor Uids
[**get_broker_auth**](SensorsApi.md#get_broker_auth) | **POST** /api/v1/sensors/broker_auth | Get Broker Auth
[**get_data_product_by_name**](SensorsApi.md#get_data_product_by_name) | **POST** /api/v1/sensors/{sensor_uid}/data_products/{data_product_name} | Get Data Product By Name
[**get_data_product_by_uid**](SensorsApi.md#get_data_product_by_uid) | **GET** /api/v1/sensors/{sensor_uid}/data_products/{data_product_uid} | Get Data Product By Uid
[**get_measurement_from_sensor**](SensorsApi.md#get_measurement_from_sensor) | **POST** /api/v1/sensors/{sensor_uid}/measurements | Get Measurement From Sensor
[**get_own_sensors**](SensorsApi.md#get_own_sensors) | **GET** /api/v1/user/me/sensors | Get Own Sensors
[**get_sensor_by_uid**](SensorsApi.md#get_sensor_by_uid) | **GET** /api/v1/sensors/{sensor_uid} | Get Sensor By Uid
[**remove_group_from_sensor**](SensorsApi.md#remove_group_from_sensor) | **DELETE** /api/v1/sensors/{sensor_uid}/group/{group_uid} | Remove Group From Sensor
[**set_data_product_limit**](SensorsApi.md#set_data_product_limit) | **PUT** /api/v1/sensors/{sensor_uid}/set_data_product_limit | Set Data Product Limit
[**trigger_sensor_action**](SensorsApi.md#trigger_sensor_action) | **POST** /api/v1/sensors/{sensor_uid}/action | Trigger Sensor Action
[**update_group_of_sensor**](SensorsApi.md#update_group_of_sensor) | **PUT** /api/v1/sensors/{sensor_uid}/group/{group_uid} | Update Group Of Sensor
[**update_primary_group_of_sensor**](SensorsApi.md#update_primary_group_of_sensor) | **PUT** /api/v1/sensors/{sensor_uid}/group/primary | Update Primary Group Of Sensor
[**update_sensor_config_by_uid**](SensorsApi.md#update_sensor_config_by_uid) | **PUT** /api/v1/sensors/{sensor_uid}/config | Update Sensor Config By Uid



## add_group_to_sensor

> models::Sensor add_group_to_sensor(sensor_uid, sensor_group_add)
Add Group To Sensor

Add a group to the sensor.  Args: ----     sensor: The sensor to update.     group_add: The data to for adding a group.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**sensor_group_add** | [**SensorGroupAdd**](SensorGroupAdd.md) |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_product_by_uid

> models::DataProductQueryResult delete_data_product_by_uid(data_product_uid, sensor_uid)
Delete Data Product By Uid

Delete a data product by uid.  Args: ----     data_product_uid: The uid of the data product to delete     sensor: The sensor to get the data product from.  Returns: -------     DataProductQueryResult: A with metadata and a list of all deleted values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_product_uid** | **uuid::Uuid** |  | [required] |
**sensor_uid** | **String** |  | [required] |

### Return type

[**models::DataProductQueryResult**](DataProductQueryResult.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sensor_by_uid

> models::Sensor delete_sensor_by_uid(sensor_uid)
Delete Sensor By Uid

Delete a sensor and all its data by uid.  Args: ----     sensor: The sensor to get.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the deleted sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_warning

> models::Sensor delete_warning(sensor_uid, title)
Delete Warning

Clear a specified warning from the sensor.  Args: ----     sensor: The sensor to remove the warning from.     title: The title of the warning to clear.  Returns: -------     Sensor: The updated sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**title** | **String** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

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


## get_broker_auth

> models::BrokerConfig get_broker_auth(user_authenticate)
Get Broker Auth

Get the authentication details for a sensor from the user providing the auth.  Args: ----     auth: The authentication details to get the sensor auth details.  Returns: -------     SensorAuth: The auth details needed for a sensor to connect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_authenticate** | [**UserAuthenticate**](UserAuthenticate.md) |  | [required] |

### Return type

[**models::BrokerConfig**](BrokerConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_product_by_name

> models::DataProductQueryResult get_data_product_by_name(data_product_name, sensor_uid, start_time, end_time, skip, limit, request_body)
Get Data Product By Name

Get data product values by parameter.  The time boundaries are inclusive.  Args: ----     sensor: The sensor to get the data product from.     skip: The number of entries to skip.     data_product_name: The name of the data product to get.     excluded_attributes: Attribute names to exclude from the data product if exists.     limit: The maximum number of values to get.     [default=100, greater_equal=1, less_equal=100]     start_time: The start of the requested time series.     end_time: The end of the requested time series.  Returns: -------     DataProductQueryResult: A with metadata and a list of all found values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_product_name** | **String** |  | [required] |
**sensor_uid** | **String** |  | [required] |
**start_time** | Option<**String**> |  |  |
**end_time** | Option<**String**> |  |  |
**skip** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 100]
**request_body** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::DataProductQueryResult**](DataProductQueryResult.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_product_by_uid

> models::DataProductQueryResult get_data_product_by_uid(data_product_uid, sensor_uid)
Get Data Product By Uid

Get a data product by uid.  Args: ----     data_product_uid: The uid of the data product to delete     sensor: The sensor to get the data product from.  Returns: -------     DataProductQueryResult: A with metadata and a list of all found values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_product_uid** | **uuid::Uuid** |  | [required] |
**sensor_uid** | **String** |  | [required] |

### Return type

[**models::DataProductQueryResult**](DataProductQueryResult.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_measurement_from_sensor

> models::MeasurementResult get_measurement_from_sensor(sensor_uid, measurement_query)
Get Measurement From Sensor

Get data product values by parameter.  The time boundaries are inclusive.  Args: ----     query: The query to run.     sensor: The sensor to get the data product from.  Returns: -------     MeasurementResult: A result with metadata and a list of all found values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**measurement_query** | [**MeasurementQuery**](MeasurementQuery.md) |  | [required] |

### Return type

[**models::MeasurementResult**](MeasurementResult.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
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


## get_sensor_by_uid

> models::Sensor get_sensor_by_uid(sensor_uid)
Get Sensor By Uid

Get a sensor by uid.  Args: ----     sensor: The sensor to get.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_from_sensor

> models::Sensor remove_group_from_sensor(sensor_uid, group_uid)
Remove Group From Sensor

Remove a group from the sensor.  Args: ----     sensor: The sensor to update.     group: The group to remove.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**group_uid** | **String** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_product_limit

> models::Sensor set_data_product_limit(sensor_uid, limit)
Set Data Product Limit

Set the limit of stored data products.  The limit applies to all data products of the sensor.  Args: ----     sensor: The sensor to set the limit of.     limit: The maximum number of data products stored per data product.  Returns: -------     Sensor: The updated sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**limit** | **i32** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_sensor_action

> std::collections::HashMap<String, String> trigger_sensor_action(sensor_uid, action_name, body)
Trigger Sensor Action

Trigger action on a sensor by uid.  Args: ----     action_name: The name of the action     sensor: The sensor to trigger.     action: The action to send.     current_active_user: A logged in and active user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**action_name** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_of_sensor

> models::Sensor update_group_of_sensor(sensor_uid, group_uid, sensor_group_update)
Update Group Of Sensor

Update a group of the sensor.  Args: ----     sensor: The sensor to update.     group: The group to update.     group_update: The new data for the group.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**group_uid** | **String** |  | [required] |
**sensor_group_update** | [**SensorGroupUpdate**](SensorGroupUpdate.md) |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_primary_group_of_sensor

> models::Sensor update_primary_group_of_sensor(sensor_uid, group_uid)
Update Primary Group Of Sensor

Update the primary group of the sensor.  Args: ----     sensor: The sensor to update.     group: The new primary group.     current_active_user: A logged in and active user.  Returns: -------     Sensor: The schema of the sensor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**group_uid** | **String** |  | [required] |

### Return type

[**models::Sensor**](Sensor.md)

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sensor_config_by_uid

> std::collections::HashMap<String, String> update_sensor_config_by_uid(sensor_uid, config_name, body)
Update Sensor Config By Uid

Update config of a sensor by uid.  Args: ----     sensor: The sensor to update.     config: The new sensor config data.     config_name: The name of the config.     current_active_user: A logged in and active user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensor_uid** | **String** |  | [required] |
**config_name** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[OAuth2PasswordBearer](../README.md#OAuth2PasswordBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

