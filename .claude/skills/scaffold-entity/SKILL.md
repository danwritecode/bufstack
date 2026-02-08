---
name: scaffold-entity
description: Scaffold a new gRPC entity from an existing migration file. Generates protobuf, Rust models, repositories, and gRPC service implementations. Use this when you need to create CRUD operations for a new database entity.
---

# Scaffold Entity

You are helping scaffold a new entity in the bufstack backend project. This skill reads an existing migration file and generates all necessary code: protobuf definition, Rust models, repositories, gRPC service implementations, and auto-registers the service.

## Project Structure Context

- **Protos**: `protos/`
- **Backend API**: `backend/api/`
- **Backend Data**: `backend/data/`
- **Migrations**: `backend/data/migrations/`
- **Build Config**: `backend/api/build.rs`
- **gRPC Server**: `backend/api/src/grpc.rs`

## Technology Stack

- **gRPC Framework**: Tonic (v0.14.2) with tonic-prost-build
- **Database**: SQLite with SQLx
- **Protobuf**: proto3 syntax
- **Auth**: Clerk-based authentication with user_id context

## Workflow Overview

The user has already created a migration file (e.g., `20260119212811_entity_name.up.sql`). Your job is to:
1. Read and parse the migration file to understand the schema
2. Generate protobuf definition
3. Update build.rs to compile the new proto
4. Generate Rust model
5. Generate repository implementation
6. Generate gRPC service implementation
7. Auto-register the service in grpc.rs

## Step 1: Identify and Read Migration File

Ask the user which migration file to use, or if they want to use the most recent one. The migration files are in `backend/data/migrations/`.

Use the Read tool to read the migration file and parse the CREATE TABLE statement.

**Expected Migration Format:**
```sql
CREATE TABLE entity_name (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    field_name TYPE [NOT NULL],
    ...
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Parse the following information:**
- Table name (entity name)
- Field names and types
- Which fields are required (NOT NULL)
- Skip parsing: id, user_id, created_at, updated_at (these are standard)

## Step 2: Generate Protobuf Definition

Create a proto file at `protos/{entity_name_lowercase}.proto`

**Template:**
```protobuf
syntax = "proto3";
package {entity_name_lowercase};

// Service definition with standard CRUD operations
// IMPORTANT: Service name MUST be {EntityName}Service to avoid naming conflicts with the message type
service {EntityName}Service {
    rpc Create{EntityName} (Create{EntityName}Request) returns (Create{EntityName}Response);
    rpc Get{EntityName} (Get{EntityName}Request) returns (Get{EntityName}Response);
    rpc List{EntityName}s (List{EntityName}sRequest) returns (List{EntityName}sResponse);
    rpc Update{EntityName} (Update{EntityName}Request) returns (Update{EntityName}Response);
    rpc Delete{EntityName} (Delete{EntityName}Request) returns (Delete{EntityName}Response);
}

// Main entity message
message {EntityName} {
    int32 id = 1;
    string user_id = 2;
    {/* Generate fields from migration starting at field number 3 */}
    string created_at = {/* last_field_num + 1 */};
    string updated_at = {/* last_field_num + 2 */};
}

// Request/Response messages
message Create{EntityName}Request {
    {/* Fields except id, user_id, created_at, updated_at */}
}

message Create{EntityName}Response {
    {EntityName} {entity_name_lowercase} = 1;
}

message Get{EntityName}Request {
    int32 id = 1;
}

message Get{EntityName}Response {
    {EntityName} {entity_name_lowercase} = 1;
}

message List{EntityName}sRequest {
    int32 page = 1;
    int32 page_size = 2;
}

message List{EntityName}sResponse {
    repeated {EntityName} {entity_name_lowercase}s = 1;
    int32 total = 2;
}

message Update{EntityName}Request {
    int32 id = 1;
    {/* Updatable fields - all fields except id, user_id, created_at, updated_at */}
}

message Update{EntityName}Response {
    {EntityName} {entity_name_lowercase} = 1;
}

