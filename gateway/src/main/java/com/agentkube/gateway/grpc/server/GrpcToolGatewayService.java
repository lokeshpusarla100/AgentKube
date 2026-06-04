package com.agentkube.gateway.grpc.server;

import com.agentkube.gateway.grpc.ToolExecutionRequest;
import com.agentkube.gateway.grpc.ToolExecutionResult;
import com.agentkube.gateway.grpc.ToolGatewayServiceGrpc;
import com.agentkube.gateway.service.ToolExecutionService;
import io.grpc.stub.StreamObserver;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import net.devh.boot.grpc.server.service.GrpcService;
import tools.jackson.core.JacksonException;
import tools.jackson.databind.JsonNode;
import tools.jackson.databind.ObjectMapper;

import java.util.List;

/**
 * gRPC server implementation that exposes tool execution to the Rust engine.
 * This class acts as a bridge between the gRPC protocol and our internal ToolExecutionService.
 */
@Slf4j
@GrpcService
@RequiredArgsConstructor
public class GrpcToolGatewayService extends ToolGatewayServiceGrpc.ToolGatewayServiceImplBase {

    private final ToolExecutionService executionService;
    private final ObjectMapper objectMapper = new ObjectMapper();

    @Override
    public void executeTool(ToolExecutionRequest request, StreamObserver<ToolExecutionResult> responseObserver) {
        log.info("Received tool execution request for agent: {}, tool: {}", 
                request.getAgentId(), request.getToolName());

        try {
            // 1. Parse the input JSON string from gRPC into a Jackson JsonNode.
            JsonNode arguments = objectMapper.readTree(request.getInputJson());

            // 2. Delegate to the internal service for validation and execution.
            com.agentkube.gateway.model.ToolExecutionResult internalResult = 
                    executionService.execute(request.getToolName(), arguments);

            // 3. Map our internal result back to the gRPC protobuf format.
            ToolExecutionResult protoResult = ToolExecutionResult.newBuilder()
                    .setSuccess(internalResult.success())
                    .setOutput(internalResult.output() != null ? internalResult.output() : "")
                    .addAllErrors(internalResult.errors() != null ? internalResult.errors() : List.of())
                    .build();

            // 4. Send the response back to the Rust client.
            responseObserver.onNext(protoResult);
            responseObserver.onCompleted();

        } catch (JacksonException e) {
            log.error("Failed to parse tool input JSON", e);
            
            // If the JSON is invalid, we return a failed proto result.
            ToolExecutionResult errorResult = ToolExecutionResult.newBuilder()
                    .setSuccess(false)
                    .addAllErrors(List.of("Invalid JSON input: " + e.getMessage()))
                    .build();
            
            responseObserver.onNext(errorResult);
            responseObserver.onCompleted();
        } catch (Exception e) {
            log.error("Unexpected error during tool execution", e);
            responseObserver.onError(e);
        }
    }
}
