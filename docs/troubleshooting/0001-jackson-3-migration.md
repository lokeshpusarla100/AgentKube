# Troubleshooting Entry 001: Jackson 3 & JSON Schema Validator 3 Migration

## The Problem (What)
We encountered 13+ compilation errors when trying to run tests in the Java Gateway. 
1. `package com.fasterxml.jackson.databind does not exist`
2. `cannot find symbol: class JsonSchemaFactory`
3. `no suitable method found for validate(...)`

## The Cause (Why)
The project is running on **Spring Boot 4.0.6**, which is built on **Jackson 3**.
1. **Jackson Namespace Change**: Jackson 3 moved from `com.fasterxml.jackson` to `tools.jackson`. Old libraries and code expecting the old namespace fail.
2. **Library Rewrite**: `json-schema-validator` version 3.0.x is a ground-up rewrite for Jackson 3. It renamed core classes (`JsonSchema` -> `Schema`) and moved to a functional `OutputFormat` API.

## The Fix (How)
1. **Import Update**: Changed all Jackson imports in `ToolDefinition`, `SchemaValidator`, and `ToolExecutionService` to use `tools.jackson.databind.*`.
2. **API Alignment**: Updated `SchemaValidator` to use the new `SchemaRegistry` and `Schema` classes.
3. **Functional Validation**: Switched from `schema.validate(data)` to `schema.validate(data, OutputFormat.DEFAULT)` to support the new non-blocking API style.
4. **Error Formatting**: Improved error reporting by mapping the new `Error` objects using `error.getInstanceLocation()` for better agent feedback.

## Prevention
Always check if a major Spring Boot version (like 4.x) has migrated underlying core libraries (like Jackson) before adding standard dependencies.
