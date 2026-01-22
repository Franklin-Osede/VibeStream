# Project Tracker MCP Server

This MCP server manages the project state, enforcing a strict **Backend First -> Frontend Second** workflow and providing intelligent assistance for frontend development.

## Installation

Add this to your MCP settings file (e.g., `~/Library/Application Support/Cursor/User/globalStorage/cursor.mcp/mcp.json` or VSCode equivalent):

```json
{
  "mcpServers": {
    "project-tracker": {
      "command": "node",
      "args": [
        "/Users/domoblock/Documents/Proycts-dev/Vibestream/tools/project-tracker/build/index.js"
      ],
      "env": {}
    }
  }
}
```

## Tools

### Backend Phase

- **`add_backend_task(description)`**: Create a new backend task.
- **`log_endpoint(taskId, method, path, ...)`**: completion of a task by logging the API endpoint it created.
- **`update_phase("FRONTEND")`**: Switch to frontend phase (requires all backend tasks to be complete).

### Frontend Phase

- **`register_screen(name, description)`**: define a screen you are building.
- **`get_suggested_endpoints(screenDescription)`**: Ask the server which backend endpoints are relevant for this screen.
- **`link_screen_to_endpoint(screenId, endpointId)`**: Record the dependency.
- **`update_screen_status(screenId, status)`**: Track progress.

## Workflow

1.  **Backend**: create tasks -> implement -> `log_endpoint`.
2.  **Switch**: `update_phase("FRONTEND")`.
3.  **Frontend**: `register_screen` -> `get_suggested_endpoints` -> build & connect -> `update_screen_status`.