message Delete{EntityName}Request {
    int32 id = 1;
}

message Delete{EntityName}Response {
    bool success = 1;
}
```

**SQLite to Protobuf Type Mapping:**
- `TEXT` → `string`
- `INTEGER` → `int32` (or `int64` if explicitly large values)
- `REAL` → `double`
- `BLOB` → `bytes`

**Optional Fields:**
- If SQL field allows NULL (no NOT NULL), use `optional` in proto3 or just omit the field in requests

## Step 3: Update Build Configuration

Update `backend/api/build.rs` to include the new proto file.

Read the existing build.rs, then add a new line:
```rust
tonic_prost_build::compile_protos("../../protos/{entity_name_lowercase}.proto")?;
```

Use the Edit tool to add this line before the `Ok(())`.

## Step 4: Generate Rust Model

Create `backend/data/src/models/{entity_name_lowercase}.rs`

**Template:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct {EntityName} {
    pub id: i32,
    pub user_id: String,
    {/* Generate fields from migration with Rust types */}
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create{EntityName}Input {
    {/* Fields except id, user_id, created_at, updated_at */}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update{EntityName}Input {
    {/* Updatable fields - same as Create but all Option<T> for partial updates */}
}
```

**SQLite to Rust Type Mapping:**
- `TEXT NOT NULL` → `String`
- `TEXT` (nullable) → `Option<String>`
- `INTEGER NOT NULL` → `i32`
- `INTEGER` (nullable) → `Option<i32>`
- `REAL NOT NULL` → `f64`
- `REAL` (nullable) → `Option<f64>`
- `BLOB` → `Vec<u8>` or `Option<Vec<u8>>`

Update `backend/data/src/models/mod.rs` to add:
```rust
pub mod {entity_name_lowercase};
pub use {entity_name_lowercase}::*;
```

## Step 5: Generate Repository Implementation

Create `backend/data/src/repositories/{entity_name_lowercase}_repository.rs`

**Template:**
```rust
use crate::models::{EntityName, Create{EntityName}Input, Update{EntityName}Input};
use crate::repositories::Repository;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct {EntityName}Repository {
    pool: Pool<Sqlite>,
}

impl {EntityName}Repository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Create a new {entity_name_lowercase}
    pub async fn create(
        &self,
        user_id: &str,
        input: Create{EntityName}Input,
    ) -> Result<{EntityName}> {
        let {entity_name_lowercase} = sqlx::query_as::<_, {EntityName}>(
            r#"
            INSERT INTO {entity_name_lowercase} (user_id, {/* comma-separated field names */})
            VALUES (?, {/* comma-separated ? placeholders */})
            RETURNING *
            "#,
        )
        .bind(user_id)
        {/* .bind(input.field_name) for each field */}
        .fetch_one(&self.pool)
        .await?;

        Ok({entity_name_lowercase})
    }

    /// Get a {entity_name_lowercase} by ID
    pub async fn get(&self, id: i32, user_id: &str) -> Result<Option<{EntityName}>> {
        let {entity_name_lowercase} = sqlx::query_as::<_, {EntityName}>(
            "SELECT * FROM {entity_name_lowercase} WHERE id = ? AND user_id = ?"
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok({entity_name_lowercase})
    }

    /// List all {entity_name_lowercase}s for a user
    pub async fn list(
        &self,
        user_id: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<{EntityName}>> {
        let offset = (page - 1) * page_size;
        let {entity_name_lowercase}s = sqlx::query_as::<_, {EntityName}>(
            r#"
            SELECT * FROM {entity_name_lowercase}
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(user_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok({entity_name_lowercase}s)
    }

    /// Count total {entity_name_lowercase}s for a user
    pub async fn count(&self, user_id: &str) -> Result<i32> {
        let count: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM {entity_name_lowercase} WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    /// Update a {entity_name_lowercase}
    pub async fn update(
        &self,
        id: i32,
        user_id: &str,
        input: Update{EntityName}Input,
    ) -> Result<Option<{EntityName}>> {
        // Build dynamic UPDATE query based on which fields are Some()
        let mut updates = Vec::new();
        let mut bindings = Vec::new();

        {/* For each field in Update input:
        if let Some(value) = input.field_name {
            updates.push("field_name = ?");
            bindings.push(value);
        }
        */}

        if updates.is_empty() {
            return self.get(id, user_id).await;
        }

        let query = format!(
            "UPDATE {entity_name_lowercase} SET {} WHERE id = ? AND user_id = ? RETURNING *",
            updates.join(", ")
        );

        let mut q = sqlx::query_as::<_, {EntityName}>(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        let {entity_name_lowercase} = q
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok({entity_name_lowercase})
    }

    /// Delete a {entity_name_lowercase}
    pub async fn delete(&self, id: i32, user_id: &str) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM {entity_name_lowercase} WHERE id = ? AND user_id = ?"
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

impl Repository for {EntityName}Repository {
    fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
```

