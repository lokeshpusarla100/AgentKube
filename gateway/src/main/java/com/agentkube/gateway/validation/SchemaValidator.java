package com.agentkube.gateway.validation;

import tools.jackson.databind.JsonNode;
import com.networknt.schema.Schema;
import com.networknt.schema.SchemaRegistry;
import com.networknt.schema.SpecificationVersion;
import com.networknt.schema.OutputFormat;
import org.springframework.stereotype.Component;

import java.util.Set;
import java.util.stream.Collectors;

// A component that checks if the data sent by an agent matches the tool's requirements.
@Component
public class SchemaValidator {
    
    // Initialize the registry once for the Draft 7 specification.
    private final SchemaRegistry registry = SchemaRegistry.withDefaultDialect(SpecificationVersion.DRAFT_7);

    /**
     * Checks if the provided data matches the schema.
     * Returns a set of error messages, or an empty set if valid.
     */
    public Set<String> validate(JsonNode data, JsonNode schemaNode) {
        // 1. Get the compiled schema from the registry.
        Schema schema = registry.getSchema(schemaNode);
        
        // 2. Validate the data and format errors as "path: message".
        return schema.validate(data, OutputFormat.DEFAULT).stream()
                .map(error -> error.getInstanceLocation() + ": " + error.getMessage())
                .collect(Collectors.toSet());
    }
}
