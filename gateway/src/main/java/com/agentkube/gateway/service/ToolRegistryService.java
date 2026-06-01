package com.agentkube.gateway.service;

import com.agentkube.gateway.model.ToolDefinition;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;

// In-memory catalog of all tools the platform can execute.
@Service
public class ToolRegistryService {

    private final Map<String, ToolDefinition> tools = new ConcurrentHashMap<>();

    // Adds a new tool to the master registry.
    public void registerTool(ToolDefinition tool) {
        tools.put(tool.name(), tool);
    }

    // Finds a tool by name, returning empty if it doesn't exist.
    public Optional<ToolDefinition> getTool(String name) {
        return Optional.ofNullable(tools.get(name));
    }

    // Returns all tools currently in the registry.
    public List<ToolDefinition> getAllTools() {
        return new ArrayList<>(tools.values());
    }
}