Update `backend/data/src/repositories/mod.rs` to add:
```rust
pub mod {entity_name_lowercase}_repository;
pub use {entity_name_lowercase}_repository::*;
```

## Step 6: Generate gRPC Service Implementation

Create `backend/api/src/services/{entity_name_lowercase}_service.rs`

**Template:**
```rust
use tonic::{Request, Response, Status};
use data::repositories::{EntityName}Repository;
use data::models::{Create{EntityName}Input, Update{EntityName}Input};

pub mod {entity_name_lowercase} {
    tonic::include_proto!("{entity_name_lowercase}");
}

use {entity_name_lowercase}::*;

pub struct {EntityName}Service {
    repository: {EntityName}Repository,
}

impl {EntityName}Service {
    pub fn new(repository: {EntityName}Repository) -> Self {
        Self { repository }
    }

    /// Extract user_id from request headers (set by auth middleware)
    fn get_user_id<T>(request: &Request<T>) -> Result<String, Status> {
        request
            .metadata()
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| Status::unauthenticated("User not authenticated"))
    }
}

#[tonic::async_trait]
impl {entity_name_lowercase}::{entity_name_lowercase}_service_server::{EntityName}Service for {EntityName}Service {
    async fn create_{entity_name_lowercase}(
        &self,
        request: Request<Create{EntityName}Request>,
    ) -> Result<Response<Create{EntityName}Response>, Status> {
        let user_id = Self::get_user_id(&request)?;
        let req = request.into_inner();

        let input = Create{EntityName}Input {
            {/* Map proto request fields to input struct */}
        };

        let {entity_name_lowercase} = self
            .repository
            .create(&user_id, input)
            .await
            .map_err(|e| Status::internal(format!("Failed to create {entity_name_lowercase}: {}", e)))?;

        let response = Create{EntityName}Response {
            {entity_name_lowercase}: Some({EntityName} {
                id: {entity_name_lowercase}.id,
                user_id: {entity_name_lowercase}.user_id,
                {/* Map model fields to proto response */}
                created_at: {entity_name_lowercase}.created_at,
                updated_at: {entity_name_lowercase}.updated_at,
            }),
        };

        Ok(Response::new(response))
    }

    async fn get_{entity_name_lowercase}(
        &self,
        request: Request<Get{EntityName}Request>,
    ) -> Result<Response<Get{EntityName}Response>, Status> {
        let user_id = Self::get_user_id(&request)?;
        let req = request.into_inner();

        let {entity_name_lowercase} = self
            .repository
            .get(req.id, &user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get {entity_name_lowercase}: {}", e)))?
            .ok_or_else(|| Status::not_found("{EntityName} not found"))?;

        let response = Get{EntityName}Response {
            {entity_name_lowercase}: Some({EntityName} {
                id: {entity_name_lowercase}.id,
                user_id: {entity_name_lowercase}.user_id,
                {/* Map fields */}
                created_at: {entity_name_lowercase}.created_at,
                updated_at: {entity_name_lowercase}.updated_at,
            }),
        };

        Ok(Response::new(response))
    }

    async fn list_{entity_name_lowercase}s(
        &self,
        request: Request<List{EntityName}sRequest>,
    ) -> Result<Response<List{EntityName}sResponse>, Status> {
        let user_id = Self::get_user_id(&request)?;
        let req = request.into_inner();

        let page = if req.page <= 0 { 1 } else { req.page };
        let page_size = if req.page_size <= 0 { 10 } else { req.page_size };

        let {entity_name_lowercase}s = self
            .repository
            .list(&user_id, page, page_size)
            .await
            .map_err(|e| Status::internal(format!("Failed to list {entity_name_lowercase}s: {}", e)))?;

        let total = self
            .repository
            .count(&user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to count {entity_name_lowercase}s: {}", e)))?;

        let response = List{EntityName}sResponse {
            {entity_name_lowercase}s: {entity_name_lowercase}s
                .into_iter()
                .map(|item| {EntityName} {
                    id: item.id,
                    user_id: item.user_id,
                    {/* Map fields */}
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                })
                .collect(),
            total,
        };

        Ok(Response::new(response))
    }

    async fn update_{entity_name_lowercase}(
        &self,
        request: Request<Update{EntityName}Request>,
    ) -> Result<Response<Update{EntityName}Response>, Status> {
        let user_id = Self::get_user_id(&request)?;
        let req = request.into_inner();

        let input = Update{EntityName}Input {
            {/* Map proto fields to input struct - convert to Option<T> */}
        };

        let {entity_name_lowercase} = self
            .repository
            .update(req.id, &user_id, input)
            .await
            .map_err(|e| Status::internal(format!("Failed to update {entity_name_lowercase}: {}", e)))?
            .ok_or_else(|| Status::not_found("{EntityName} not found"))?;

        let response = Update{EntityName}Response {
            {entity_name_lowercase}: Some({EntityName} {
                id: {entity_name_lowercase}.id,
                user_id: {entity_name_lowercase}.user_id,
                {/* Map fields */}
                created_at: {entity_name_lowercase}.created_at,
                updated_at: {entity_name_lowercase}.updated_at,
            }),
        };

        Ok(Response::new(response))
    }

    async fn delete_{entity_name_lowercase}(
        &self,
        request: Request<Delete{EntityName}Request>,
    ) -> Result<Response<Delete{EntityName}Response>, Status> {
        let user_id = Self::get_user_id(&request)?;
        let req = request.into_inner();

        let success = self
            .repository
            .delete(req.id, &user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to delete {entity_name_lowercase}: {}", e)))?;

        let response = Delete{EntityName}Response { success };

        Ok(Response::new(response))
    }
}
```

