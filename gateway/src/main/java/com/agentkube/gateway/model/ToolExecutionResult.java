package com.agentkube.gateway.model;

import java.util.List;

// The final response sent back to the agent after a tool attempt.
public record ToolExecutionResult(
    boolean success,
    String output,
    List<String> errors
) {
    // Factory method for a successful run.
    public static ToolExecutionResult success(String output) {
        return new ToolExecutionResult(true, output, List.of());
    }

    // Factory method for a failed validation or execution.
    public static ToolExecutionResult failure(List<String> errors) {
        return new ToolExecutionResult(false, null, errors);
    }
}
