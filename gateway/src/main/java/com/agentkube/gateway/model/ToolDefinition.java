package com.agentkube.gateway.model;

import tools.jackson.databind.JsonNode;

// Data structure that tells the LLM how to use a tool and what data it needs.
public record ToolDefinition(
    String name,
    String description,
    JsonNode inputSchema
) {}