If services directory doesn't exist, create it. Then update `backend/api/src/services/mod.rs` with:
```rust
pub mod {entity_name_lowercase}_service;
pub use {entity_name_lowercase}_service::*;
```

## Step 7: Auto-Register Service in grpc.rs

Read `backend/api/src/grpc.rs` and update it to:

1. **Add import** at the top:
   ```rust
   use crate::services::{EntityName}Service;
   use crate::services::{entity_name_lowercase}::{entity_name_lowercase}_service_server::{EntityName}ServiceServer;
   ```

2. **Initialize repository and service** in the main function (after database pool is created):
   ```rust
   let {entity_name_lowercase}_repository = data::repositories::{EntityName}Repository::new(pool.clone());
   let {entity_name_lowercase}_service = {EntityName}Service::new({entity_name_lowercase}_repository);
   ```

3. **Add service to server builder** (chain with `.add_service()`):
   ```rust
   .add_service({EntityName}ServiceServer::new({entity_name_lowercase}_service))
   ```

Use the Edit tool to make these changes. Look for the pattern of how other services are registered and follow the same pattern.

## Step 8: Update Frontend Testing Page

Update `frontend/app/pages/_testing.vue` to add CRUD UI for the new entity.

**Note:** This file is prefixed with `_` so Nuxt ignores it by default. It's conditionally included only in non-production environments via the `pages:extend` hook in nuxt.config.ts.

