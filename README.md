Nama: Arya Raditya Kusuma
NPM: 2306215816

# Reflection
> 1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?

The key differences between RPC methods lie in their communication patterns. Unary RPC follows a simple request-response model suitable for one-time operations like payment processing. Server streaming allows the server to push multiple responses to a single request, ideal for delivering transaction histories or real-time feeds. Bidirectional streaming enables ongoing two-way communication, perfect for chat systems or collaborative tools. Unary works best for stateless operations, server streaming excels at data delivery, and bidirectional shines in interactive scenarios where both sides need to send continuous updates.

> 2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?

Implementing secure gRPC services in Rust requires multiple considerations. Transport security should use TLS encryption to protect data in transit. Authentication can be implemented through JWT tokens or mTLS certificates, while authorization should validate permissions per RPC call. Input validation is crucial to prevent injection attacks, and Rust's ownership model helps prevent memory safety issues. However, developers must still implement proper rate limiting and audit logging, as type safety alone doesn't prevent logical vulnerabilities or DDoS attacks.

> 3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?

Bidirectional streaming in Rust gRPC presents several challenges. The asynchronous nature requires careful handling of backpressure when clients can't keep up with message flow. Connection management becomes complex, needing proper reconnection logic for dropped streams. Resource usage grows with each active connection, requiring efficient memory management. Rust's borrow checker adds complexity when sharing state between streams, though it prevents data races. These factors make bidirectional streaming powerful but demanding to implement correctly compared to simpler unary calls.

> 4. What are the advantages and disadvantages of using the `tokio_stream::wrappers::ReceiverStream` for streaming responses in Rust gRPC services?

Using ReceiverStream in gRPC services offers clear tradeoffs. Its main advantage is seamless integration with Tokio's channel system, automatically converting MPSC channels into compliant streams. It handles backpressure naturally through channel capacity limits and provides clean stream termination. However, it adds abstraction layers that can obscure debugging, and the fixed channel size may not suit all use cases. While excellent for basic streaming needs, complex scenarios might require implementing custom streams for finer control over behavior and performance characteristics.

> 5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?

Structuring Rust gRPC code for maintainability involves several best practices. Services should be separated into distinct modules with clear interfaces. Shared protocol buffer types should be centralized to ensure consistency. Middleware like interceptors can handle cross-cutting concerns like logging uniformly. The business logic should remain decoupled from transport details, enabling easier testing and future protocol changes. This modular approach makes the codebase more navigable and simplifies updates to individual components without widespread changes.

> 6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?

Enhancing the payment service requires several realistic improvements. The implementation should validate payment amounts and user IDs against business rules before processing. Adding idempotency keys would prevent duplicate processing of retried requests. Transaction status should be properly logged for auditing purposes. The service should integrate with a payment gateway with appropriate timeout and retry logic. These additions would make the basic example production-ready while maintaining the clean service interface.

> 7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?

Adopting gRPC significantly influences system architecture. Its strong typing and code generation promote well-defined service contracts between components. The HTTP/2 foundation enables efficient communication but requires infrastructure support. While excellent for internal microservices, gRPC's binary format complicates browser integration compared to REST. The protocol encourages different thinking about API design, emphasizing streaming capabilities and making it particularly suitable for systems requiring high-performance inter-service communication.

> 8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

HTTP/2 provides substantial advantages over HTTP/1.1 for API communication. Its multiplexing allows multiple concurrent streams over one connection, reducing latency. Header compression shrinks message sizes significantly. However, HTTP/2's binary format makes manual debugging harder than HTTP/1.1's text-based format. While WebSocket offers simpler real-time communication, gRPC over HTTP/2 provides better built-in flow control and more efficient bidirectional streaming, making it superior for structured data exchange.

> 9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?

REST and gRPC streaming represent fundamentally different communication models. REST's request-response pattern is simple but requires clients to poll for updates, adding latency. gRPC streaming maintains persistent connections that push data immediately when available. While REST works well for occasional data fetches, gRPC streaming provides lower latency for real-time updates. However, streaming demands more sophisticated client implementations and connection management compared to REST's stateless nature.

> 10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?

Protocol Buffers and JSON represent different serialization philosophies. Protobuf's schema-driven approach ensures type safety and enables efficient binary encoding, reducing payload sizes by 60-80% compared to JSON. However, it requires upfront schema definition and compilation. JSON offers human-readability and flexibility for rapid iteration but lacks built-in validation and generates larger payloads. Protobuf is ideal for performance-sensitive internal APIs, while JSON may be preferable for public-facing APIs needing quick iteration or browser compatibility.