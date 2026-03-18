# EdgeIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_node_id** | **String** |  | 
**target_node_id** | Option<**String**> |  | [optional][default to ]
**edge_type** | Option<**EdgeType**> |  (enum: direct, conditional) | [optional][default to Direct]
**edge_label** | Option<**EdgeLabel**> |  (enum: , llm, tool, output_parser, loop_body, loop_return, skill) | [optional][default to Empty]
**condition_mapping** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**condition_value** | Option<**String**> |  | [optional][default to ]
**priority** | Option<**i32**> |  | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


