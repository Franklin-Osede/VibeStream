
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { z } from "zod";
import { Store } from "./store.js";
import { BackendTask, EndpointDef, Screen } from "./types.js";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT_DIR = path.join(__dirname, ".."); // Adjusted for build/ structure

const store = new Store(ROOT_DIR);

const server = new Server(
  {
    name: "project-tracker",
    version: "0.1.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Helper to generate IDs
const generateId = () => Math.random().toString(36).substring(2, 15);

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "get_project_state",
        description: "Get the full current state of the project (backend/frontend progress).",
        inputSchema: {
          type: "object",
          properties: {},
        },
      },
      {
        name: "update_phase",
        description: "Switch project phase (BACKEND <-> FRONTEND). Enforces strict backend completion before switching to frontend.",
        inputSchema: {
          type: "object",
          properties: {
            phase: {
              type: "string",
              enum: ["BACKEND", "FRONTEND"],
              description: "The target phase to switch to."
            },
            force: {
              type: "boolean",
              description: "Force switch even if backend is incomplete (Not recommended)."
            }
          },
          required: ["phase"],
        },
      },
      {
        name: "add_backend_task",
        description: "Add a new task to the backend phase.",
        inputSchema: {
          type: "object",
          properties: {
            description: { type: "string" },
          },
          required: ["description"],
        },
      },
      {
        name: "complete_backend_task",
        description: "Mark a backend task as completed.",
        inputSchema: {
          type: "object",
          properties: {
            taskId: { type: "string" },
          },
          required: ["taskId"],
        },
      },
      {
        name: "log_endpoint",
        description: "Log technical details of an implemented endpoint. Links to a task.",
        inputSchema: {
          type: "object",
          properties: {
            taskId: { type: "string", description: "The task this endpoint resolves" },
            method: { type: "string", enum: ["GET", "POST", "PUT", "DELETE", "PATCH"] },
            path: { type: "string" },
            description: { type: "string" },
            parameters: { type: "array", items: { type: "string" } },
          },
          required: ["taskId", "method", "path", "description"],
        },
      },
      {
        name: "register_screen",
        description: "Register a new frontend screen to track.",
        inputSchema: {
          type: "object",
          properties: {
            name: { type: "string" },
            description: { type: "string" },
          },
          required: ["name", "description"],
        },
      },
      {
        name: "get_suggested_endpoints",
        description: "Get suggested backend endpoints for a screen based on its description.",
        inputSchema: {
          type: "object",
          properties: {
            screenDescription: { type: "string" },
          },
          required: ["screenDescription"],
        },
      },
      {
        name: "link_screen_to_endpoint",
        description: "Explicitly link a screen to a backend endpoint dependency.",
        inputSchema: {
          type: "object",
          properties: {
            screenId: { type: "string" },
            endpointId: { type: "string" },
          },
          required: ["screenId", "endpointId"],
        },
      },
      {
        name: "update_screen_status",
        description: "Update the status of a screen.",
        inputSchema: {
          type: "object",
          properties: {
            screenId: { type: "string" },
            status: { type: "string", enum: ["DESIGNED", "HTML_BUILT", "CONNECTED", "COMPLETED"] },
          },
          required: ["screenId", "status"],
        },
      },
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  await store.init(); // Ensure state is loaded
  const state = store.getState();

  switch (request.params.name) {
    case "get_project_state": {
      return {
        content: [{ type: "text", text: JSON.stringify(state, null, 2) }],
      };
    }

    case "update_phase": {
      const { phase, force } = request.params.arguments as any;
      
      if (phase === "FRONTEND" && !force && !state.backend.completed) {
         throw new Error("Cannot switch to FRONTEND phase: Backend is not marked as completed. Use 'force: true' if you are sure.");
      }

      await store.update((s) => {
        s.phase = phase;
      });

      return {
        content: [{ type: "text", text: `Phase switched to ${phase}` }],
      };
    }

    case "add_backend_task": {
      const { description } = request.params.arguments as any;
      const newTask: BackendTask = {
        id: generateId(),
        description,
        status: "PENDING",
      };
      await store.update((s) => {
        s.backend.tasks.push(newTask);
      });
      return {
        content: [{ type: "text", text: `Added backend task: ${newTask.id}` }],
      };
    }

    case "complete_backend_task": {
      const { taskId } = request.params.arguments as any;
      await store.update((s) => {
        const task = s.backend.tasks.find((t) => t.id === taskId);
        if (task) task.status = "COMPLETED";
      });
      return {
        content: [{ type: "text", text: `Marked task ${taskId} as COMPLETED` }],
      };
    }

    case "log_endpoint": {
      const { taskId, method, path, description, parameters } = request.params.arguments as any;
      const endpointDef: EndpointDef = {
        id: generateId(),
        method,
        path,
        description,
        parameters: parameters || [],
      };
      
      await store.update((s) => {
        const task = s.backend.tasks.find((t) => t.id === taskId);
        if (!task) throw new Error(`Task ${taskId} not found`);
        task.endpointDef = endpointDef;
        task.status = "COMPLETED"; // Auto-complete task when endpoint is logged
      });
      
      return {
        content: [{ type: "text", text: `Logged endpoint ${method} ${path} for task ${taskId}` }],
      };
    }

    case "register_screen": {
      const { name, description } = request.params.arguments as any;
      const newScreen: Screen = {
        id: generateId(),
        name,
        description,
        status: "DESIGNED",
        requiredEndpoints: [],
      };
      await store.update((s) => {
        s.frontend.screens.push(newScreen);
      });
      return {
        content: [{ type: "text", text: `Registered screen: ${newScreen.name} (${newScreen.id})` }],
      };
    }

    case "get_suggested_endpoints": {
      const { screenDescription } = request.params.arguments as any;
      const descLower = String(screenDescription).toLowerCase();
      
      // Intelligent matching: Find endpoints whose description/path match the screen description keywords
      // This is simple keyword matching for now, but efficient.
      const suggestions = state.backend.tasks
        .filter(t => t.endpointDef)
        .map(t => t.endpointDef!)
        .filter(ep => {
          const text = `${ep.method} ${ep.path} ${ep.description}`.toLowerCase();
          const keywords = descLower.split(" ").filter(w => w.length > 3);
          return keywords.some(k => text.includes(k));
        });

      return {
        content: [{ type: "text", text: JSON.stringify(suggestions, null, 2) }],
      };
    }

    case "link_screen_to_endpoint": {
      const { screenId, endpointId } = request.params.arguments as any;
      await store.update((s) => {
        const screen = s.frontend.screens.find(sc => sc.id === screenId);
        if (!screen) throw new Error("Screen not found");
        if (!screen.requiredEndpoints.includes(endpointId)) {
          screen.requiredEndpoints.push(endpointId);
        }
      });
      return {
        content: [{ type: "text", text: `Linked screen ${screenId} to endpoint ${endpointId}` }],
      };
    }
      
    case "update_screen_status": {
        const { screenId, status } = request.params.arguments as any;
        await store.update((s) => {
            const screen = s.frontend.screens.find(sc => sc.id === screenId);
            if (!screen) throw new Error("Screen not found");
             
            // Validation: Cannot be CONNECTED unless linked endpoints exist
            if (status === "CONNECTED" && screen.requiredEndpoints.length === 0) {
                 // Warning/Error? Letting it slide for now but could enforce
            }
            screen.status = status;
        });
        return {
             content: [{ type: "text", text: `Updated screen ${screenId} to ${status}` }],
        };
    }

    default:
      throw new Error("Unknown tool");
  }
});

async function runServer() {
  await store.init();
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("Project Tracker MCP Server running on stdio");
}

runServer().catch((error) => {
  console.error("Fatal error in main loop:", error);
  process.exit(1);
});
