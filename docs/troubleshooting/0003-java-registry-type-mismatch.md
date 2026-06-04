# Troubleshooting Entry 003: Java Tool Registry Type Mismatch

## The Problem (What)
The Java Gateway failed to compile with the following error:
`incompatible types: java.lang.String cannot be converted to tools.jackson.databind.JsonNode`

## The Cause (Why)
In `ToolRegistryService.java`, we tried to register a tool by passing a raw JSON string as the `inputSchema`. However, the `ToolDefinition` record was designed to hold a `JsonNode` (the parsed AST) rather than a raw string. 

Because we moved to Jackson 3, the compiler is extremely strict about the difference between a `String` and a `JsonNode`.

## The Fix (How)
1.  Injected an `ObjectMapper` into the `ToolRegistryService`.
2.  Used `objectMapper.readTree(schemaJson)` inside a `@PostConstruct` method to parse the string into a real `JsonNode` before creating the `ToolDefinition`.
3.  Wrapped the logic in a try-catch block to handle potential `JsonProcessingException` during startup.

## Prevention
Always parse static configuration strings into their proper object representations during the "Setup/Initialization" phase of a service, rather than trying to use raw strings in domain models.