### 8.1 Add Import

In the `<script setup>` section, add the service import:
```typescript
import { {EntityName}Service } from "../gen/{entity_name_lowercase}_pb";
```

### 8.2 Add Client

After the existing client definitions, add:
```typescript
const {entity_name_lowercase}Client = createClient({EntityName}Service, transport);
```

### 8.3 Add State

Add state variables for the new entity:
```typescript
// {EntityName} state
const new{EntityName} = ref({
  {/* fields from migration with default values - strings: "", numbers: 0 */}
});
const {entity_name_lowercase}List = ref<any[]>([]);
```

### 8.4 Add CRUD Functions

Add the CRUD operations:
```typescript
// {EntityName} CRUD operations
async function create{EntityName}() {
  if (!new{EntityName}.value.{/* required_field */}) {
    showStatus("Please fill required fields", "error");
    return;
  }

  try {
    await {entity_name_lowercase}Client.create{EntityName}({
      {/* map fields from state */}
    });

    new{EntityName}.value = {
      {/* reset to defaults */}
    };

    showStatus("{EntityName} created successfully!", "success");
    await load{EntityName}s();
  } catch (error) {
    showStatus(`Error creating {entity_name_lowercase}: ${error}`, "error");
    console.error(error);
  }
}

async function load{EntityName}s() {
  try {
    const res = await {entity_name_lowercase}Client.list{EntityName}s({ page: 1, pageSize: 100 });
    {entity_name_lowercase}List.value = res.{entity_name_lowercase}s;
  } catch (error) {
    showStatus(`Error loading {entity_name_lowercase}s: ${error}`, "error");
    console.error(error);
  }
}

async function delete{EntityName}(id: number) {
  if (!confirm("Are you sure you want to delete this {entity_name_lowercase}?")) return;

  try {
    await {entity_name_lowercase}Client.delete{EntityName}({ id });
    showStatus("{EntityName} deleted successfully!", "success");
    await load{EntityName}s();
  } catch (error) {
    showStatus(`Error deleting {entity_name_lowercase}: ${error}`, "error");
    console.error(error);
  }
}
```

### 8.5 Update onMounted

Add the load function to onMounted:
```typescript
onMounted(() => {
  // ... existing loads
  load{EntityName}s();
});
```

### 8.6 Add Template Section

Add a new section in the template (before the Status Messages section):
```vue
<!-- {EntityName} Section -->
<div class="border border-gray-300 rounded-lg p-6 space-y-4">
  <h2 class="text-2xl font-semibold">{EntityName} Management</h2>

  <!-- Create {EntityName} Form -->
  <div class="bg-gray-50 p-4 rounded">
    <h3 class="font-medium mb-2">Create New {EntityName}</h3>
    <div class="space-y-2">
      {/* For each field, add appropriate input:
          - TEXT fields: <input type="text" v-model="new{EntityName}.fieldName" placeholder="Field name" class="w-full px-3 py-2 border border-gray-300 rounded" />
          - TEXT (long): <textarea v-model="new{EntityName}.fieldName" placeholder="Field name" class="w-full px-3 py-2 border border-gray-300 rounded" rows="2" />
          - INTEGER: <input type="number" v-model="new{EntityName}.fieldName" placeholder="Field name" class="w-full px-3 py-2 border border-gray-300 rounded" />
          - Foreign key: <select v-model="new{EntityName}.parentId" class="w-full px-3 py-2 border border-gray-300 rounded">...</select>
          - Enum: <select v-model="new{EntityName}.enumField" class="w-full px-3 py-2 border border-gray-300 rounded"><option :value="0">Option1</option>...</select>
      */}
      <button
        @click="create{EntityName}"
        class="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
      >
        Create {EntityName}
      </button>
    </div>
  </div>

  <!-- {EntityName} List -->
  <div>
    <div class="flex justify-between items-center mb-2">
      <h3 class="font-medium">{EntityName} List</h3>
      <button
        @click="load{EntityName}s"
        class="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
      >
        Refresh
      </button>
    </div>

    <div v-if="{entity_name_lowercase}List.length === 0" class="text-gray-500 italic">
      No {entity_name_lowercase}s yet. Create one above!
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="item in {entity_name_lowercase}List"
        :key="item.id"
        class="p-4 bg-white border border-gray-200 rounded"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <h4 class="font-medium">{{ item.{/* primary display field */} }}</h4>
            <div class="text-xs text-gray-500 mt-2 space-y-1">
              {/* Display each field: <div>FieldName: {{ item.fieldName }}</div> */}
            </div>
          </div>

          <div class="flex gap-2">
            <button
              @click="delete{EntityName}(item.id)"
              class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
```

