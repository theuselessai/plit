# EdgeUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_node_id** | Option<**String**> |  | [optional]
**target_node_id** | Option<**String**> |  | [optional]
**edge_type** | Option<**EdgeType**> |  (enum: direct, conditional) | [optional]
**edge_label** | Option<**EdgeLabel**> |  (enum: , llm, tool, output_parser, loop_body, loop_return, skill) | [optional]
**condition_mapping** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**condition_value** | Option<**String**> |  | [optional]
**priority** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


