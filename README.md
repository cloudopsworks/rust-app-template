# rust-app-template

Rust Lang Application Template with Github Action Gitops

## Makefile Targets Usage

### code/init Target

The `code/init` target initializes your Go application with the following actions:

- Installs required packages (gitversion, gh, yq)
- Removes the existing go.mod file
- Initializes a new Go module with the current project name
- Runs `go mod tidy` to ensure dependencies are properly managed
- Replaces all instances of "hello-service" with your project name in all Go files

Usage:

```bash
make code/init
```

### version Target

The `version` target creates a VERSION file for your application using GitVersion:

- If the current commit is a Git tag, it extracts the version from the tag
- Otherwise, it uses GitVersion to generate a semantic version
- Replaces '+' with '-' in the version string for compatibility with Docker and Helm

Usage:

```bash
make version
```



## To use this API service:

1. Make sure all files are in their correct locations according to the directory structure shown above.

2. Add the missing dependency to `Cargo.toml` by running:
```bash
cargo add chrono
cargo add tracing
```

3. Build and run the project:
```bash
cargo build
cargo run
```

The API will be available at `http://localhost:8080` with the following endpoints:
- `GET /api/v1/health` - Returns service health status
- `GET /api/v1/hello` - Returns hello world message
- `GET /api/v1/hello?name=YourName` - Returns personalized hello message

The implementation includes several production-ready features:
- Request logging
- Response compression
- Path normalization
- API versioning
- Comprehensive test coverage
- Error handling
- Request tracing
- Type-safe responses
- Documentation

