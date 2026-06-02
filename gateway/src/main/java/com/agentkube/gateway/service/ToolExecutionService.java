package com.agentkube.gateway.service;

import com.agentkube.gateway.model.ToolDefinition;
import com.agentkube.gateway.model.ToolExecutionResult;
import com.agentkube.gateway.validation.SchemaValidator;
import tools.jackson.databind.JsonNode;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Set;

// The main controller that finds, checks, and runs tools for our agents.
@Service
@RequiredArgsConstructor
public class ToolExecutionService {

    private final ToolRegistryService registry;
    private final SchemaValidator validator;

    /**
     * Executes a tool after validating the input against its schema.
     */
    public ToolExecutionResult execute(String toolName, JsonNode arguments) {
        // 1. Look up the tool in the registry
        return registry.getTool(toolName)
            .map(tool -> validateAndRun(tool, arguments))
            .orElseGet(() -> ToolExecutionResult.failure(List.of("Tool not found: " + toolName)));
    }

    private ToolExecutionResult validateAndRun(ToolDefinition tool, JsonNode arguments) {
        // 2. Validate the input data against the tool's schema
        Set<String> errors = validator.validate(arguments, tool.inputSchema());
        
        if (!errors.isEmpty()) {
            return ToolExecutionResult.failure(List.copyOf(errors));
        }

        // 3. TODO: In Phase 2, we will route this to actual tool implementations.
        // For now, we return a mock success message to prove the plumbing works.
        return ToolExecutionResult.success("Mock output for tool: " + tool.name());
    }
}