### 8.7 Add Helper Functions (if needed)

If the entity has enum fields, add a format helper:
```typescript
function format{FieldName}(value: number): string {
  const options = ['Option1', 'Option2', ...];
  return options[value] || 'Unknown';
}
```

## Step 9: Validate Backend Code

IMPORTANT: After generating backend files, run `cargo check` to validate the code:

```bash
cd backend/api
cargo check
```

Cargo check will:
- Compile all generated code
- Identify any errors or type mismatches
- Tell you exactly what needs to be fixed

If there are any errors:
1. Read the error messages carefully
2. Fix each error one by one (common issues: missing imports, type mismatches, incorrect trait implementations)
3. Re-run `cargo check` after each fix until all errors are resolved

## Step 10: Provide Integration Summary

After all cargo check errors are resolved, provide a summary with:

1. **Files created:**
   - Proto file path
   - Model file path
   - Repository file path
   - Service file path

2. **Files updated:**
   - build.rs
   - models/mod.rs
   - repositories/mod.rs
   - services/mod.rs (if created)
   - grpc.rs
   - _testing.vue (frontend CRUD UI, dev-only)
   - Cargo.toml (if data dependency was added)

3. **Validation status:**
   - Cargo check passed
   - Summary of any errors that were fixed

## Important Parsing Notes

When parsing the migration file:

- **Skip standard fields**: id, user_id, created_at, updated_at
- **Extract field info**: name, SQL type, whether NOT NULL
- **Handle foreign keys**: If you see `FOREIGN KEY (field_id) REFERENCES other_table(id)`, note this is a relationship
- **Generate field numbers**: Start custom fields at 3 in proto (after id=1, user_id=2)
- **Preserve field order**: Keep the same order from the SQL table in proto and Rust structs

## Important Notes

- **User Isolation (CRITICAL SECURITY)**:
  - If the migration table has a `user_id` field (or similar field like `owner_id`, `account_id`), ALL repository queries MUST filter by this field for security
  - Every repository method (get, list, count, update, delete) MUST include `WHERE user_id = ?` (or equivalent) in the SQL query
  - The `user_id` comes from the auth context (extracted in the service layer via `get_user_id()`)
  - Never allow users to access data from other users
  - If a table does NOT have a user_id field, it may be a global/system table and doesn't need user isolation
- **Auth Context**: The `user_id` is injected by the AuthInterceptor middleware and available in request metadata
- **No Down Migrations**: Only .up.sql files are used
- **Auto-Registration**: Always update grpc.rs to register the new service
- **Error Handling**: Use proper Status codes (not_found, internal, unauthenticated)
- **Pagination**: List endpoints support pagination with page and page_size

## Execution Steps

1. Ask user which migration file to scaffold from (or use most recent)
2. Read and parse the migration file
3. Generate all files (proto, model, repository, service)
4. Update all module files (mod.rs files)
5. Update build.rs
6. Update grpc.rs to register the service
7. Update testing.vue with CRUD UI for the new entity
8. Run `cargo check` to validate all generated code
9. Fix any compilation errors identified by cargo check
10. Provide integration summary with validation results
