package com.agentkube.gateway.service;

import com.agentkube.gateway.model.ToolDefinition;
import com.agentkube.gateway.model.ToolExecutionResult;
import com.agentkube.gateway.validation.SchemaValidator;
import tools.jackson.databind.ObjectMapper;
import tools.jackson.databind.node.ObjectNode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

// Verifies that the ToolExecutionService correctly orchestrates registry lookup and validation.
class ToolExecutionServiceTest {

    private ToolExecutionService executionService;
    private ToolRegistryService registry;
    private ObjectMapper mapper = new ObjectMapper();

    @BeforeEach
    void setUp() {
        registry = new ToolRegistryService();
        SchemaValidator validator = new SchemaValidator();
        executionService = new ToolExecutionService(registry, validator);

        // Register a test tool with a simple schema requiring a "query" string.
        ObjectNode schema = mapper.createObjectNode();
        schema.put("type", "object");
        ObjectNode properties = schema.putObject("properties");
        properties.putObject("query").put("type", "string");
        schema.putArray("required").add("query");

        registry.registerTool(new ToolDefinition("search", "Search the web", schema));
    }

    @Test
    void shouldExecuteSuccessfullyWhenInputIsValid() {
        ObjectNode args = mapper.createObjectNode();
        args.put("query", "What is Rust?");

        ToolExecutionResult result = executionService.execute("search", args);

        assertTrue(result.success());
        assertEquals("Mock output for tool: search", result.output());
        assertTrue(result.errors().isEmpty());
    }

    @Test
    void shouldFailWhenToolDoesNotExist() {
        ToolExecutionResult result = executionService.execute("unknown", mapper.createObjectNode());

        assertFalse(result.success());
        assertEquals(List.of("Tool not found: unknown"), result.errors());
    }

    @Test
    void shouldFailWhenInputIsInvalid() {
        ObjectNode args = mapper.createObjectNode();
        args.put("query", 123); // Should be a string

        ToolExecutionResult result = executionService.execute("search", args);

        assertFalse(result.success());
        assertFalse(result.errors().isEmpty());
        // Check that at least one error message contains information about the 'query' field.
        assertTrue(result.errors().stream().anyMatch(e -> e.contains("query")), 
            "Errors should contain 'query', but was: " + result.errors());
    }
}
